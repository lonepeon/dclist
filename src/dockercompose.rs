pub struct Error;

pub struct Service {
    pub name: String,
    pub service: String,
    pub published_ports: Vec<u16>,
}

impl Service {
    pub fn new(name: String, service: String, published_ports: Vec<u16>) -> Self {
        Self {
            name,
            service,
            published_ports,
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
            .map(|p| p.published_port)
            .collect();

        let svc = Self::new(svc_json.name, svc_json.service, ports);

        Ok(svc)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
struct PublisherJSON {
    pub published_port: u16,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ServiceJSON {
    pub name: String,
    pub service: String,
    pub publishers: Option<Vec<PublisherJSON>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_service() {
        let services: Vec<super::Service> = serde_json::from_str(
            r#"
            [
              {
                "ID": "1553b0236cf4d2715845f053a4ee97042c4f9a2ef655731ee34f1f7940eaa41a",
                "Name": "example-bar-1",
                "Command": "/docker-entrypoint.sh nginx -g daemon off;",
                "Project": "example",
                "Service": "bar",
                "State": "exited",
                "Health": "",
                "ExitCode": 0,
                "Publishers": null
              },
              {
                "ID": "f02a4efaabb67416e1ff127d51c4b5578634a0ad5743bd65225ff7d1909a3fa0",
                "Name": "example-foo-1",
                "Command": "/docker-entrypoint.sh nginx -g daemon off;",
                "Project": "example",
                "Service": "foo",
                "State": "running",
                "Health": "",
                "ExitCode": 0,
                "Publishers": [
                  {
                    "URL": "0.0.0.0",
                    "TargetPort": 80,
                    "PublishedPort": 8080,
                    "Protocol": "tcp"
                  }
                ]
              }
            ]
        "#,
        )
        .expect("failed to parse JSON");

        assert_eq!(2, services.len());

        assert_eq!("example-bar-1", services[0].name);
        assert_eq!("bar", services[0].service);
        assert_eq!(0, services[0].published_ports.len());

        assert_eq!("example-foo-1", services[1].name);
        assert_eq!("foo", services[1].service);
        assert_eq!(1, services[1].published_ports.len());
        assert_eq!(8080, services[1].published_ports[0]);
    }
}
