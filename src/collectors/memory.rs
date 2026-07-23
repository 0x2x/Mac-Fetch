use crate::collectors::sysctl::get_sysctl_result;

pub fn memory() -> String {    
    let bytes: String = get_sysctl_result().get("hw.memsize")
        .cloned()
        .unwrap();
    let gb = bytes.parse::<u64>().unwrap() / 1024 /1024 /1024; // Convert MB to GB
    format!("{} GB / {} MB", gb, bytes)
}