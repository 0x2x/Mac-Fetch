use crate::macos::runner;

pub fn gpu() -> String {
    runner::execute_command(
        "sysctl",
        &["-a", "|", "grep", "-i", "gpu"]
    )
}