use create::macos::runner;

pub fn cpu() -> String {
    runner::execute_command(
        "hostname",
        &[]
    )
}