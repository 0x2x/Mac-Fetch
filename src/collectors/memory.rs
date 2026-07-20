use create::macos::runner;

pub fn memory() -> String {
    let bytes = runner::execute_command(
        "sysctl",
        &["-n", "hw.memsize"]
    ); // Execute Command to grab Memory

    let gb = bytes.parse::<u64>().unwrap() / 1024 /1024 /1024; // Convert MB to GB

    format!("{} GB", gb)
}