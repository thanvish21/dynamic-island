use serde::Serialize;
use sysinfo::{Disks, Networks, System};

#[derive(Serialize, Clone)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub uptime: u64,
    pub cpu_count: usize,
}

#[derive(Serialize, Clone)]
pub struct CpuInfo {
    pub name: String,
    pub usage: f32,
    pub frequency: u64,
}

#[derive(Serialize, Clone)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub usage_percent: f64,
    pub swap_total: u64,
    pub swap_used: u64,
}

#[derive(Serialize, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub usage_percent: f64,
    pub fs_type: String,
}

#[derive(Serialize, Clone)]
pub struct NetworkInfo {
    pub interface: String,
    pub received: u64,
    pub transmitted: u64,
}

#[derive(Serialize, Clone)]
pub struct BatteryInfo {
    pub percent: f64,
    pub charging: bool,
    pub time_remaining: Option<String>,
}

#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());

    let mut sys = System::new_all();
    sys.refresh_all();

    SystemInfo {
        hostname,
        os_name,
        os_version,
        kernel_version,
        uptime: System::uptime(),
        cpu_count: sys.cpus().len(),
    }
}

#[tauri::command]
pub fn get_cpu_usage() -> Vec<CpuInfo> {
    let mut sys = System::new();
    sys.refresh_cpu_all();
    // Need a small delay for accurate readings
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu_all();

    sys.cpus()
        .iter()
        .map(|cpu| CpuInfo {
            name: cpu.name().to_string(),
            usage: cpu.cpu_usage(),
            frequency: cpu.frequency(),
        })
        .collect()
}

#[tauri::command]
pub fn get_memory_info() -> MemoryInfo {
    let mut sys = System::new();
    sys.refresh_memory();

    let total = sys.total_memory();
    let used = sys.used_memory();
    let free = sys.available_memory();

    MemoryInfo {
        total,
        used,
        free,
        usage_percent: if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        },
        swap_total: sys.total_swap(),
        swap_used: sys.used_swap(),
    }
}

#[tauri::command]
pub fn get_disk_info() -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();

    disks
        .iter()
        .map(|disk| {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total.saturating_sub(available);

            DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total_space: total,
                available_space: available,
                usage_percent: if total > 0 {
                    (used as f64 / total as f64) * 100.0
                } else {
                    0.0
                },
                fs_type: String::from_utf8_lossy(disk.file_system()).to_string(),
            }
        })
        .collect()
}

#[tauri::command]
pub fn get_network_info() -> Vec<NetworkInfo> {
    let networks = Networks::new_with_refreshed_list();

    networks
        .iter()
        .map(|(name, data)| NetworkInfo {
            interface: name.to_string(),
            received: data.total_received(),
            transmitted: data.total_transmitted(),
        })
        .collect()
}

#[tauri::command]
pub fn get_battery_info() -> BatteryInfo {
    // Read from /sys/class/power_supply on Linux
    #[cfg(target_os = "linux")]
    {
        let capacity = std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
            .unwrap_or_else(|_| "100".to_string())
            .trim()
            .parse::<f64>()
            .unwrap_or(100.0);

        let status = std::fs::read_to_string("/sys/class/power_supply/BAT0/status")
            .unwrap_or_else(|_| "Unknown".to_string());

        let charging = status.trim() == "Charging" || status.trim() == "Full";

        BatteryInfo {
            percent: capacity,
            charging,
            time_remaining: None,
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        BatteryInfo {
            percent: 100.0,
            charging: true,
            time_remaining: None,
        }
    }
}
