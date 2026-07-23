use crate::collectors::sysctl::get_sysctl_result;

pub fn cpu() -> String { // TODO: fix this soon
    get_sysctl_result().get("machdep.cpu.brand_string")
        .cloned()
        .unwrap()
}