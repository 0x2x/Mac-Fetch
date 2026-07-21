
#[test] // INFO: Test 1
fn split_key_value() {
    let s = "this.is.a.test: 1";
    for line in s.lines() {
        let parts: Vec<&str> = line.splitn(2, ":").collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim();
            print!("Key: {}\nValue: {}\n\n", key, value)
        }
    }
}


use mac_fetch::collectors::sysctl::{self, get_sysctl_result};
use mac_fetch::display::output;

#[test] 
fn test_sysctl_init() {
    let result = get_sysctl_result();
    output::debug(result);
}