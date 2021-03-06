use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new (mut args: env::Args) -> Result<Config, &'static str> {
        args.next();  // Pula o nome do arquivo main

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
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
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!(
            "{}",
            line.replace(
                &config.query,
                format!("\x1b[38;2;192;32;56m{}\x1B[0m", &config.query).as_str()
            )
        );
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Iterando diretamente com o iterador
    contents
        .lines()
        .filter(|line| line.contains(query)) // Closure usando 'query' do escopo
        .collect()

    /*
    Iterando com loop 'for'

    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.replace(
                query,
                format!("\x1b[38;2;192;32;56m{}\x1B[0m", query).as_str()
            ))
        }
    }

    results
    */
}

pub fn search_case_insensitive<'a>(
    query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = &query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents =  "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    
    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}