use std::collections::HashMap;
use std::sync::OnceLock;


// Using same system as src/collectors/sysctl.rs
static CACHE: OnceLock<HashMap<String, String>> = OnceLock::new(); // Data exists until Application Exits

// cache config file
pub fn cache_config() -> &'static HashMap<String, String> { // TODO: verify working
    CACHE.get_or_init(|| {
        init() // TODO: make sure function works
    })
}

// Initalize Config
// TODO: complete all functions

pub fn init() -> HashMap<String, String>{
    if exist() { // if config file already exists

    } else { // if the file doesn't exist
        if create_config() { // create the file if it doesnt exist

        } else { //  an issue occurs while creating config

        }
    }

    let hashmap = HashMap::new();
    hashmap
}
// config functions
pub fn exist() -> bool{
    // Check if config file does exist
    return false;
}
pub fn create_config() -> bool{
    return false;
}
pub fn load_config() -> bool {
    return false;
}
// config quick functions
pub fn get() {

}

pub fn update() {

}

pub fn insert() {

}

pub fn delete() {

}