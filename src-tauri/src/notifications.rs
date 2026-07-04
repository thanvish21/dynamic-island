use serde::Serialize;
use std::sync::LazyLock;
use std::sync::Mutex;

#[derive(Serialize, Clone)]
pub struct Notification {
    pub id: u64,
    pub app: String,
    pub title: String,
    pub body: String,
    pub timestamp: String,
    pub read: bool,
    pub icon: Option<String>,
}

static NOTIFICATIONS: LazyLock<Mutex<Vec<Notification>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static DND_MODE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

#[tauri::command]
pub fn get_notifications() -> Vec<Notification> {
    NOTIFICATIONS.lock().unwrap().clone()
}

#[tauri::command]
pub fn dismiss_notification(id: u64) -> bool {
    let mut notifs = NOTIFICATIONS.lock().unwrap();
    notifs.retain(|n| n.id != id);
    true
}

#[tauri::command]
pub fn toggle_dnd() -> bool {
    let mut dnd = DND_MODE.lock().unwrap();
    *dnd = !*dnd;
    *dnd
}

#[tauri::command]
pub fn get_dnd_status() -> bool {
    *DND_MODE.lock().unwrap()
}
