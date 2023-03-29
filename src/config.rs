use std::io::Read;

#[derive(serde::Deserialize, Debug)]
struct Port {
    command: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct Service {
    command: Option<String>,
    ports: Option<std::collections::HashMap<String, Port>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Config<'a> {
    #[serde(skip)]
    tmpl_registry: handlebars::Handlebars<'a>,
    command: Option<String>,
    services: Option<std::collections::HashMap<String, Service>>,
}

impl<'a> std::default::Default for Config<'a> {
    fn default() -> Self {
        Self {
            tmpl_registry: handlebars::Handlebars::new(),
            command: None,
            services: None,
        }
    }
}

impl<'a> Config<'a> {
    pub fn from_toml(path: &str) -> Result<Self, crate::error::Error> {
        let mut config_content = String::new();

        std::fs::File::open(path)
            .and_then(|mut f| f.read_to_string(&mut config_content))
            .map_err(|e| crate::error::Error::Config(e.to_string()))?;

        toml::from_str(&config_content).map_err(|e| crate::error::Error::Config(e.to_string()))
    }

    pub fn render(&self, service_name: &str, internal_port: u16, exposed_port: u16) -> String {
        let tmpl = self
            .command_template(service_name, internal_port)
            .unwrap_or("open http://localhost:{{exposed_port}}");

        self.tmpl_registry
            .render_template(tmpl, &serde_json::json!({ "exposed_port": exposed_port }))
            .unwrap_or_else(|e| e.to_string())
    }

    fn command_template(&self, service_name: &str, internal_port: u16) -> Option<&str> {
        let port_name = internal_port.to_string();

        self.services
            .as_ref()
            .and_then(|services| {
                services.get(service_name).and_then(|service| {
                    service
                        .ports
                        .as_ref()
                        .and_then(|ports| ports.get(&port_name).and_then(|p| p.command.as_ref()))
                        .or(service.command.as_ref())
                })
            })
            .or(self.command.as_ref())
            .map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn config_from_toml() {
        let config =
            super::Config::from_toml("testdata/config.toml").expect("failed to parse config file");

        assert_eq!(
            "default command",
            config.render("unknown-service", 8080, 10001),
            "undefined service should rely on default command"
        );

        assert_eq!(
            "command for foobar:81 => 10002",
            config.render("foobar", 81, 10002),
            "defined service/port port should use specific command"
        );

        assert_eq!(
            "command for foobar:82 => 10003",
            config.render("foobar", 82, 10003),
            "defined service/port port should use specific command"
        );

        assert_eq!(
            "default foobar command",
            config.render("foobar", 1000, 10003),
            "unknown port should fallback on default service command"
        );
    }

    #[test]
    fn config_from_toml_no_default() {
        let config = super::Config::from_toml("testdata/config-no-default.toml")
            .expect("failed to parse config file");

        assert_eq!(
            "open http://localhost:10001",
            config.render("unknown-service", 8080, 10001),
            "undefined service and no default command should return none"
        );

        assert_eq!(
            "command for foobar:81",
            config.render("foobar", 81, 10002),
            "defined service/port port should use specific command"
        );

        assert_eq!(
            "command for foobar:82",
            config.render("foobar", 82, 10003),
            "defined service/port port should use specific command"
        );
    }

    #[test]
    fn config_from_toml_unparsable_command() {
        let config = super::Config::from_toml("testdata/config-unparsable-command.toml")
            .expect("failed to parse config file");

        assert_eq!(
            "Failed to parse template.",
            config.render("unknown-service", 8080, 10001),
            "unparsable command should display error"
        );
    }

    #[test]
    fn config_from_toml_invalid_format() {
        let err = super::Config::from_toml("testdata/config-bad-format.toml").unwrap_err();
        let crate::error::Error::Config(msg) = err else { panic!("invalid error type") };

        assert_eq!(
            "TOML parse error at line 1, column 5
  |
1 | this:
  |     ^
expected `.`, `=`
",
            msg
        )
    }

    #[test]
    fn config_from_toml_file_not_found() {
        let err = super::Config::from_toml("/tmp/do-not-exist.toml").unwrap_err();
        let crate::error::Error::Config(msg) = err else { panic!("invalid error type") };

        assert_eq!("No such file or directory (os error 2)", msg)
    }
}
