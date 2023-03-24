fn main() -> Result<(), docker_compose_portlist::Error> {
    let compose = docker_compose_portlist::dockercompose::Command::default();
    let fzf = docker_compose_portlist::fzf::Command::default();

    let formatted_data = compose
        .list_services()?
        .into_iter()
        .fold(Vec::new(), |mut data, s| {
            data.append(s.ports.into_iter().fold(&mut Vec::new(), |d, p| {
                d.push(format!(
                    "{}:{} [{}] {}",
                    s.service,
                    p.port,
                    s.state,
                    p.url()
                ));
                d
            }));

            data
        });

    fzf.execute(&formatted_data)
}
