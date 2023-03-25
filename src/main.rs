use clap::Parser;

fn main() -> Result<(), dclist::Error> {
    let rst = dclist::cli::Cli::parse().execute();
    if let Some(error) = rst.err() {
        eprintln!("{}", error);
        std::process::exit(1);
    }

    Ok(())
}
