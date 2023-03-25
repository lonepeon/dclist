use std::io::Write;

pub struct Command {
    pub path: String,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            path: "fzf".to_string(),
        }
    }
}

impl Command {
    pub fn execute(&self, suggestions: &[String]) -> Result<(), crate::Error> {
        let data = suggestions.join("\n");

        let mut column_cmd = std::process::Command::new("column")
            .arg("-t")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| crate::Error::Fzf(format!("failed to spawn column program: {}", e)))?;

        // make sure we drop STDIN so the application doesn't hang, waiting for incoming data
        {
            column_cmd
                .stdin
                .take()
                .as_mut()
                .unwrap()
                .write_all(data.as_bytes())
                .map_err(|e| {
                    crate::Error::Fzf(format!("failed to write to column program: {}", e))
                })?;
        }

        let column_output = column_cmd.stdout.ok_or(crate::Error::Fzf(
            "failed to get ouput from column program".to_string(),
        ))?;

        std::process::Command::new(&self.path)
            .arg("--bind")
            .arg("enter:execute(open {3})+abort")
            .stdin(std::process::Stdio::from(column_output))
            .spawn()
            .map_err(|e| crate::Error::Fzf(format!("failed to spawn fzf program: {}", e)))?
            .wait()
            .map_err(|e| {
                crate::Error::Fzf(format!("failed to wait for user inpur in fzf: {}", e))
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn command_default() {
        let cmd = super::Command::default();
        assert_eq!("fzf", cmd.path)
    }

    #[test]
    fn execute_command() {
        let cmd = super::Command {
            path: "testdata/fzf-mock".to_string(),
        };

        let data = vec![
            "web:80 [running] http://localhost:8080".to_string(),
            "documentation:80 [exited] http://localhost:8081".to_string(),
        ];

        cmd.execute(&data).expect("failed to execute fzf");
    }
}
