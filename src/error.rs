#[derive(Debug)]
pub enum Error {
    DockerCompose(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DockerCompose(err) => {
                write!(f, "failed to get processes from docker-compose: {}", err)
            }
        }
    }
}

impl std::error::Error for Error {}
