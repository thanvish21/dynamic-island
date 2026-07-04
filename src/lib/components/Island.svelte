<script lang="ts">
  import { activeWidget, islandState, isHovered, type WidgetType } from '$lib/stores/island';
  import Particles from './Particles.svelte';
  import MediaWidget from './MediaWidget.svelte';
  import SystemWidget from './SystemWidget.svelte';
  import WeatherWidget from './WeatherWidget.svelte';
  import NotificationWidget from './NotificationWidget.svelte';
  import TimerWidget from './TimerWidget.svelte';
  import ClipboardWidget from './ClipboardWidget.svelte';

  let currentWidget: WidgetType = $state('system');
  let currentState = $state<'compact' | 'expanded' | 'wide'>('compact');
  let hovered = $state(false);
  let now = $state(new Date());
  let glowIntensity = $state(0);
  let expandTimeout: ReturnType<typeof setTimeout> | undefined;

  // Sync stores
  $effect(() => { activeWidget.set(currentWidget); });
  $effect(() => { islandState.set(currentState); });
  $effect(() => { isHovered.set(hovered); });

  // Clock
  $effect(() => {
    const interval = setInterval(() => { now = new Date(); }, 1000);
    return () => clearInterval(interval);
  });

  // Glow pulse
  $effect(() => {
    let frame: number;
    let t = 0;
    function pulse() {
      t += 0.03;
      glowIntensity = (Math.sin(t) + 1) / 2;
      frame = requestAnimationFrame(pulse);
    }
    pulse();
    return () => cancelAnimationFrame(frame);
  });

  const timeStr = $derived(
    now.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', hour12: true })
  );

  const widgets: { type: WidgetType; icon: string; label: string }[] = [
    { type: 'system', icon: '⚡', label: 'System' },
    { type: 'media', icon: '🎵', label: 'Media' },
    { type: 'weather', icon: '🌸', label: 'Weather' },
    { type: 'notifications', icon: '🔔', label: 'Alerts' },
    { type: 'timer', icon: '⏱', label: 'Timer' },
    { type: 'clipboard', icon: '📋', label: 'Clips' },
  ];

  function expand(widget?: WidgetType) {
    if (widget) currentWidget = widget;
    currentState = currentWidget === 'media' ? 'wide' : 'expanded';
  }

  function collapse() {
    currentState = 'compact';
  }

  function toggleExpand() {
    if (currentState === 'compact') {
      expand();
    } else {
      collapse();
    }
  }

  function selectWidget(w: WidgetType) {
    currentWidget = w;
    currentState = w === 'media' ? 'wide' : 'expanded';
  }

  function handleMouseEnter() {
    hovered = true;
    clearTimeout(expandTimeout);
  }

  function handleMouseLeave() {
    hovered = false;
    if (currentState !== 'compact') {
      expandTimeout = setTimeout(collapse, 2000);
    }
  }

  const islandWidth = $derived(
    currentState === 'compact' ? 380 : currentState === 'wide' ? 420 : 380
  );
  const islandHeight = $derived(
    currentState === 'compact' ? 48 : currentState === 'wide' ? 200 : 300
  );

  const glowColor = $derived(
    `rgba(255, 45, 149, ${0.3 + glowIntensity * 0.25})`
  );
  const glowColor2 = $derived(
    `rgba(0, 240, 255, ${0.2 + glowIntensity * 0.2})`
  );
</script>

<div
  class="island-wrapper"
  style="width: {islandWidth}px; height: {islandHeight}px;"
