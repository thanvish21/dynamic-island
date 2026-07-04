use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct WeatherData {
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: f64,
    pub description: String,
    pub icon: String,
    pub city: String,
    pub wind_speed: f64,
    pub condition: String,
}

#[derive(Deserialize)]
struct OpenMeteoResponse {
    current: Option<OpenMeteoCurrent>,
}

#[derive(Deserialize)]
struct OpenMeteoCurrent {
    temperature_2m: Option<f64>,
    relative_humidity_2m: Option<f64>,
    apparent_temperature: Option<f64>,
    wind_speed_10m: Option<f64>,
    weather_code: Option<i32>,
}

fn weather_code_to_description(code: i32) -> (&'static str, &'static str) {
    match code {
        0 => ("Clear sky", "☀️"),
        1 | 2 | 3 => ("Partly cloudy", "⛅"),
        45 | 48 => ("Foggy", "🌫️"),
        51 | 53 | 55 => ("Drizzle", "🌦️"),
        61 | 63 | 65 => ("Rain", "🌧️"),
        71 | 73 | 75 => ("Snow", "❄️"),
        80 | 81 | 82 => ("Rain showers", "🌧️"),
        95 => ("Thunderstorm", "⛈️"),
        96 | 99 => ("Thunderstorm with hail", "⛈️"),
        _ => ("Unknown", "🌡️"),
    }
}

#[tauri::command]
pub async fn get_weather(lat: Option<f64>, lon: Option<f64>) -> Result<WeatherData, String> {
    let latitude = lat.unwrap_or(28.6139); // Default: New Delhi
    let longitude = lon.unwrap_or(77.2090);

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,wind_speed_10m,weather_code",
        latitude, longitude
    );

    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch weather: {}", e))?;

    let data: OpenMeteoResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse weather: {}", e))?;

    let current = data.current.ok_or("No current weather data")?;
    let code = current.weather_code.unwrap_or(0);
    let (description, icon) = weather_code_to_description(code);

    Ok(WeatherData {
        temperature: current.temperature_2m.unwrap_or(0.0),
        feels_like: current.apparent_temperature.unwrap_or(0.0),
        humidity: current.relative_humidity_2m.unwrap_or(0.0),
        description: description.to_string(),
        icon: icon.to_string(),
        city: "Current Location".to_string(),
        wind_speed: current.wind_speed_10m.unwrap_or(0.0),
        condition: description.to_string(),
    })
}
