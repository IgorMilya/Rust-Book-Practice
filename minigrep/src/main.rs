use ::std::env;
use config::Config;

mod utils;
mod config;
mod run;


fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        utils::exit_with_error("Problem with arguments", err);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(err) = run::run(config) {
        utils::exit_with_error("Application error", err);
    }
}
