use clap::Parser;

fn main() -> Result<(), fzf_dclist::Error> {
    let rst = fzf_dclist::cli::Cli::parse().execute();
    if let Some(error) = rst.err() {
        eprintln!("{}", error);
        std::process::exit(1);
    }

    Ok(())
}
