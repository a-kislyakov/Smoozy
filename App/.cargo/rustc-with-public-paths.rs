// Cargo's Windows entry point. Python still owns all public-path settings.
// Built once by rustc-with-public-paths.py, outside Cargo's wrapper chain.
use std::{
    env,
    process::{self, Command},
};

fn main() {
    let status = Command::new(env!("SMOOZY_PUBLIC_PATHS_PYTHON"))
        .arg(env!("SMOOZY_PUBLIC_PATHS_HELPER"))
        .args(env::args_os().skip(1))
        .status()
        .unwrap_or_else(|error| {
            eprintln!("could not start the public-path helper: {error}");
            process::exit(1);
        });
    process::exit(status.code().unwrap_or(1));
}
