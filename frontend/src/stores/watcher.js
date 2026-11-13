import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { api } from '@/services/api';
import { wsClient } from '@/services/websocket';
import { useAuthStore } from '@/stores/auth';
import { useProjectStore } from '@/stores/project';
import { toast } from '@/utils/toast';

export const useWatcherStore = defineStore('watcher', () => {
  const authStore = useAuthStore();
  const projectStore = useProjectStore();
  const watchers = ref([]);
  const loading = ref(false);
  const currentIssueId = ref(null);

  const isCurrentUserWatching = computed(() => {
    const myId = authStore.user?.id;
    if (!myId) return false;
    return watchers.value.some((w) => w.user_id === myId);
  });

  const watcherCount = computed(() => watchers.value.length);

  async function fetchWatchers(issueId) {
    if (!issueId) {
      watchers.value = [];
      currentIssueId.value = null;
      return [];
    }
    currentIssueId.value = issueId;
    loading.value = true;
    try {
      const data = await api.get(`/issues/${issueId}/watchers`);
      watchers.value = data || [];
      return watchers.value;
    } catch (e) {
      console.error('Failed to fetch watchers', e);
      return [];
    } finally {
      loading.value = false;
    }
  }

  async function addWatcher(issueId, userId = null) {
    try {
      const payload = userId ? { user_id: userId } : {};
      const updated = await api.post(`/issues/${issueId}/watchers`, payload);
      watchers.value = updated || [];
      const issue = projectStore.issues.find((i) => i.id === issueId);
      if (issue) {
        issue.watcher_count = watchers.value.length;
      }
      return watchers.value;
    } catch (e) {
      toast.error(e.message || 'Gagal menambahkan pengamat');
      throw e;
    }
  }

  async function removeWatcher(issueId, userId) {
    try {
      const updated = await api.delete(`/issues/${issueId}/watchers/${userId}`);
      watchers.value = updated || [];
      const issue = projectStore.issues.find((i) => i.id === issueId);
      if (issue) {
        issue.watcher_count = watchers.value.length;
      }
      return watchers.value;
    } catch (e) {
      toast.error(e.message || 'Gagal menghapus pengamat');
      throw e;
    }
  }

  async function toggleWatch(issueId) {
    const myId = authStore.user?.id;
    if (!myId) return;
    if (isCurrentUserWatching.value) {
      await removeWatcher(issueId, myId);
      toast.info('Anda berhenti mengamati tiket ini');
    } else {
      await addWatcher(issueId, myId);
      toast.success('Anda sekarang mengamati tiket ini');
    }
  }

  // WebSocket sync
  wsClient.subscribe((msg) => {
    if (msg.event === 'WATCHER_ADDED' || msg.event === 'WATCHER_REMOVED') {
      if (msg.data && msg.data.issue_id === currentIssueId.value) {
        if (msg.data.watchers) {
          watchers.value = msg.data.watchers;
        }
      }
      if (msg.data && msg.data.issue_id) {
        const issue = projectStore.issues.find((i) => i.id === msg.data.issue_id);
        if (issue && msg.data.watchers) {
          issue.watcher_count = msg.data.watchers.length;
        }
      }
    }
  });

  return {
    watchers,
    loading,
    currentIssueId,
    isCurrentUserWatching,
    watcherCount,
    fetchWatchers,
    addWatcher,
    removeWatcher,
    toggleWatch,
  };
});

