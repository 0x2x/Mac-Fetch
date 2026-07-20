pub fn apple_logo() {
    print!("Imagine a Apple")
}

pub fn detail_line(key: &str, value: &str) {
    println!("\x1b[31m{}: {}\x1b[0m", key, value);
}