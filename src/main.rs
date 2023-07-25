use clap::Parser;
use nocommit::{run, Cli, Settings};

fn main() {
    let arguments = Cli::parse();
    let settings = Settings::new(String::from("nocommit"), arguments);
    run(settings)
}
