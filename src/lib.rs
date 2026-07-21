pub mod display { // Include Graphical Tui Information
    pub mod display;
    pub mod output;
}

pub mod sys { // Include System Attributes
    pub mod system;
}

pub mod spotify { // Spotify
    pub mod token; // Spotifys refresh/Active tokens
    pub mod spotify; // Spotify functionality
}

pub mod apple_music {
    pub mod osascripts { // Complete
        // pub mod osascript;
    }
}


pub mod macos { // MacOS 
    pub mod runner; // Execute terminal commands at ease
}


pub mod collectors {
    pub mod cpu;
    pub mod hostname;
    pub mod kernel;
    pub mod memory;
    pub mod os_version;
    pub mod gpu;
    pub mod os_type; // Display: Darwin
    pub mod resolution;
    pub mod sysctl;
}