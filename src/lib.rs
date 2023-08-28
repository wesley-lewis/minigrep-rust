use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String, 
    pub filename: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line); 
        }
    }
    results
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 2 {
            let query = args[1].clone();
            let filename = args[2].clone();

            Ok(Config { query, filename })
        } else {
            // panic!("Usage --query --filename");
            return Err("Not enough arguments");
        }       
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe, fast, productive.";
        let contents = "\
Rust: 
safe, fast, productive.
Pick Three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
