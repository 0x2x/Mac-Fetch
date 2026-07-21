use crate::macos::runner;

pub fn hostname() -> String {
    runner::execute_command(
        "hostname",
        &[]
    )
}