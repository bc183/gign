use clap::Parser;
use gign::Args;
use std::process;

fn main() {
    let args = Args::parse();
    if let Err(e) = gign::run(&args) {
        println!("{}", e);
        process::exit(1);
    }
}
