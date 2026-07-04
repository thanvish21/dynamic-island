<script lang="ts">
  import { tauriInvoke } from '$lib/utils/tauri';

  interface Notification {
    id: number;
    app: string;
    title: string;
    body: string;
    timestamp: string;
    read: boolean;
    icon: string | null;
  }

  let notifications = $state<Notification[]>([]);
  let dndEnabled = $state(false);
  let loading = $state(true);

  async function fetchNotifications() {
    try {
      const [notifs, dnd] = await Promise.all([
        tauriInvoke<Notification[]>('get_notifications'),
        tauriInvoke<boolean>('get_dnd_status'),
      ]);
      notifications = notifs;
      dndEnabled = dnd;
    } catch (e) {
      console.error('Notification fetch error:', e);
    }
    loading = false;
  }

  $effect(() => {
    fetchNotifications();
    const interval = setInterval(fetchNotifications, 5000);
    return () => clearInterval(interval);
  });

  async function dismiss(id: number) {
    await tauriInvoke('dismiss_notification', { id });
    notifications = notifications.filter(n => n.id !== id);
  }

  async function toggleDnd() {
    dndEnabled = await tauriInvoke<boolean>('toggle_dnd');
  }
</script>

<div class="notif-widget">
  <div class="notif-header">
    <span class="notif-title">Notifications</span>
    <button class="dnd-btn" class:active={dndEnabled} onclick={toggleDnd} title="Do Not Disturb">
      {dndEnabled ? '🌙' : '🔔'}
    </button>
  </div>

  <div class="notif-list">
    {#if loading}
      <div class="empty">Loading...</div>
    {:else if notifications.length === 0}
      <div class="empty">
        <span class="empty-icon">✨</span>
        <span class="empty-text">All clear! No notifications</span>
      </div>
    {:else}
      {#each notifications as notif (notif.id)}
        <div class="notif-card">
          <div class="notif-icon">{notif.icon || '📩'}</div>
          <div class="notif-content">
            <div class="notif-app">{notif.app}</div>
            <div class="notif-msg-title">{notif.title}</div>
            <div class="notif-body">{notif.body}</div>
          </div>
          <button class="dismiss-btn" onclick={() => dismiss(notif.id)} title="Dismiss">✕</button>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .notif-widget {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .notif-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .notif-title {
    font-size: 11px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.7);
    text-transform: uppercase;
    letter-spacing: 0.8px;
  }

  .dnd-btn {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    border: none;
    background: rgba(255, 255, 255, 0.06);
    font-size: 14px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
    padding: 0;
  }

  .dnd-btn:hover { background: rgba(255, 45, 149, 0.2); }
  .dnd-btn.active {
    background: rgba(138, 43, 226, 0.3);
    box-shadow: 0 0 10px rgba(138, 43, 226, 0.3);
  }

  .notif-list {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .notif-list::-webkit-scrollbar { width: 3px; }
  .notif-list::-webkit-scrollbar-thumb {
    background: rgba(255, 45, 149, 0.3);
    border-radius: 2px;
  }

  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 6px;
    color: rgba(255, 255, 255, 0.35);
    font-size: 12px;
  }

  .empty-icon { font-size: 24px; }

  .notif-card {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    padding: 8px;
    border-radius: 10px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.04);
    animation: slide-in 0.3s ease;
  }

  .notif-icon { font-size: 16px; flex-shrink: 0; padding-top: 1px; }

  .notif-content {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .notif-app {
    font-size: 9px;
    color: rgba(0, 240, 255, 0.6);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .notif-msg-title {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.85);
    font-weight: 600;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .notif-body {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.45);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .dismiss-btn {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    border: none;
    background: rgba(255, 255, 255, 0.06);
    color: rgba(255, 255, 255, 0.4);
    font-size: 10px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
    flex-shrink: 0;
    padding: 0;
  }

  .dismiss-btn:hover {
    background: rgba(255, 45, 149, 0.3);
    color: #fff;
  }

  @keyframes slide-in {
    from { opacity: 0; transform: translateX(-8px); }
    to { opacity: 1; transform: translateX(0); }
  }
</style>
