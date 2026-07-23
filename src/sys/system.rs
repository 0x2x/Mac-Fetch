use crate::collectors::{
    cpu::cpu, get_terminal::get_terminal, gpu::gpu, hostname::hostname, kernel::kernel, memory::memory, resolution::resolution, uptime::uptime
};

pub struct OS {
    pub os_name: String,
    pub host_name: String,
    pub kernel: String,
    pub uptime: String,
    pub packages: String,
    pub shell: String,
    pub resolution: String,
    pub terminal: String,
    pub cpu: String,
    pub gpu: String,
    pub memory: String,
    pub disk: String,
    
    // Dev Versions
    pub rust_version: String,
    pub cargo_version: String,
    pub git_version: String,
    pub homebrew_version: String,
    pub docker_version: String,
    pub node_npm_version: String,
    pub python_version: String,
    pub go_version: String,
    // Extras
    pub github_profile: String,
    pub time: String,
    pub date: String,
    // Spotify API
    pub spotify_name: String,
    pub spotify_now_playing_title: String,
    pub spotify_artist_name: String,
    // Apple Music TODO: Add variables
}

impl OS {
    pub fn new() -> Self {
        Self {
            os_name: String::new(),
            host_name: String::new(),
            kernel: String::new(),
            uptime: String::new(),
            packages: String::new(),
            shell: String::new(),
            resolution: String::new(),
            terminal: String::new(),
            cpu: String::new(),
            gpu: String::new(),
            memory: String::new(),
            disk: String::new(),
            
            // Dev Versions
            rust_version: String::new(),
            cargo_version: String::new(),
            git_version: String::new(),
            homebrew_version: String::new(),
            docker_version: String::new(),
            node_npm_version: String::new(),
            python_version: String::new(),
            go_version: String::new(),
            // Extras
            github_profile: String::new(),
            time: String::new(),
            date: String::new(),
            // Spotify API
            spotify_name: String::new(),
            spotify_now_playing_title: String::new(),
            spotify_artist_name: String::new(),
        }
        
    }
    pub fn os_details() -> OS { // Generate an object
        //
        let value_os_name = String::new();
        let value_host_name= hostname();
        let value_kernel = kernel();
        // TODO: 
        let value_uptime = uptime();
        let value_packages = String::new();
        let value_shell = String::new();
        let value_resolution = resolution();
        let value_terminal = get_terminal();
        let value_cpu = cpu();
        let value_gpu = gpu();
        let value_memory = memory();
        let value_disk = String::new();

        let value_spotify_name = String::new();
        let value_spotify_now_playing_track_name = String::new();
        let value_spotify_artist_name = String::new();

        let value_rust_version = String::new();
        let value_cargo_version = String::new();
        let value_git_version = String::new();
        let value_homebrew_version = String::new();
        let value_docker_version = String::new();
        let value_node_npm_version = String::new();
        let value_python_version = String::new();
        let value_go_version = String::new();
        // Extras
        let value_github_profile = String::new();
        let value_time = String::new();
        let value_date = String::new();
        return OS {
            os_name: value_os_name,
            host_name: value_host_name,
            kernel: value_kernel,
            uptime: value_uptime,
            packages: value_packages,
            shell: value_shell,
            resolution: value_resolution,
            terminal: value_terminal,
            cpu: value_cpu,
            gpu: value_gpu,
            memory: value_memory,
            disk: value_disk,
            spotify_name: value_spotify_name,
            spotify_now_playing_title:
            value_spotify_now_playing_track_name,
            spotify_artist_name: value_spotify_artist_name,
            rust_version: value_rust_version,
            cargo_version: value_cargo_version,
            git_version: value_git_version,
            homebrew_version: value_homebrew_version,
            docker_version: value_docker_version,
            node_npm_version: value_node_npm_version,
            python_version: value_python_version,
            go_version: value_go_version,
            github_profile: value_github_profile,
            time: value_time,
            date: value_date
        }
    }
}