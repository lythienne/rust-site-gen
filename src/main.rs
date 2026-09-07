use std::env;
use std::fs;
use std::process;
use std::error::Error;

use site_gen::compile;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = parse_args(&args).unwrap_or_else(
        |err| {
            eprintln!("Error parsing arguments: {err}");
            process::exit(1);
        }
    );
    if let Err(e) = run(filename) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    println!("contents:\n{contents}");

    let html_gen = compile(&contents);
    println!("html:\n{html_gen}");

    Ok(())
}

fn parse_args(args: &[String]) -> Result<&str, &'static str> {
    if args.len() < 2 {
        Err("site_gen needs a target .md file. Usage: site_gen [.md file]")
    }
    else {
        Ok(&args[1])
    }
}
