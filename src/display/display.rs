use crate::api::github;

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
pub async fn github_line(username: &str) {
    // What are some stuff to show?
    match github::github::get_profile(&username).await {
        Ok(user) => {
            // Count earned follows from previous cache TODO: Implement
            let previous_count = 3; // TODO: Grab from cache
            let follower_count = user.followers;
            print!("\tGithub: {}\n\tEarned Followers: {}", &username, follower_count - previous_count)
        }
        Err(error) => {
            print!("\tGithub: Error\n\tError: {}", error)
        }
    }
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