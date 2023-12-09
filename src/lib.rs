use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let lines = search(&config.query, &contents);
    println!("{:#?}", lines);
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            lines.push(line);
        }
    }
    lines
}

pub struct Config {
    file_path: String,
    query: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enought arguments!!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Self { query, file_path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
