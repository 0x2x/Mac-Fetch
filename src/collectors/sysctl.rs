// Realized, the current method is spamming sysctl...Fixed before issued is Committed
use std::sync::OnceLock;
use std::collections::HashMap;
use crate::macos::runner;


static SYSCTL_PARSED_RESULTS: OnceLock<HashMap<String, String>> = OnceLock::new(); // Data exists until Application Exits

fn sysctl_parse() -> HashMap<String, String> { // Parse through datat
    let sysctl_result = runner::execute_command(
        "sysctl",
        &["-a"]
    );

    let mut sysctl_hashmap: HashMap<String, String> = HashMap::new();
    /*
    Explaination: we want key and value.
    To make that happen I decidied to split by ':' which would then give me ["user.cs_path", "/usr..."],
        then I decidided to do a For loop and grab the key and value
        if key is NOT empty then insert key and value into a hashmap to summarize it all up.

    Result Example: 
    user.cs_path: /usr/bin:/bin:/usr/sbin:/sbin
    user.bc_base_max: 99
    user.bc_dim_max: 2048
    user.bc_scale_max: 99
    user.bc_string_max: 1000
    user.coll_weights_max: 2
    user.expr_nest_max: 32
    user.line_max: 2048
    user.re_dup_max: 255
    kern.skywalk.flowswitch.anpi0.ipfm.frag_count: 0
     */

    // Using Test 1 function
    for line in sysctl_result.lines() {
        let parts: Vec<&str> = line.splitn(2, ":").collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim();
            sysctl_hashmap.insert(key.to_string(), value.to_string()); // {"user.line_max": 2048}
        }
    }

    return sysctl_hashmap
}

pub fn get_sysctl_result() -> &'static HashMap<String, String> { // TODO: verify working
    SYSCTL_PARSED_RESULTS.get_or_init(|| {
        sysctl_parse()
    })
}