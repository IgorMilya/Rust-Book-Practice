use ::std::env;
use minigrep::{exit_with_error, run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        exit_with_error("Problem with arguments", err);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(err) = run(config) {
        exit_with_error("Application error", err);
    }
}

