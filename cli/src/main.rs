use std::env;
use std::process;
use cli::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    println!("{}, {}", cfg.filepath, cfg.query);

    run(cfg).expect("error");
}
