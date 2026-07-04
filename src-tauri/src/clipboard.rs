use chrono::Utc;
use serde::Serialize;
use std::sync::LazyLock;
use std::sync::Mutex;

#[derive(Serialize, Clone)]
pub struct ClipboardEntry {
    pub content: String,
    pub timestamp: String,
    pub content_type: String,
}

static CLIPBOARD_HISTORY: LazyLock<Mutex<Vec<ClipboardEntry>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

#[tauri::command]
pub fn get_clipboard_history() -> Vec<ClipboardEntry> {
    CLIPBOARD_HISTORY.lock().unwrap().clone()
}

#[tauri::command]
pub fn add_clipboard_entry(content: String) -> bool {
    let entry = ClipboardEntry {
        content,
        timestamp: Utc::now().to_rfc3339(),
        content_type: "text".to_string(),
    };

    let mut history = CLIPBOARD_HISTORY.lock().unwrap();
    // Keep last 50 entries
    if history.len() >= 50 {
        history.remove(0);
    }
    history.push(entry);
    true
}

#[tauri::command]
pub fn clear_clipboard_history() -> bool {
    CLIPBOARD_HISTORY.lock().unwrap().clear();
    true
}
