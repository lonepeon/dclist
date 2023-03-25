#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DockerCompose(String),
    Fzf(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DockerCompose(err) => {
                write!(f, "failed to get processes from docker-compose: {}", err)
            }
            Self::Fzf(err) => {
                write!(f, "failed to execute fzf: {}", err)
            }
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    #[test]
    fn error_displat_docker_compose() {
        let err = super::Error::DockerCompose("some explanation".to_string());
        assert_eq!(
            "failed to get processes from docker-compose: some explanation",
            format!("{}", err)
        )
    }

    #[test]
    fn error_displat_fzf() {
        let err = super::Error::Fzf("some explanation".to_string());
        assert_eq!(
            "failed to execute fzf: some explanation",
            format!("{}", err)
        )
    }
}
