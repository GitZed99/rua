use std::env;
use std::process;

use rua::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = rua::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
