use std::env;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

use minientr::{self, Config};

fn run(config: Config) -> io::Result<()> {
    if let Err(e) = minientr::watch(config) {
        eprintln!("error: {:?}", e);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let program: &Path;
    let mut args: Vec<String> = Vec::new();

    let input_args: Vec<String> = env::args().collect();
    match &input_args[..] {
        [_] => panic!("no cmd"),
        cmd_with_args => {
            program = Path::new(&cmd_with_args[1]);
            for arg in &cmd_with_args[2..] {
                args.push(arg.clone());
            }
        }
    }

    let stdin = io::stdin();
    let handle = stdin.lock();

    let paths: Result<Vec<String>, io::Error> = handle.lines().collect();

    run(Config::new(&paths?, Duration::from_secs(2), program, &args)?)
}
