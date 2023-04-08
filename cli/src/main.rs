use std::env;
use std::process;
use cli::{run, Config};

fn main() {
    let cfg = Config::build(env::args()).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    println!("{}, {}", cfg.filepath, cfg.query);

    run(cfg).expect("error");
}
