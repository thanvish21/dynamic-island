<script lang="ts">
  import { tauriInvoke } from '$lib/utils/tauri';

  interface WeatherData {
    temperature: number;
    feels_like: number;
    humidity: number;
    wind_speed: number;
    condition: string;
    location: string;
  }

  let weather: WeatherData = $state({
    temperature: 28,
    feels_like: 31,
    humidity: 65,
    wind_speed: 12,
    condition: 'partly_cloudy',
    location: 'Hyderabad',
  });

  const conditionIcons: Record<string, string> = {
    clear: '☀️',
    partly_cloudy: '⛅',
    cloudy: '☁️',
    rain: '🌧️',
    thunderstorm: '⛈️',
    snow: '❄️',
    mist: '🌫️',
    wind: '💨',
  };

  const conditionLabels: Record<string, string> = {
    clear: 'Clear Sky',
    partly_cloudy: 'Partly Cloudy',
    cloudy: 'Overcast',
    rain: 'Rainy',
    thunderstorm: 'Thunderstorm',
    snow: 'Snowing',
    mist: 'Misty',
    wind: 'Windy',
  };

  let icon = $derived(conditionIcons[weather.condition] || '🌤️');
  let label = $derived(conditionLabels[weather.condition] || 'Unknown');

  // Weather particle effects
  let showRain = $derived(weather.condition === 'rain' || weather.condition === 'thunderstorm');
  let showSnow = $derived(weather.condition === 'snow');
  let showSun = $derived(weather.condition === 'clear');

  async function fetchWeather() {
    try {
      weather = await tauriInvoke<WeatherData>('get_weather');
    } catch {}
  }

  $effect(() => {
    fetchWeather();
    const interval = setInterval(fetchWeather, 60000);
    return () => clearInterval(interval);
  });
</script>

<div class="weather-widget">
  <!-- Weather effects overlay -->
  {#if showRain}
    <div class="weather-fx rain-fx">
      {#each Array(12) as _, i}
        <div class="raindrop" style="left: {8 + i * 8}%; animation-delay: {i * 0.15}s; animation-duration: {0.6 + Math.random() * 0.4}s"></div>
      {/each}
    </div>
  {/if}
  {#if showSnow}
    <div class="weather-fx snow-fx">
      {#each Array(10) as _, i}
        <div class="snowflake" style="left: {5 + i * 10}%; animation-delay: {i * 0.3}s;">✧</div>
      {/each}
    </div>
  {/if}
  {#if showSun}
    <div class="sun-rays"></div>
  {/if}

  <div class="weather-main">
    <div class="weather-left">
      <div class="weather-icon">{icon}</div>
      <div class="weather-condition">{label}</div>
      <div class="weather-location">📍 {weather.location}</div>
    </div>

    <div class="weather-right">
      <div class="temp-display">
        <span class="temp-value">{weather.temperature}</span>
        <span class="temp-unit">°C</span>
      </div>
    </div>
  </div>

  <div class="weather-details">
    <div class="detail-item">
      <span class="detail-icon">🌡️</span>
      <span class="detail-label">Feels</span>
      <span class="detail-value">{weather.feels_like}°</span>
    </div>
    <div class="detail-divider"></div>
    <div class="detail-item">
      <span class="detail-icon">💧</span>
      <span class="detail-label">Humid</span>
      <span class="detail-value">{weather.humidity}%</span>
    </div>
    <div class="detail-divider"></div>
    <div class="detail-item">
      <span class="detail-icon">💨</span>
      <span class="detail-label">Wind</span>
      <span class="detail-value">{weather.wind_speed} km/h</span>
    </div>
  </div>
</div>

<style>
  .weather-widget {
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: 12px;
    position: relative;
    overflow: hidden;
    animation: fade-scale 0.3s ease;
  }

  /* Weather effects */
  .weather-fx {
    position: absolute;
    inset: 0;
    pointer-events: none;
    z-index: 0;
    overflow: hidden;
  }

  .raindrop {
    position: absolute;
    top: -10px;
    width: 2px;
    height: 12px;
    background: linear-gradient(to bottom, transparent, rgba(0, 240, 255, 0.5));
    border-radius: 1px;
    animation: rain-fall 0.8s linear infinite;
  }

  @keyframes rain-fall {
    to { transform: translateY(280px); opacity: 0; }
  }

  .snowflake {
    position: absolute;
    top: -10px;
    font-size: 10px;
    color: rgba(200, 220, 255, 0.6);
    animation: snow-fall 3s ease-in infinite;
  }

  @keyframes snow-fall {
    to {
      transform: translateY(280px) rotate(360deg);
      opacity: 0;
    }
  }

  .sun-rays {
    position: absolute;
    top: -20px;
    right: -20px;
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background: radial-gradient(circle, rgba(255, 200, 50, 0.15) 0%, transparent 70%);
    animation: sun-pulse 3s ease-in-out infinite;
    pointer-events: none;
  }

  @keyframes sun-pulse {
    0%, 100% { transform: scale(1); opacity: 0.6; }
    50% { transform: scale(1.2); opacity: 1; }
  }

  .weather-main {
    display: flex;
    align-items: center;
    justify-content: space-between;
    position: relative;
    z-index: 1;
  }

  .weather-left {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .weather-icon {
    font-size: 36px;
    line-height: 1;
    filter: drop-shadow(0 0 8px rgba(255, 183, 197, 0.4));
  }

  .weather-condition {
    font-size: 13px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.9);
    margin-top: 4px;
  }

  .weather-location {
    font-size: 10px;
    color: rgba(255, 183, 197, 0.6);
  }

  .weather-right {
    display: flex;
    align-items: flex-start;
  }

  .temp-display {
    display: flex;
    align-items: flex-start;
  }

  .temp-value {
    font-size: 48px;
    font-weight: 800;
    color: #fff;
    line-height: 1;
    text-shadow: 0 0 20px rgba(0, 240, 255, 0.3), 0 0 40px rgba(255, 45, 149, 0.15);
  }

  .temp-unit {
    font-size: 18px;
    font-weight: 300;
    color: rgba(0, 240, 255, 0.7);
    margin-top: 4px;
  }

  .weather-details {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 10px 14px;
    background: rgba(255, 255, 255, 0.04);
    border-radius: 14px;
    border: 1px solid rgba(255, 255, 255, 0.05);
    position: relative;
    z-index: 1;
  }

  .detail-item {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .detail-icon {
    font-size: 12px;
    line-height: 1;
  }

  .detail-label {
    font-size: 9px;
    color: rgba(255, 255, 255, 0.4);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .detail-value {
    font-size: 12px;
    font-weight: 600;
    color: #fff;
  }

  .detail-divider {
    width: 1px;
    height: 16px;
    background: rgba(255, 255, 255, 0.1);
  }

  @keyframes fade-scale {
    from { opacity: 0; transform: scale(0.95); }
    to { opacity: 1; transform: scale(1); }
  }
</style>
