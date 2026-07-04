use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct MediaStatus {
    pub playing: bool,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_art_url: Option<String>,
    pub progress: f64,
    pub duration: f64,
    pub player: String,
}

#[tauri::command]
pub fn get_media_status() -> Option<MediaStatus> {
    #[cfg(target_os = "linux")]
    {
        // Try to get MPRIS media info via playerctl
        let output = std::process::Command::new("playerctl")
            .args([
                "metadata",
                "--format",
                "{{status}}\n{{title}}\n{{artist}}\n{{album}}\n{{mpris:artUrl}}\n{{position}}\n{{mpris:length}}\n{{playerName}}",
            ])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let text = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = text.trim().split('\n').collect();

        if lines.len() < 8 {
            return None;
        }

        let position: f64 = lines[5].parse().unwrap_or(0.0) / 1_000_000.0; // microseconds to seconds
        let length: f64 = lines[6].parse().unwrap_or(0.0) / 1_000_000.0;

        Some(MediaStatus {
            playing: lines[0] == "Playing",
            title: lines[1].to_string(),
            artist: lines[2].to_string(),
            album: lines[3].to_string(),
            album_art_url: if lines[4].is_empty() {
                None
            } else {
                Some(lines[4].to_string())
            },
            progress: position,
            duration: length,
            player: lines[7].to_string(),
        })
    }

    #[cfg(target_os = "windows")]
    {
        // Windows media session - simplified fallback
        None
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        None
    }
}

#[tauri::command]
pub fn media_play_pause() -> bool {
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("playerctl")
            .arg("play-pause")
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

#[tauri::command]
pub fn media_next() -> bool {
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("playerctl")
            .arg("next")
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

#[tauri::command]
pub fn media_previous() -> bool {
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("playerctl")
            .arg("previous")
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}
