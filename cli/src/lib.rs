use std::fs;
use std::error::Error;

/// Holds filepath and query that is to be searched in file content
pub struct Config {
    pub filepath: String,
    pub query: String,
}

impl Config {
    /// Builds Config object from given Iterator
    /// Frist item from args is skipped assuming it the name of binary executable
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let filepath = match args.next() {
            Some(a) => a,
            None => return Err("filepath not provided"),
        };

        let query = match args.next() {
            Some(a) => a,
            None => return Err("query not provided"),
        };

        return Ok(Config {
            filepath,
            query,
        });
    }
}

/// Finds given query in the given file content
///
/// # EXample
///
/// ```
/// let arg = vec!["execpath".to_string(), "filepath".to_string(), "query".to_string()];
/// let cfg = cli::onfig::build(arg.iter());
/// cli::run(cfg);
/// ```
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
        let cfg = Config::build(arg.iter());

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
