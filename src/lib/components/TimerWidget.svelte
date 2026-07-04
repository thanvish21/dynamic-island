<script lang="ts">
  import { tauriInvoke } from '$lib/utils/tauri';

  interface TimerStatus {
    active: boolean;
    remaining_seconds: number;
    total_seconds: number;
    label: string;
  }

  interface StopwatchStatus {
    active: boolean;
    elapsed_seconds: number;
  }

  let mode = $state<'timer' | 'stopwatch'>('timer');
  let timer = $state<TimerStatus>({ active: false, remaining_seconds: 0, total_seconds: 0, label: 'Timer' });
  let stopwatch = $state<StopwatchStatus>({ active: false, elapsed_seconds: 0 });

  const presets = [
    { label: '5m', seconds: 300 },
    { label: '15m', seconds: 900 },
    { label: '25m', seconds: 1500 },
    { label: '45m', seconds: 2700 },
  ];

  $effect(() => {
    const interval = setInterval(async () => {
      if (mode === 'timer') {
        timer = await tauriInvoke<TimerStatus>('get_timer_status');
      } else {
        stopwatch = await tauriInvoke<StopwatchStatus>('get_stopwatch_status');
      }
    }, 500);
    return () => clearInterval(interval);
  });

  async function startPreset(seconds: number, label: string) {
    timer = await tauriInvoke<TimerStatus>('start_timer', { seconds, label });
  }

  async function stopTimer() {
    timer = await tauriInvoke<TimerStatus>('stop_timer');
  }

  async function startStopwatch() {
    stopwatch = await tauriInvoke<StopwatchStatus>('start_stopwatch');
  }

  async function stopStopwatch() {
    stopwatch = await tauriInvoke<StopwatchStatus>('stop_stopwatch');
  }

  function formatTime(totalSec: number): string {
    const h = Math.floor(totalSec / 3600);
    const m = Math.floor((totalSec % 3600) / 60);
    const s = Math.floor(totalSec % 60);
    if (h > 0) return `${h}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
    return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  }

  const timerProgress = $derived(
    timer.total_seconds > 0 ? (timer.remaining_seconds / timer.total_seconds) * 100 : 0
  );
</script>

<div class="timer-widget">
  <div class="mode-toggle">
    <button class="mode-btn" class:active={mode === 'timer'} onclick={() => mode = 'timer'}>⏱ Timer</button>
    <button class="mode-btn" class:active={mode === 'stopwatch'} onclick={() => mode = 'stopwatch'}>⏲ Stopwatch</button>
  </div>

  {#if mode === 'timer'}
    <div class="timer-display">
      <div class="big-time" class:active={timer.active} class:danger={timer.remaining_seconds < 10 && timer.active}>
        {formatTime(timer.remaining_seconds)}
      </div>
      {#if timer.active}
        <div class="timer-progress">
          <div class="timer-progress-fill" style="width: {timerProgress}%"></div>
        </div>
        <div class="timer-label">{timer.label}</div>
        <button class="action-btn stop" onclick={stopTimer}>⏹ Stop</button>
      {:else}
        <div class="presets">
          {#each presets as p}
            <button class="preset-btn" onclick={() => startPreset(p.seconds, p.label + ' Timer')}>
              {p.label}
            </button>
          {/each}
        </div>
      {/if}
    </div>
  {:else}
    <div class="timer-display">
      <div class="big-time" class:active={stopwatch.active}>
        {formatTime(stopwatch.elapsed_seconds)}
      </div>
      <div class="stopwatch-controls">
        {#if stopwatch.active}
          <button class="action-btn stop" onclick={stopStopwatch}>⏹ Stop</button>
        {:else}
          <button class="action-btn start" onclick={startStopwatch}>▶ Start</button>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .timer-widget {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 10px;
    align-items: center;
  }

  .mode-toggle {
    display: flex;
    gap: 4px;
    background: rgba(255, 255, 255, 0.04);
    border-radius: 10px;
    padding: 2px;
  }

  .mode-btn {
    padding: 4px 12px;
    border: none;
    border-radius: 8px;
    background: transparent;
    color: rgba(255, 255, 255, 0.5);
    font-size: 11px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .mode-btn.active {
    background: linear-gradient(135deg, rgba(255, 45, 149, 0.25), rgba(0, 240, 255, 0.15));
    color: #fff;
    box-shadow: 0 0 8px rgba(255, 45, 149, 0.2);
  }

  .timer-display {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    width: 100%;
  }

  .big-time {
    font-size: 36px;
    font-weight: 700;
    color: rgba(255, 255, 255, 0.85);
    font-variant-numeric: tabular-nums;
    letter-spacing: 2px;
    text-shadow: 0 0 20px rgba(0, 240, 255, 0.3);
    transition: all 0.3s ease;
  }

  .big-time.active {
    color: #00f0ff;
    text-shadow: 0 0 25px rgba(0, 240, 255, 0.5);
    animation: pulse-glow 1.5s ease-in-out infinite;
  }

  .big-time.danger {
    color: #ff2d95;
    text-shadow: 0 0 25px rgba(255, 45, 149, 0.5);
  }

  .timer-progress {
    width: 80%;
    height: 3px;
    background: rgba(255, 255, 255, 0.08);
    border-radius: 2px;
    overflow: hidden;
  }

  .timer-progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #ff2d95, #00f0ff);
    border-radius: 2px;
    transition: width 0.5s linear;
    box-shadow: 0 0 6px rgba(0, 240, 255, 0.4);
  }

  .timer-label {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.4);
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .presets {
    display: flex;
    gap: 6px;
  }

  .preset-btn {
    padding: 6px 14px;
    border: 1px solid rgba(255, 45, 149, 0.2);
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.04);
    color: rgba(255, 255, 255, 0.7);
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .preset-btn:hover {
    background: rgba(255, 45, 149, 0.15);
    border-color: rgba(255, 45, 149, 0.4);
    box-shadow: 0 0 10px rgba(255, 45, 149, 0.2);
    transform: scale(1.05);
  }

  .stopwatch-controls { display: flex; gap: 8px; }

  .action-btn {
    padding: 6px 18px;
    border: none;
    border-radius: 12px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .action-btn.start {
    background: linear-gradient(135deg, rgba(0, 240, 255, 0.2), rgba(0, 200, 255, 0.3));
    color: #00f0ff;
    border: 1px solid rgba(0, 240, 255, 0.3);
  }

  .action-btn.stop {
    background: linear-gradient(135deg, rgba(255, 45, 149, 0.2), rgba(255, 45, 149, 0.3));
    color: #ff2d95;
    border: 1px solid rgba(255, 45, 149, 0.3);
  }

  .action-btn:hover { transform: scale(1.05); box-shadow: 0 0 12px rgba(255, 45, 149, 0.3); }

  @keyframes pulse-glow {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.85; }
  }
</style>
