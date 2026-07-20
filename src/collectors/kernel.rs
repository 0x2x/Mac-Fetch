use create::macos::runner;

pub fn kernel() -> String {
    runner::execute_command(
        "sysctl",
        &["-n", "hw.memsize"]
    );
}