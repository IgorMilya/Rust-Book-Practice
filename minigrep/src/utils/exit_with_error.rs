use std::fmt::Display;
use std::process;

pub fn exit_with_error<T: Display>(prefix: &str, err: T) -> ! {
    eprintln!("{}: {}", prefix, err);
    process::exit(1);
}