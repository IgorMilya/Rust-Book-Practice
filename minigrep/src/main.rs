use ::std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem with arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(err) = run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    }

}

fn run(config: Config) -> Result<(), Box<dyn Error>>  {
    let contents = fs::read_to_string(&config.file_path)?;

    println!("{}", contents);

    Ok(())
}

struct Config {
    query: String,
    file_path: String,

}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }

    // fn new(args: &[String]) -> Result<Self, &str> {
    //     if args.len() < 3 {
    //         return Err("not enough arguments");
    //     }
    //
    //     Ok(Self {
    //         query: args[1].clone(),
    //         file_path: args[2].clone(),
    //     })
    // }
}
