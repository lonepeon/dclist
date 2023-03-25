fn main() -> Result<(), dclist::Error> {
    let compose = dclist::dockercompose::Command::default();
    let fzf = dclist::fzf::Command::default();

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
