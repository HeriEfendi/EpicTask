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

  // Real-time WS notification listener
  wsClient.subscribe((msg) => {
    if (msg && msg.event === 'NOTIFICATION') {
      const title = msg.data?.title || 'EpicTask Notification';
      const body = msg.data?.message || '';
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
