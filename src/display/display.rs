pub fn apple_logo() {
    print!("Imagine a Apple")
}

pub fn display_fetch() {
    let r = "                        .8 
                      .888
                    .8888'
                   .8888'
                   888'
                   8'
      .88888888888. .88888888888.
   .8888888888888888888888888888888.
 .8888888888888888888888888888888888.
.&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&'
&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&'
&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&'
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@:
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@:
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@:
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%.
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%.
`%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%.
 `00000000000000000000000000000000000'
  `000000000000000000000000000000000'
   `0000000000000000000000000000000'
     `###########################'
jgs    `#######################'
         `#########''########'
           `\"\"\"\"\"\"'  `\"\"\"\"\"\'
";
}

pub fn header(username: &str, computername: &str) {
    // display: username@computername
    // example: ray@IamAComputer
    println!("\x1b[1;31m{}\x1b[37m@\x1b[0m\x1b[1;31m{}\x1b[37m", username, computername);
}
pub fn divider() {
    println!("-----------------------")
}
pub fn detail_line(key: &str, value: &str) {
    println!("\x1b[1;31m {}:\x1b[37m {}\x1b[0m", key, value);
}

pub fn color_array() {
    let optional_colors = [
        "\x1b[40m", // Black
        "\x1b[41m", // Red
        "\x1b[42m", // Green
        "\x1b[43m", // Yellow
        "\x1b[44m", // Blue
        "\x1b[45m", // Magenta
        "\x1b[46m", // Cyan
        "\x1b[47m" // White
    ];

    for color in optional_colors { // Loop though optional_colors and print result
        print!("{}  ", color);
    }
}


// third party
pub fn github_line(username: &str) {
    // What are some stuff to show?
    
    println!("https://github.com/{}", username);
}

pub fn spotify_line(track: &str, artist: &str) {
    println!("Listening to {} by {}", track, artist); // TODO: Change foreground color to green
}

pub fn apple_line(track: &str, artist: &str) {
    println!("Listening to {} by {}", track, artist); // TODO: Change foreground to a apple-music scheme color?
}

pub fn lastfm(track: &str, artist: &str) {
    println!("Listening to {} by {}", track, artist);
}