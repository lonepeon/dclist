use std::io::Write;

pub struct Command<'a> {
    pub path: &'a str,
}

impl<'a> Command<'a> {
    pub fn new(path: &'a str) -> Self {
        Self { path }
    }
    pub fn execute(&self, suggestions: &[String]) -> Result<(), crate::Error> {
        let mut data = suggestions.join("\n");
        data.push('\n');

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

        let status = std::process::Command::new(self.path)
            .arg("--bind")
            .arg("enter:execute(open {3})+accept")
            .stdin(std::process::Stdio::from(column_output))
            .spawn()
            .map_err(|e| crate::Error::Fzf(format!("failed to spawn fzf program: {}", e)))?
            .wait()
            .map_err(|e| {
                crate::Error::Fzf(format!("failed to wait for fzf program to finish: {}", e))
            })?;

        if !status.success() {
            return Err(crate::Error::Fzf(format!(
                "fzf exited with code {}",
                status.code().unwrap(),
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn execute_command_success() {
        let cmd = super::Command::new("testdata/fzf-mock");

        let data = vec![
            "foo:80     [running]  http://localhost:8080".to_string(),
            "foobar:81  [exited]   http://localhost:8081".to_string(),
            "foobar:82  [exited]   http://localhost:8082".to_string(),
        ];

        cmd.execute(&data).expect("failed to execute fzf");
    }

    #[test]
    fn execute_command_fail() {
        let cmd = super::Command::new("testdata/fzf-failing-mock");

        let data = vec![];

        let err = cmd.execute(&data).unwrap_err();

        let msg = if let crate::error::Error::Fzf(msg) = err {
            msg
        } else {
            panic!("unexpected error")
        };

        assert!(msg.starts_with("fzf exited with code "))
    }
}