>
  <div
    class="island"
    class:compact={currentState === 'compact'}
    class:expanded={currentState !== 'compact'}
    class:hovered
    role="button"
    tabindex="0"
    onmouseenter={handleMouseEnter}
    onmouseleave={handleMouseLeave}
    onclick={currentState === 'compact' ? () => toggleExpand() : undefined}
    onkeydown={(e) => e.key === 'Enter' && toggleExpand()}
    style="
      --glow-1: {glowColor};
      --glow-2: {glowColor2};
      box-shadow:
        0 0 15px {glowColor},
        0 0 30px {glowColor2},
        inset 0 0 20px rgba(255, 45, 149, 0.05);
    "
  >
    <Particles />

    <div class="island-content">
      {#if currentState === 'compact'}
        <!-- Compact: time + status dots -->
        <div class="compact-content">
          <div class="compact-left">
            <span class="time-display">{timeStr}</span>
          </div>
          <div class="compact-right">
            {#each widgets as w}
              <button
                class="status-dot"
                class:active={currentWidget === w.type}
                title={w.label}
                onclick={(e: MouseEvent) => { e.stopPropagation(); expand(w.type); }}
              >
                <span class="dot-icon">{w.icon}</span>
              </button>
            {/each}
          </div>
        </div>
      {:else}
        <!-- Expanded: tab bar + widget -->
        <div class="expanded-content">
          <div class="tab-bar">
            {#each widgets as w}
              <button
                class="tab-btn"
                class:active={currentWidget === w.type}
                onclick={() => selectWidget(w.type)}
              >
                <span class="tab-icon">{w.icon}</span>
                <span class="tab-label">{w.label}</span>
              </button>
            {/each}
            <button class="collapse-btn" onclick={collapse} title="Collapse">
              <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
                <path d="M3 5L7 9L11 5" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              </svg>
            </button>
          </div>

          <div class="widget-area">
            {#if currentWidget === 'media'}
              <MediaWidget />
            {:else if currentWidget === 'system'}
              <SystemWidget />
            {:else if currentWidget === 'weather'}
              <WeatherWidget />
            {:else if currentWidget === 'notifications'}
              <NotificationWidget />
            {:else if currentWidget === 'timer'}
              <TimerWidget />
            {:else if currentWidget === 'clipboard'}
              <ClipboardWidget />
            {/if}
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .island-wrapper {
    position: fixed;
    top: 8px;
    left: 50%;
    transform: translateX(-50%);
    transition:
      width 0.4s cubic-bezier(0.34, 1.56, 0.64, 1),
      height 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
    z-index: 9999;
  }

  .island {
    position: relative;
    width: 100%;
    height: 100%;
    border-radius: 26px;
    background: linear-gradient(
      135deg,
      rgba(26, 10, 46, 0.88) 0%,
      rgba(10, 10, 26, 0.92) 50%,
      rgba(26, 10, 46, 0.88) 100%
    );
    backdrop-filter: blur(24px) saturate(1.8);
    -webkit-backdrop-filter: blur(24px) saturate(1.8);
    border: 1px solid rgba(255, 45, 149, 0.2);
    overflow: hidden;
    cursor: default;
    transition:
      box-shadow 0.3s ease,
      border-color 0.3s ease;
    animation: glow-pulse 3s ease-in-out infinite;
  }

  .island.hovered {
    border-color: rgba(255, 45, 149, 0.45);
  }

  .island.compact {
    cursor: pointer;
  }

  @keyframes glow-pulse {
    0%, 100% {
      border-color: rgba(255, 45, 149, 0.2);
    }
    50% {
      border-color: rgba(0, 240, 255, 0.35);
    }
  }

  .island-content {
    position: relative;
    z-index: 1;
    width: 100%;
    height: 100%;
  }

  /* Compact state */
  .compact-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 100%;
    padding: 0 16px;
    animation: fade-scale 0.3s ease;
  }

  .compact-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .time-display {
    font-size: 14px;
    font-weight: 600;
    color: #fff;
    letter-spacing: 0.5px;
    text-shadow: 0 0 10px rgba(0, 240, 255, 0.5);
  }

  .compact-right {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .status-dot {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    border: none;
    background: rgba(255, 255, 255, 0.06);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s ease;
    padding: 0;
  }

  .status-dot:hover {
    background: rgba(255, 45, 149, 0.2);
    transform: scale(1.15);
  }

  .status-dot.active {
    background: rgba(255, 45, 149, 0.25);
    box-shadow: 0 0 8px rgba(255, 45, 149, 0.4);
  }

  .dot-icon {
    font-size: 12px;
    line-height: 1;
  }

  /* Expanded state */
  .expanded-content {
    display: flex;
    flex-direction: column;
    height: 100%;
    animation: fade-scale 0.35s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .tab-bar {
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 6px 8px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    flex-shrink: 0;
  }

  .tab-btn {
    display: flex;
    align-items: center;
    gap: 3px;
    padding: 4px 8px;
    border: none;
    border-radius: 12px;
    background: transparent;
    color: rgba(255, 255, 255, 0.5);
    font-size: 11px;
    cursor: pointer;
    transition: all 0.2s ease;
    white-space: nowrap;
  }

  .tab-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    color: rgba(255, 255, 255, 0.8);
  }

  .tab-btn.active {
    background: linear-gradient(135deg, rgba(255, 45, 149, 0.25), rgba(0, 240, 255, 0.15));
    color: #fff;
    box-shadow: 0 0 10px rgba(255, 45, 149, 0.2);
  }

  .tab-icon {
    font-size: 12px;
    line-height: 1;
  }

  .tab-label {
    font-size: 10px;
    font-weight: 500;
    letter-spacing: 0.3px;
  }

  .collapse-btn {
    margin-left: auto;
    width: 24px;
    height: 24px;
    border-radius: 50%;
    border: none;
    background: rgba(255, 255, 255, 0.08);
    color: rgba(255, 255, 255, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s ease;
    padding: 0;
  }

  .collapse-btn:hover {
    background: rgba(255, 45, 149, 0.2);
    color: #fff;
  }

  .widget-area {
    flex: 1;
    overflow: hidden;
    padding: 8px 12px 10px;
    min-height: 0;
  }

  @keyframes fade-scale {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }
</style>
