<script lang="ts">
  import { tauriInvoke } from '$lib/utils/tauri';

  interface ClipboardEntry {
    content: string;
    timestamp: string;
    content_type: string;
  }

  let entries = $state<ClipboardEntry[]>([]);
  let loading = $state(true);

  async function fetchClipboard() {
    try {
      entries = await tauriInvoke<ClipboardEntry[]>('get_clipboard_history');
    } catch (e) {
      console.error('Clipboard fetch error:', e);
    }
    loading = false;
  }

  $effect(() => {
    fetchClipboard();
    const interval = setInterval(fetchClipboard, 3000);
    return () => clearInterval(interval);
  });

  async function copyEntry(content: string) {
    try {
      await navigator.clipboard.writeText(content);
    } catch { /* fallback */ }
  }

  async function clearAll() {
    await tauriInvoke('clear_clipboard_history');
    entries = [];
  }

  function truncate(str: string, len: number): string {
    return str.length > len ? str.slice(0, len) + '...' : str;
  }

  function timeAgo(timestamp: string): string {
    const diff = Date.now() - new Date(timestamp).getTime();
    const mins = Math.floor(diff / 60000);
    if (mins < 1) return 'just now';
    if (mins < 60) return mins + 'm ago';
    return Math.floor(mins / 60) + 'h ago';
  }
</script>

<div class="clipboard-widget">
  <div class="clip-header">
    <span class="clip-title">Clipboard History</span>
    {#if entries.length > 0}
      <button class="clear-btn" onclick={clearAll} title="Clear all">🗑️</button>
    {/if}
  </div>

  <div class="clip-list">
    {#if loading}
      <div class="empty">Loading...</div>
    {:else if entries.length === 0}
      <div class="empty">
        <span class="empty-icon">📋</span>
        <span class="empty-text">No clipboard entries</span>
        <span class="empty-hint">Copy something to see it here</span>
      </div>
    {:else}
      {#each entries.slice().reverse() as entry, i}
        <button class="clip-card" onclick={() => copyEntry(entry.content)} title="Click to copy">
          <div class="clip-content">{truncate(entry.content, 60)}</div>
          <div class="clip-meta">
            <span class="clip-type">{entry.content_type}</span>
            <span class="clip-time">{timeAgo(entry.timestamp)}</span>
          </div>
        </button>
      {/each}
    {/if}
  </div>
</div>

<style>
  .clipboard-widget { width: 100%; height: 100%; display: flex; flex-direction: column; gap: 6px; }
  .clip-header { display: flex; align-items: center; justify-content: space-between; }
  .clip-title { font-size: 11px; font-weight: 600; color: rgba(255, 255, 255, 0.7); text-transform: uppercase; letter-spacing: 0.8px; }
  .clear-btn { width: 26px; height: 26px; border-radius: 50%; border: none; background: rgba(255, 255, 255, 0.06); font-size: 12px; cursor: pointer; display: flex; align-items: center; justify-content: center; transition: all 0.2s ease; padding: 0; }
  .clear-btn:hover { background: rgba(255, 45, 149, 0.2); transform: scale(1.1); }
  .clip-list { flex: 1; overflow-y: auto; display: flex; flex-direction: column; gap: 4px; }
  .clip-list::-webkit-scrollbar { width: 3px; }
  .clip-list::-webkit-scrollbar-thumb { background: rgba(0, 240, 255, 0.3); border-radius: 2px; }
  .empty { display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; gap: 4px; color: rgba(255, 255, 255, 0.35); font-size: 12px; }
  .empty-icon { font-size: 24px; }
  .empty-hint { font-size: 10px; opacity: 0.6; }
  .clip-card { display: flex; flex-direction: column; gap: 3px; padding: 8px 10px; border-radius: 8px; background: rgba(255, 255, 255, 0.04); border: 1px solid rgba(255, 255, 255, 0.04); cursor: pointer; transition: all 0.2s ease; text-align: left; width: 100%; color: inherit; }
  .clip-card:hover { background: rgba(0, 240, 255, 0.08); border-color: rgba(0, 240, 255, 0.15); box-shadow: 0 0 8px rgba(0, 240, 255, 0.1); }
  .clip-content { font-size: 11px; color: rgba(255, 255, 255, 0.8); line-height: 1.3; word-break: break-all; }
  .clip-meta { display: flex; justify-content: space-between; align-items: center; }
  .clip-type { font-size: 8px; color: rgba(0, 240, 255, 0.5); text-transform: uppercase; letter-spacing: 0.5px; }
  .clip-time { font-size: 8px; color: rgba(255, 255, 255, 0.3); }
</style>
