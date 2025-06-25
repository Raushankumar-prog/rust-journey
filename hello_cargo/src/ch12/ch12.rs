use std::env;
use std::fs;
use std::process;

pub fn ch12() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let contents = fs::read_to_string(config.file).unwrap_or_else(|err| {
        println!("Problem reading file: {err}");
        process::exit(1);
    });

    println!("{}", contents);
}

struct Config {
    query: String,
    file: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file = args[2].clone();
        Ok(Config { query, file })
    }
}
