use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &mut env::Args) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            _ => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            _ => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = search(&config.query, &contents, config.case_sensitive);

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let lower_query;

    let predicate: Box<dyn Fn(&str) -> bool> = if case_sensitive {
        Box::new(|line: &str| line.contains(query))
    } else {
        lower_query = query.to_lowercase();
        Box::new(|line: &str| line.to_lowercase().contains(&lower_query))
    };

    contents.lines().filter(|&line| predicate(line)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, false));
    }
}
