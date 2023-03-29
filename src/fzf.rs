use std::io::Write;

use itertools::Itertools;

pub struct Command<'a> {
    pub path: &'a str,
}

impl<'a> Command<'a> {
    pub fn new(path: &'a str) -> Self {
        Self { path }
    }
    pub fn execute(&self, suggestions: &[String]) -> Result<(), crate::Error> {
        let data = suggestions.join("\n");

        let mut cmd = std::process::Command::new(self.path)
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| crate::Error::Fzf(format!("failed to spawn fzf program: {}", e)))?;

        // make sure we drop STDIN so the application doesn't hang, waiting for incoming
        // data
        {
            cmd.stdin
                .take()
                .as_mut()
                .ok_or_else(|| {
                    crate::Error::Fzf("failed to get STDIN handle for fzf program".to_string())
                })?
                .write_all(&data.into_bytes())
                .map_err(|e| crate::Error::Fzf(format!("failed to write to fzf's STDIN: {}", e)))?;
        }

        let status = cmd.wait().map_err(|e| {
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

pub fn format_commands(
    cfg: &crate::config::Config,
    services: &[crate::dockercompose::Service],
) -> Vec<String> {
    struct Fields {
        service: String,
        state: String,
        cmd: String,
    }

    let entries = services
        .iter()
        .flat_map(|service| {
            service.ports.iter().map(|port| Fields {
                service: format!("{}:{}", service.service, port.port),
                state: service.state.clone(),
                cmd: cfg.render(&service.service, port.port, port.exposed_port),
            })
        })
        .collect_vec();

    let largest_service = entries.iter().map(|s| s.service.len()).max().unwrap_or(0);
    let largest_state = entries.iter().map(|s| s.state.len()).max().unwrap_or(0);

    entries
        .into_iter()
        .map(|f| {
            format!(
                "{:<service_pad$} [{}]{} {}",
                f.service,
                f.state,
                " ".repeat(largest_state - f.state.len()),
                f.cmd,
                service_pad = largest_service,
            )
        })
        .sorted()
        .unique()
        .collect_vec()
}

#[cfg(test)]
mod tests {
    #[test]
    fn execute_command_success() {
        let cmd = super::Command::new("testdata/fzf-mock");

        let data = vec![
            "foo:80    [running] open http://localhost:8080".to_string(),
            "foobar:81 [exited]  open http://localhost:8081".to_string(),
            "foobar:82 [exited]  open http://localhost:8082".to_string(),
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
