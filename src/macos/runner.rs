use std::process::Command;

pub fn execute_command(command: &str, args: &[&str]) -> String {
    let output = Command::new(command)
        .args(args)
        .output()
        .unwrap();

    String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string()
    
}