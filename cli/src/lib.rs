use std::fs;
use std::error::Error;


pub struct Config {
    pub filepath: String,
    pub query: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        return Ok(Config {
            filepath: args[1].clone(),
            query: args[2].clone(),
        });
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let c = fs::read_to_string(&cfg.filepath)?;

    let found = c.contains(cfg.query.as_str());

    if found {
        println!("found");
    } else {
        println!("not found");
    }
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build() {
        let arg = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        let cfg = Config::build(&arg);

        assert_eq!(cfg.is_err(), false);

        let c = cfg.unwrap();
        assert_eq!(c.filepath, arg[1]);
        assert_eq!(c.query, arg[2]);

    }

    #[test]
    fn run() {
        //TODO

    }
}
