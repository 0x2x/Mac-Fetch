use std::process::Command;

pub fn osascript_grab_spotify_track() -> String {
    let script = r#"
    tell application "Spotify"
        if player state is playing then
            return artist of current track & " - " & name of current track
        else
            return "NOT_PLAYING"
        end if
    end tell
    "#;

    let output = Command::new("osascript")
        .args(["-e", script])
        .output()
        .expect("Failed to execute osascript");

    let result = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    if result.is_empty() {
        None
    } else {
        Some(result)
    }

}