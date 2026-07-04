use tauri::Manager;

mod clipboard;
mod media;
mod notifications;
mod system;
mod timer;
mod weather;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Dynamic Island 🏝️", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Get the main window and configure it for dynamic island look
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_decorations(false);
                let _ = window.set_always_on_top(true);
                let _ = window.set_skip_taskbar(true);
                // Center at top of screen
                let monitor = window.current_monitor().ok().flatten();
                if let Some(monitor) = monitor {
                    let screen_width = monitor.size().width as f64 / monitor.scale_factor();
                    let x = (screen_width / 2.0) - 200.0; // 400px wide island / 2
                    let _ = window.set_position(tauri::PhysicalPosition::new(x as i32, 10));
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            system::get_system_info,
            system::get_cpu_usage,
            system::get_memory_info,
            system::get_disk_info,
            system::get_network_info,
            system::get_battery_info,
            media::get_media_status,
            media::media_play_pause,
            media::media_next,
            media::media_previous,
            weather::get_weather,
            clipboard::get_clipboard_history,
            clipboard::add_clipboard_entry,
            clipboard::clear_clipboard_history,
            notifications::get_notifications,
            notifications::dismiss_notification,
            notifications::toggle_dnd,
            notifications::get_dnd_status,
            timer::start_timer,
            timer::stop_timer,
            timer::get_timer_status,
            timer::start_stopwatch,
            timer::stop_stopwatch,
            timer::get_stopwatch_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Dynamic Island");
}
