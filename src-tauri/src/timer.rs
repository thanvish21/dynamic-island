use serde::Serialize;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::time::Instant;

#[derive(Serialize, Clone)]
pub struct TimerStatus {
    pub active: bool,
    pub remaining_seconds: f64,
    pub total_seconds: f64,
    pub label: String,
}

#[derive(Serialize, Clone)]
pub struct StopwatchStatus {
    pub active: bool,
    pub elapsed_seconds: f64,
}

struct TimerState {
    active: bool,
    start_time: Option<Instant>,
    duration_secs: f64,
    label: String,
}

struct StopwatchState {
    active: bool,
    start_time: Option<Instant>,
    accumulated: f64,
}

static TIMER: LazyLock<Mutex<TimerState>> = LazyLock::new(|| {
    Mutex::new(TimerState {
        active: false,
        start_time: None,
        duration_secs: 0.0,
        label: String::new(),
    })
});

static STOPWATCH: LazyLock<Mutex<StopwatchState>> = LazyLock::new(|| {
    Mutex::new(StopwatchState {
        active: false,
        start_time: None,
        accumulated: 0.0,
    })
});

#[tauri::command]
pub fn start_timer(seconds: f64, label: Option<String>) -> TimerStatus {
    let mut timer = TIMER.lock().unwrap();
    timer.active = true;
    timer.start_time = Some(Instant::now());
    timer.duration_secs = seconds;
    timer.label = label.unwrap_or_else(|| "Timer".to_string());

    TimerStatus {
        active: true,
        remaining_seconds: seconds,
        total_seconds: seconds,
        label: timer.label.clone(),
    }
}

#[tauri::command]
pub fn stop_timer() -> TimerStatus {
    let mut timer = TIMER.lock().unwrap();
    timer.active = false;
    let remaining = if let Some(start) = timer.start_time {
        (timer.duration_secs - start.elapsed().as_secs_f64()).max(0.0)
    } else {
        0.0
    };
    timer.start_time = None;

    TimerStatus {
        active: false,
        remaining_seconds: remaining,
        total_seconds: timer.duration_secs,
        label: timer.label.clone(),
    }
}

#[tauri::command]
pub fn get_timer_status() -> TimerStatus {
    let timer = TIMER.lock().unwrap();
    let remaining = if timer.active {
        if let Some(start) = timer.start_time {
            (timer.duration_secs - start.elapsed().as_secs_f64()).max(0.0)
        } else {
            0.0
        }
    } else {
        0.0
    };

    TimerStatus {
        active: timer.active && remaining > 0.0,
        remaining_seconds: remaining,
        total_seconds: timer.duration_secs,
        label: timer.label.clone(),
    }
}

#[tauri::command]
pub fn start_stopwatch() -> StopwatchStatus {
    let mut sw = STOPWATCH.lock().unwrap();
    sw.active = true;
    sw.start_time = Some(Instant::now());

    StopwatchStatus {
        active: true,
        elapsed_seconds: sw.accumulated,
    }
}

#[tauri::command]
pub fn stop_stopwatch() -> StopwatchStatus {
    let mut sw = STOPWATCH.lock().unwrap();
    sw.active = false;
    if let Some(start) = sw.start_time.take() {
        sw.accumulated += start.elapsed().as_secs_f64();
    }

    StopwatchStatus {
        active: false,
        elapsed_seconds: sw.accumulated,
    }
}

#[tauri::command]
pub fn get_stopwatch_status() -> StopwatchStatus {
    let sw = STOPWATCH.lock().unwrap();
    let elapsed = if sw.active {
        sw.accumulated + sw.start_time.map(|s| s.elapsed().as_secs_f64()).unwrap_or(0.0)
    } else {
        sw.accumulated
    };

    StopwatchStatus {
        active: sw.active,
        elapsed_seconds: elapsed,
    }
}
