<script lang="ts">
  import { tauriInvoke } from '$lib/utils/tauri';

  interface CpuInfo { usage_percent: number; }
  interface MemoryInfo { used_gb: number; total_gb: number; usage_percent: number; }
  interface BatteryInfo { percent: number; charging: boolean; }

  let cpu = $state(32);
  let mem = $state({ used: 8.2, total: 16, percent: 51 });
  let battery = $state({ percent: 78, charging: false });
  let netUp = $state('1.2');
  let netDown = $state('5.8');
  let sparkle = $state(false);

  async function fetchSystem() {
    try {
      const cpuData = await tauriInvoke<CpuInfo>('get_cpu_usage');
      const oldCpu = cpu;
      cpu = Math.round(cpuData.usage_percent);
      if (Math.abs(cpu - oldCpu) > 5) triggerSparkle();
    } catch {}

    try {
      const memData = await tauriInvoke<MemoryInfo>('get_memory_info');
      mem = {
        used: Math.round(memData.used_gb * 10) / 10,
        total: Math.round(memData.total_gb * 10) / 10,
        percent: Math.round(memData.usage_percent),
      };
    } catch {}

    try {
      const batData = await tauriInvoke<BatteryInfo>('get_battery_info');
      battery = { percent: batData.percent, charging: batData.charging };
    } catch {}
  }

  function triggerSparkle() {
    sparkle = true;
    setTimeout(() => { sparkle = false; }, 600);
  }

  $effect(() => {
    fetchSystem();
    const interval = setInterval(fetchSystem, 2000);
    return () => clearInterval(interval);
  });

  function ringStyle(percent: number, color1: string, color2: string): string {
    const deg = (percent / 100) * 360;
    return `background: conic-gradient(${color1} 0deg, ${color2} ${deg}deg, rgba(255,255,255,0.06) ${deg}deg)`;
  }
</script>

<div class="system-widget">
  <div class="rings-row">
    <!-- CPU Ring -->
    <div class="ring-container" class:sparkle>
      <div class="ring" style={ringStyle(cpu, '#ff2d95', '#ff6eb4')}>
        <div class="ring-inner">
          <span class="ring-value">{cpu}</span>
          <span class="ring-unit">%</span>
        </div>
      </div>
      <span class="ring-label">CPU</span>
    </div>

    <!-- RAM Ring -->
    <div class="ring-container">
      <div class="ring" style={ringStyle(mem.percent, '#00f0ff', '#7b68ee')}>
        <div class="ring-inner">
          <span class="ring-value">{mem.percent}</span>
          <span class="ring-unit">%</span>
        </div>
      </div>
      <span class="ring-label">RAM</span>
    </div>

    <!-- Battery -->
    <div class="ring-container">
      <div
        class="ring"
        style={ringStyle(
          battery.percent,
          battery.charging ? '#00ff88' : battery.percent < 20 ? '#ff4444' : '#ffb7c5',
          battery.charging ? '#00cc66' : battery.percent < 20 ? '#ff6666' : '#ff8fab'
        )}
      >
        <div class="ring-inner">
          <span class="ring-value">{battery.percent}</span>
          <span class="ring-unit">{battery.charging ? '⚡' : '%'}</span>
        </div>
      </div>
      <span class="ring-label">BAT</span>
    </div>
  </div>

  <div class="stats-row">
    <div class="stat-item">
      <span class="stat-icon">↓</span>
      <span class="stat-value">{netDown}</span>
      <span class="stat-unit">MB/s</span>
    </div>
    <div class="stat-divider"></div>
    <div class="stat-item">
      <span class="stat-icon">↑</span>
      <span class="stat-value">{netUp}</span>
      <span class="stat-unit">MB/s</span>
    </div>
    <div class="stat-divider"></div>
    <div class="stat-item">
      <span class="stat-icon mem-icon">⬢</span>
      <span class="stat-value">{mem.used}</span>
      <span class="stat-unit">/ {mem.total} GB</span>
    </div>
  </div>
</div>

<style>
  .system-widget {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 14px;
    height: 100%;
    justify-content: center;
    animation: fade-scale 0.3s ease;
  }

  .rings-row {
    display: flex;
    align-items: center;
    gap: 20px;
  }

  .ring-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    position: relative;
  }

  .ring-container.sparkle::after {
    content: '✦';
    position: absolute;
    top: -4px;
    right: -4px;
    font-size: 14px;
    color: #ff2d95;
    animation: sparkle-pop 0.6s ease forwards;
    text-shadow: 0 0 8px rgba(255, 45, 149, 0.8);
  }

  @keyframes sparkle-pop {
    0% { transform: scale(0) rotate(0deg); opacity: 0; }
    50% { transform: scale(1.3) rotate(180deg); opacity: 1; }
    100% { transform: scale(0.8) rotate(360deg); opacity: 0; }
  }

  .ring {
    width: 72px;
    height: 72px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    transition: all 0.5s ease;
    box-shadow: 0 0 15px rgba(255, 45, 149, 0.15);
  }

  .ring-inner {
    width: 56px;
    height: 56px;
    border-radius: 50%;
    background: rgba(10, 10, 26, 0.9);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1px;
  }

  .ring-value {
    font-size: 18px;
    font-weight: 700;
    color: #fff;
    text-shadow: 0 0 8px rgba(0, 240, 255, 0.4);
  }

  .ring-unit {
    font-size: 9px;
    color: rgba(255, 255, 255, 0.5);
    margin-top: 4px;
  }

  .ring-label {
    font-size: 9px;
    font-weight: 600;
    color: rgba(255, 183, 197, 0.7);
    letter-spacing: 1px;
    text-transform: uppercase;
  }

  .stats-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 16px;
    background: rgba(255, 255, 255, 0.04);
    border-radius: 14px;
    border: 1px solid rgba(255, 255, 255, 0.05);
  }

  .stat-item {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .stat-icon {
    font-size: 11px;
    color: #00f0ff;
    text-shadow: 0 0 6px rgba(0, 240, 255, 0.5);
  }

  .mem-icon {
    color: #ff2d95;
    text-shadow: 0 0 6px rgba(255, 45, 149, 0.5);
  }

  .stat-value {
    font-size: 12px;
    font-weight: 600;
    color: #fff;
  }

  .stat-unit {
    font-size: 9px;
    color: rgba(255, 255, 255, 0.4);
  }

  .stat-divider {
    width: 1px;
    height: 14px;
    background: rgba(255, 255, 255, 0.1);
  }

  @keyframes fade-scale {
    from { opacity: 0; transform: scale(0.95); }
    to { opacity: 1; transform: scale(1); }
  }
</style>
