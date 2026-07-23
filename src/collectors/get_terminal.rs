use std::env;

pub fn get_terminal() -> String {
    env::var("TERM_PROGRAM")
        .clone()
        .unwrap()
}