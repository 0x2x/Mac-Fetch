use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};

const BLUE: &str = "\x1b[34m";
const GRAY: &str = "\x1b[90m";
const RESET: &str = "\x1b[0m";

pub fn print<T: Debug>(value: T) {
    println!("{:#?}", value)
}

pub fn debug<T: Debug>(message: T) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!(
        "{GRAY}{BLUE}{timestamp}{GRAY}] [{BLUE}Debug{GRAY}] {RESET}{:#?}",
        message
    )


}