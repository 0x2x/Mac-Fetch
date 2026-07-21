use std::alloc::alloc;
use crate::macos::runner;
use crate::collectors::sysctl::get_sysctl_result;
/*

Version 1
pub fn cpu() -> String {

    runner::execute_command(
        "sysctl",
        &["-n", "machdep.cpu.brand_string"]
    )
}
    */

// Version two
// This version is caching so sysctl is not being spammed 5+ Times.
// pub fn cpu() {
//     get_sysctl_result().get("machdep.cpu.brand_string");
// }

pub fn cpu() -> String { // TODO: fix this soon
    runner::execute_command(
        "sysctl",
        &["-n", "machdep.cpu.brand_string"]
    )
}