use mac_fetch::display::display;
use mac_fetch::sys::{self, system};
fn main() {
    println!("Hello, world!");
    display::detail_line(&"Test", &"test");
    let t = system::OS::new();
}


