use crate::macos::runner;

pub fn ostype() -> String {
    runner::execute_command(
        "sysctl",
        &["-n", "kern.ostype"]
    )
}