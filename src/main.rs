use clap::Parser;
use fox::engine;
use std::fs;

#[derive(clap::Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// The script file to execute
    #[clap(value_parser)]
    script: String,
}

fn main() {
    let args = Args::parse();

    engine::execute_script(args.script);
}
