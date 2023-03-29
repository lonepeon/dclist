[![codecov](https://codecov.io/gh/lonepeon/dclist/branch/main/graph/badge.svg?token=HC7WSVDRO2)](https://codecov.io/gh/lonepeon/dclist)

# dclist

A fuzzy finder to open any docker-compose exposed service. This tool is helpful
when juggling with a lot of different projects which does not reuse the same
ports. Remembering the exposed ports per projet is not manageable.

Preview of the CLI output. When selecting one of the line below, it send the
selected line to the shell. The `fzf-dclist` binary is not intended to be used
directly. Instead a shell function should call it in order to fill the prompt
with the expected command.

![preview](assets/preview.png)

CLI usage is shown with `--help`

```
Usage: fzf-dclist [OPTIONS]

Options:
      --docker-compose-path <DOCKER_COMPOSE_PATH>
          path to the docker compose binary [default: docker-compose]
      --fzf-path <FZF_PATH>
          path to the fzf binary [default: fzf]
  -c, --config-path <CONFIG_PATH>
          path to a config file
  -h, --help
          Print help
  -V, --version
          Print version
```

## Config

By default, the CLI generates a `open http://localhost:<port>` command but sometimes we need to set a specialized command. For such usecases, a TOML config file can be given when running the program.

Below the format of the file:

```toml
# file:config.toml
# available variable are:
# - {{exposed_port}}: the docker-compose service port exposed to the host

# default command to use if nothing else matches
command = "open http://localhost:{{exposed_port}}"

# command to use for all my-service services if nothing more specific matches
[services.my-service]
command = "my-specific-command localhost {{exposed_port}}"

# command to use for all my-service services running on internal port 8080
[services.my-service.ports.8080]
command = "more-specific-command localhost {{exposed_port}}"
```

## Installation

Pre-requisites:
- docker-compose: in order to list all docker-compose processes
- fzf: in order to display a fuzzy-finder

A pre-built binary for `fzf-dclist` is available on the [release
page](https://github.com/lonepeon/dclist/releases).

## ZSH

Integration to ZSH is as easy as dropping this function in your zshrc:

```zsh
dclist() {
  local cmd=$(fzf-dclist | awk '{ for (i=3; i<=NF; i++) printf "%s ", $i }');
  print -z "${cmd}"
}
```
