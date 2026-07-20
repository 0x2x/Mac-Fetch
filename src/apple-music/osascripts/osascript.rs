use std::process::Command;

pub fn GET_apple_music_track_currently_running() {
    let script = r#"
    if application "Music" is running then
        tell application "Music"
            if player state is playing then
                return artist of current track & " - " & name of current track
            else
                return "NOT_PLAYING"
            end if
        end tell
    else
        return "NOT_RUNNING"
    end if
    "#;

    let output = Commands::new("osascript")
        .args(["-e", script])
        .output()
        .expect("Failed to run AppleScript");

    let result = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();
    match result.as_str() {
        "NOT_RUNNING" => "Music is not running".to_string(),
        "NOT_PLAYING" => "Music is paused".to_string(),
        _ => result,
    }
}