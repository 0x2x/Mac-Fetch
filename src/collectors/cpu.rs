use create::macos::runner;

pub fn cpu() -> String {
    runner::execute_command(
        "sysctl",
        &["-n", "machdep.cpu.brand_string"]
    )
}