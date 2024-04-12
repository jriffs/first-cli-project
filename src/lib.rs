#![allow(unused, unused_imports)]
use std::fs; // gives access to file system module
use std::error::Error;
use std::env;


pub struct CliParams<'a> {
    pub search_string: &'a String,
    pub file_path: &'a String,
    pub ignore_case: bool,
}

impl<'a> CliParams <'a> {
    pub fn build(args: &'a [String]) -> Result<CliParams, &'static str> {
        if args.len() != 3 {
            return Err("please check the argments, should be 2 (search string & file name)");
        }
        let search_string = &args[1];
        let file_path = &args[2];
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(CliParams {
            search_string,
            file_path,
            ignore_case
        })
    }
}

pub fn run(config: CliParams) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.search_string, &contents)
    } else {
        search(&config.search_string, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut return_vec = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            return_vec.push(line);
        }
    }
    return_vec
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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



