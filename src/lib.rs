extern crate notify;

use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};

pub struct Config<'a> {
    paths: Vec<&'a Path>,
    delay: Duration,
    program: &'a Path,
    args: &'a Vec<String>,
}

impl<'a> Config<'a> {
    pub fn new(
        paths: &'a Vec<String>,
        delay: Duration,
        program: &'a Path,
        args: &'a Vec<String>,
    ) -> io::Result<Config<'a>> {
        let paths: Vec<&'a Path> = paths.iter().map(|path| Path::new(path)).collect();
        Ok(Config {
            paths,
            delay,
            program,
            args,
        })
    }

    pub fn paths(&self) -> &Vec<&Path> {
        &self.paths
    }

    pub fn delay(&self) -> Duration {
        self.delay
    }

    pub fn program(&self) -> &Path {
        self.program
    }

    pub fn args(&self) -> &Vec<String> {
        self.args
    }
}

pub fn watch(config: Config) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, config.delay())?;

    for path in config.paths() {
        watcher.watch(path, RecursiveMode::NonRecursive)?;
    }

    loop {
        match rx.recv() {
            Ok(notify::DebouncedEvent::Write(_)) => {
                let output = process::Command::new(config.program())
                    .args(config.args())
                    .output()?;
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            }
            Err(e) => eprintln!("{:?}", e),
            _ => (),
        }
    }
}
