<script lang="ts">
  import { tauriInvoke } from '$lib/utils/tauri';

  interface MediaStatus {
    title: string;
    artist: string;
    album: string;
    is_playing: boolean;
    position_seconds: number;
    duration_seconds: number;
    album_art_url: string;
  }

  let media: MediaStatus = $state({
    title: 'Sparkle',
    artist: 'RADWIMPS',
    album: 'Your Name OST',
    is_playing: true,
    position_seconds: 87,
    duration_seconds: 253,
    album_art_url: '',
  });

  let progressPercent = $derived(
    media.duration_seconds > 0
      ? (media.position_seconds / media.duration_seconds) * 100
      : 0
  );

  let positionStr = $derived(formatTime(media.position_seconds));
  let durationStr = $derived(formatTime(media.duration_seconds));

  function formatTime(s: number): string {
    const m = Math.floor(s / 60);
    const sec = Math.floor(s % 60);
    return `${m}:${sec.toString().padStart(2, '0')}`;
  }

  async function fetchMedia() {
    try {
      media = await tauriInvoke<MediaStatus>('get_media_status');
    } catch {
      // keep demo data
    }
  }

  async function playPause() {
    try {
      await tauriInvoke('media_play_pause');
      media.is_playing = !media.is_playing;
    } catch {
      media.is_playing = !media.is_playing;
    }
  }

  async function next() {
    try { await tauriInvoke('media_next'); } catch {}
    await fetchMedia();
  }

  async function prev() {
    try { await tauriInvoke('media_previous'); } catch {}
    await fetchMedia();
  }

  $effect(() => {
    fetchMedia();
    const interval = setInterval(fetchMedia, 3000);
    return () => clearInterval(interval);
  });

  // Progress ticker when playing
  $effect(() => {
    if (!media.is_playing) return;
    const interval = setInterval(() => {
      if (media.position_seconds < media.duration_seconds) {
        media.position_seconds += 1;
      }
    }, 1000);
    return () => clearInterval(interval);
  });
</script>

<div class="media-widget">
  <div class="album-section">
    <div class="album-art" class:spinning={media.is_playing}>
      <div class="album-inner">
        {#if media.album_art_url}
          <img src={media.album_art_url} alt="Album art" />
        {:else}
          <div class="album-placeholder">
            <span class="note-icon">♪</span>
          </div>
        {/if}
        <div class="album-ring"></div>
        <div class="album-dot"></div>
      </div>
    </div>
  </div>

  <div class="info-section">
    <div class="track-info">
      <div class="track-title">{media.title}</div>
      <div class="track-artist">{media.artist}</div>
    </div>

    <div class="progress-section">
      <div class="progress-bar">
        <div class="progress-fill" style="width: {progressPercent}%"></div>
        <div class="progress-glow" style="left: {progressPercent}%"></div>
      </div>
      <div class="progress-times">
        <span>{positionStr}</span>
        <span>{durationStr}</span>
      </div>
    </div>

    <div class="controls">
      <button class="ctrl-btn" onclick={prev} title="Previous">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
          <path d="M6 6h2v12H6zm3.5 6l8.5 6V6z"/>
        </svg>
      </button>
      <button class="ctrl-btn play-btn" onclick={playPause} title={media.is_playing ? 'Pause' : 'Play'}>
        {#if media.is_playing}
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/>
          </svg>
        {:else}
          <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
            <path d="M8 5v14l11-7z"/>
          </svg>
        {/if}
      </button>
      <button class="ctrl-btn" onclick={next} title="Next">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
          <path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z"/>
        </svg>
      </button>
    </div>
  </div>
</div>

<style>
  .media-widget {
    display: flex;
    align-items: center;
    gap: 16px;
    height: 100%;
    animation: fade-scale 0.3s ease;
  }

  .album-section {
    flex-shrink: 0;
  }

  .album-art {
    width: 100px;
    height: 100px;
    border-radius: 50%;
    position: relative;
  }

  .album-art.spinning {
    animation: spin-slow 4s linear infinite;
  }

  .album-inner {
    width: 100%;
    height: 100%;
    border-radius: 50%;
    overflow: hidden;
    position: relative;
    background: linear-gradient(135deg, #1a0a2e, #2d1b4e);
    border: 2px solid rgba(255, 45, 149, 0.3);
    box-shadow: 0 0 20px rgba(255, 45, 149, 0.2), 0 0 40px rgba(0, 240, 255, 0.1);
  }

  .album-inner img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .album-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #2d1b4e, #1a0a2e);
  }

  .note-icon {
    font-size: 32px;
    color: #ff2d95;
    text-shadow: 0 0 15px rgba(255, 45, 149, 0.6);
  }

  .album-ring {
    position: absolute;
    inset: 30%;
    border-radius: 50%;
    border: 2px solid rgba(0, 240, 255, 0.15);
  }

  .album-dot {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.6);
  }

  @keyframes spin-slow {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .info-section {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .track-info {
    min-width: 0;
  }

  .track-title {
    font-size: 14px;
    font-weight: 700;
    color: #fff;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    text-shadow: 0 0 10px rgba(255, 45, 149, 0.3);
  }

  .track-artist {
    font-size: 11px;
    color: rgba(255, 183, 197, 0.8);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-top: 2px;
  }

  .progress-section {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .progress-bar {
    width: 100%;
    height: 3px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 2px;
    position: relative;
    overflow: visible;
  }

  .progress-fill {
    height: 100%;
    border-radius: 2px;
    background: linear-gradient(90deg, #ff2d95, #00f0ff);
    transition: width 0.3s linear;
    box-shadow: 0 0 6px rgba(255, 45, 149, 0.5);
  }

  .progress-glow {
    position: absolute;
    top: 50%;
    transform: translate(-50%, -50%);
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #00f0ff;
    box-shadow: 0 0 8px rgba(0, 240, 255, 0.8);
    transition: left 0.3s linear;
  }

  .progress-times {
    display: flex;
    justify-content: space-between;
    font-size: 9px;
    color: rgba(255, 255, 255, 0.4);
  }

  .controls {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
  }

  .ctrl-btn {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    border: none;
    background: rgba(255, 255, 255, 0.06);
    color: rgba(255, 255, 255, 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s ease;
    padding: 0;
  }

  .ctrl-btn:hover {
    background: rgba(255, 45, 149, 0.2);
    color: #fff;
    transform: scale(1.1);
    box-shadow: 0 0 12px rgba(255, 45, 149, 0.3);
  }

  .play-btn {
    width: 40px;
    height: 40px;
    background: linear-gradient(135deg, rgba(255, 45, 149, 0.3), rgba(0, 240, 255, 0.2));
    border: 1px solid rgba(255, 45, 149, 0.3);
  }

  .play-btn:hover {
    background: linear-gradient(135deg, rgba(255, 45, 149, 0.5), rgba(0, 240, 255, 0.35));
    box-shadow: 0 0 20px rgba(255, 45, 149, 0.4);
  }

  @keyframes fade-scale {
    from { opacity: 0; transform: scale(0.95); }
    to { opacity: 1; transform: scale(1); }
  }
</style>
