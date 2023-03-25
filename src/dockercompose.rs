pub struct Command<'a> {
    pub path: &'a str,
}

impl<'a> Command<'a> {
    pub fn new(path: &'a str) -> Self {
        Self { path }
    }

    pub fn list_services(&self) -> Result<Vec<Service>, crate::Error> {
        let output = std::process::Command::new(self.path)
            .arg("ps")
            .arg("--format")
            .arg("json")
            .output()
            .map_err(|e| crate::Error::DockerCompose(e.to_string()))?;

        serde_json::from_slice(&output.stdout)
            .map_err(|e| crate::Error::DockerCompose(e.to_string()))
    }
}

pub struct ServicePort {
    pub port: u16,
    pub exposed_port: u16,
}

impl ServicePort {
    pub fn new(port: u16, exposed_port: u16) -> Self {
        Self { port, exposed_port }
    }

    pub fn url(&self) -> String {
        format!("http://localhost:{}", self.exposed_port)
    }
}

pub struct Service {
    pub name: String,
    pub service: String,
    pub state: String,
    pub ports: Vec<ServicePort>,
}

impl Service {
    pub fn new(name: String, service: String, state: String, ports: Vec<ServicePort>) -> Self {
        Self {
            name,
            service,
            state,
            ports,
        }
    }
}

impl<'de> serde::Deserialize<'de> for Service {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let svc_json = ServiceJSON::deserialize(deserializer)?;

        let ports = svc_json
            .publishers
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|p| ServicePort::new(p.target_port, p.published_port))
            .collect();

        let svc = Self::new(svc_json.name, svc_json.service, svc_json.state, ports);

        Ok(svc)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
struct PublisherJSON {
    target_port: u16,
    published_port: u16,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ServiceJSON {
    name: String,
    service: String,
    state: String,
    publishers: Option<Vec<PublisherJSON>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn service_deserialize_json() {
        let services: Vec<super::Service> =
            serde_json::from_str(include_str!("../testdata/containers.json"))
                .expect("failed to parse JSON");

        assert_eq!(3, services.len());

        assert_eq!("example-bar-1", services[0].name);
        assert_eq!("bar", services[0].service);
        assert_eq!("exited", services[0].state);
        assert_eq!(0, services[0].ports.len());

        assert_eq!("example-foo-1", services[1].name);
        assert_eq!("foo", services[1].service);
        assert_eq!("running", services[1].state);
        assert_eq!(1, services[1].ports.len());
        assert_eq!(80, services[1].ports[0].port);
        assert_eq!(8080, services[1].ports[0].exposed_port);
        assert_eq!("http://localhost:8080", services[1].ports[0].url());
        assert_eq!("example-foobar-1", services[2].name);
        assert_eq!("foobar", services[2].service);
        assert_eq!("exited", services[2].state);
        assert_eq!(2, services[2].ports.len());
        assert_eq!(81, services[2].ports[0].port);
        assert_eq!(8081, services[2].ports[0].exposed_port);
        assert_eq!("http://localhost:8081", services[2].ports[0].url());
        assert_eq!(82, services[2].ports[1].port);
        assert_eq!(8082, services[2].ports[1].exposed_port);
        assert_eq!("http://localhost:8082", services[2].ports[1].url());
    }

    #[test]
    fn command_list_services() {
        let cmd = super::Command::new("testdata/docker-compose-mock");

        let services = cmd.list_services().expect("failed to execute mock");

        assert_eq!(3, services.len());

        assert_eq!("example-bar-1", services[0].name);
        assert_eq!("bar", services[0].service);
        assert_eq!("exited", services[0].state);
        assert_eq!(0, services[0].ports.len());

        assert_eq!("example-foo-1", services[1].name);
        assert_eq!("foo", services[1].service);
        assert_eq!("running", services[1].state);
        assert_eq!(1, services[1].ports.len());
        assert_eq!(80, services[1].ports[0].port);
        assert_eq!(8080, services[1].ports[0].exposed_port);
        assert_eq!("http://localhost:8080", services[1].ports[0].url());

        assert_eq!("example-foobar-1", services[2].name);
        assert_eq!("foobar", services[2].service);
        assert_eq!("exited", services[2].state);
        assert_eq!(2, services[2].ports.len());
        assert_eq!(81, services[2].ports[0].port);
        assert_eq!(8081, services[2].ports[0].exposed_port);
        assert_eq!("http://localhost:8081", services[2].ports[0].url());
        assert_eq!(82, services[2].ports[1].port);
        assert_eq!(8082, services[2].ports[1].exposed_port);
        assert_eq!("http://localhost:8082", services[2].ports[1].url());
    }
}
