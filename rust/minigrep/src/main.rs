use std::{env, fs};

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        // we use build instead of new because it is often
        // expected that new will not fail
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Self {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args);

    let content =
        fs::read_to_string(config.file_path).expect("Unreadable file, please enter a valid file");

    println!("{content}")
}
