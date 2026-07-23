use mac_fetch::display::display;
use mac_fetch::sys::{self, system};

fn main() {

    // Hard-Coded to test formatting and views
    let node_npm_exist: bool = true;
    let git_version: bool = false;
    let python_version: bool = false;
    let go_version: bool = false;

    let use_apple_music: bool = false;
    let use_spotify: bool = true;
    let use_lastfm: bool = false;
    let use_github: bool = false;

    let os_details = system::OS::os_details();
    // Display details - Right Panel TODO: Fill with lines
    display::header(&"Ray", "Yep");
    if use_spotify {
        display::spotify_line("test", "test");
    }
    if use_apple_music {
        display::apple_line("test", "test");
    }
    if use_github {
        display::github_line("0x2x");
    }
    if use_lastfm {
        display::lastfm("test", "artist");
    }
    display::divider();
    display::detail_line(&"OS", &os_details.os_name); // OS
    display::detail_line(&"Host", &os_details.host_name); // Host
    
    display::detail_line(&"Model", &"test"); // Model
    display::detail_line(&"Kernel", &os_details.kernel); // Kernel
    display::detail_line(&"Uptime", &os_details.uptime); // Uptime
    display::detail_line(&"Packages", &"test_AppleMacbook"); // Packages
    display::detail_line(&"Shell", &"test_AppleMacbook"); // Shell
    display::detail_line(&"Resolution", &os_details.resolution); // Resolution
    display::detail_line(&"DE", &"test_AppleMacbook"); // OS
    display::detail_line(&"WM", &"test_AppleMacbook"); // OS
    display::detail_line(&"WM Theme", &"test_AppleMacbook"); // OS
    display::detail_line(&"Icons", &"test_AppleMacbook"); // OS
    display::detail_line(&"Terminal", &"test_AppleMacbook"); // OS
    display::detail_line(&"CPU", &os_details.cpu); // TODO: Fix Type
    display::detail_line(&"GPU", &os_details.gpu); // OS
    display::detail_line(&"Memory", &os_details.memory); // OS
    if node_npm_exist == true || git_version == true || python_version == true || go_version == true {
        display::divider();
        if node_npm_exist {
            display::detail_line("Node/NPM Version: ", "Test");
        }

        if git_version {
            display::detail_line("Git: ", "Test");
        }

        if python_version {
            display::detail_line("Python: ", "Test");
        }

        if go_version {
            display::detail_line("Go-lang: ", "Test");
        }
        
    }
    display::color_array();
}


