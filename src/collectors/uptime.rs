use crate::macos::runner;

pub fn uptime() -> String {
    runner::execute_command(
        "sysctl",
        &["-n", "machdep.cpu.brand_string"]
    )
}