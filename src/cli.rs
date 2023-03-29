#[derive(clap::Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(
        long = "docker-compose-path",
        default_value = "docker-compose",
        help = "path to the docker compose binary"
    )]
    docker_compose_path: String,
    #[arg(
        long = "fzf-path",
        default_value = "fzf",
        help = "path to the fzf binary"
    )]
    fzf_path: String,
    #[arg(short = 'c', long = "config-path", help = "path to a config file")]
    config_path: Option<String>,
}

impl Cli {
    pub fn execute(&self) -> Result<(), crate::Error> {
        let config = match self.config_path.as_ref() {
            Some(path) => crate::config::Config::from_toml(path)?,
            None => crate::config::Config::default(),
        };
        let compose = crate::dockercompose::Command::new(&self.docker_compose_path);
        let fzf = crate::fzf::Command::new(&self.fzf_path);
        let formatted_data = crate::fzf::format_commands(&config, &compose.list_services()?);

        fzf.execute(&formatted_data)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cli_execute() {
        let cli = super::Cli {
            config_path: None,
            docker_compose_path: "testdata/docker-compose-mock".to_string(),
            fzf_path: "testdata/fzf-mock".to_string(),
        };

        cli.execute().expect("failed to execute CLI");
    }

    #[test]
    fn cli_execute_with_config() {
        let cli = super::Cli {
            config_path: Some("testdata/config.toml".to_string()),
            docker_compose_path: "testdata/docker-compose-mock".to_string(),
            fzf_path: "testdata/fzf-mock-with-config".to_string(),
        };

        cli.execute().expect("failed to execute CLI");
    }
}
