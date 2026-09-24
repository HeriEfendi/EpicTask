import { defineStore } from 'pinia';
import { ref } from 'vue';
import { api } from '@/services/api';
import { wsClient } from '@/services/websocket';
import { sendDesktopNotification } from '@/services/tauri';

export const useNotificationStore = defineStore('notification', () => {
  const notifications = ref([]);
  const unreadCount = ref(0);
  const isOpen = ref(false);

  async function fetchNotifications() {
    try {
      const data = await api.get('/notifications');
      notifications.value = data.notifications || [];
      unreadCount.value = data.unread_count || 0;
    } catch (e) {
      console.warn('Failed to load notifications', e);
    }
  }

  async function markAsRead(id) {
    await api.put(`/notifications/${id}/read`);
    const notif = notifications.value.find((n) => n.id === id);
    if (notif && !notif.is_read) {
      notif.is_read = true;
      unreadCount.value = Math.max(0, unreadCount.value - 1);
    }
  }

  async function markAllAsRead() {
    await api.post('/notifications/read-all');
    notifications.value.forEach((n) => (n.is_read = true));
    unreadCount.value = 0;
  }

  function playNotificationChime() {
    try {
      if (localStorage.getItem('epictask_sound_enabled') === 'false') return;
      const AudioCtx = window.AudioContext || window.webkitAudioContext;
      if (!AudioCtx) return;
      const ctx = new AudioCtx();
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();

      osc.type = 'sine';
      osc.frequency.setValueAtTime(587.33, ctx.currentTime);
      osc.frequency.exponentialRampToValueAtTime(880, ctx.currentTime + 0.15);

      gain.gain.setValueAtTime(0.12, ctx.currentTime);
      gain.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + 0.35);

      osc.connect(gain);
      gain.connect(ctx.destination);

      osc.start();
      osc.stop(ctx.currentTime + 0.35);
    } catch (e) {
      // Audio might be blocked until user interacts with the page
    }
  }

  // Real-time WS notification listener
  wsClient.subscribe((msg) => {
    if (msg && msg.event === 'NOTIFICATION') {
      const title = msg.data?.title || 'EpicTask Notification';
      const body = msg.data?.message || '';
      playNotificationChime();
      sendDesktopNotification(title, body);
      fetchNotifications();
    }
  });

  return {
    notifications,
    unreadCount,
    isOpen,
    fetchNotifications,
    markAsRead,
    markAllAsRead,
  };
});
