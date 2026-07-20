use create::macos::runner;

pub fn os_version() -> String {
    runner::execute_command(
        "sysctl",
        &["-productVersion"]
    )
}