use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let case_sensitive = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, case_sensitive })
    }

    // fn new(args: &[String]) -> Result<Self, &str> {
    //     if args.len() < 3 {
    //         return Err("not enough arguments");
    //     }
    //
    //     Ok(Self {
    //         query: args[1].clone(),
    //         file_path: args[2].clone(),
    //         case_sensitive: env::var("IGNORE_CASE").is_ok(),
    //     })
    // }
}