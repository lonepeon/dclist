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
}

impl Cli {
    pub fn execute(&self) -> Result<(), crate::Error> {
        let compose = crate::dockercompose::Command::new(&self.docker_compose_path);
        let fzf = crate::fzf::Command::new(&self.fzf_path);
        let formatted_data = crate::fzf::format_commands(&compose.list_services()?);

        fzf.execute(&formatted_data)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cli_execute() {
        let cli = super::Cli {
            docker_compose_path: "testdata/docker-compose-mock".to_string(),
            fzf_path: "testdata/fzf-mock".to_string(),
        };

        cli.execute().expect("failed to execute CLI");
    }
}