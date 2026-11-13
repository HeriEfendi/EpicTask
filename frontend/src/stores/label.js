import { defineStore } from 'pinia';
import { ref } from 'vue';
import { api } from '@/services/api';
import { wsClient } from '@/services/websocket';
import { toast } from '@/utils/toast';

export const useLabelStore = defineStore('label', () => {
  const labels = ref([]);
  const loading = ref(false);
  const error = ref(null);
  let currentProjectId = null;

  async function fetchLabels(projectId) {
    if (!projectId) return;
    currentProjectId = projectId;
    loading.value = true;
    error.value = null;
    try {
      const data = await api.get(`/projects/${projectId}/labels`);
      labels.value = data || [];
    } catch (err) {
      error.value = err.message;
      toast.error(err.message || 'Gagal memuat label proyek');
    } finally {
      loading.value = false;
    }
  }

  async function createLabel(projectId, payload) {
    try {
      const created = await api.post(`/projects/${projectId}/labels`, payload);
      const existingIdx = labels.value.findIndex(
        (l) => l.name.toLowerCase() === created.name.toLowerCase()
      );
      if (existingIdx === -1) {
        labels.value.push(created);
      } else {
        labels.value[existingIdx] = created;
      }
      return created;
    } catch (err) {
      toast.error(err.message || 'Gagal membuat label');
      throw err;
    }
  }

  async function deleteLabel(labelId) {
    try {
      await api.delete(`/labels/${labelId}`);
      labels.value = labels.value.filter((l) => l.id !== labelId);
      toast.success('Label berhasil dihapus');
    } catch (err) {
      toast.error(err.message || 'Gagal menghapus label');
      throw err;
    }
  }

  async function setIssueLabels(issueId, labelNames) {
    try {
      const res = await api.post(`/issues/${issueId}/labels`, {
        labels: labelNames || [],
      });
      if (currentProjectId) {
        fetchLabels(currentProjectId);
      }
      return res;
    } catch (err) {
      toast.error(err.message || 'Gagal memperbarui label tiket');
      throw err;
    }
  }

  function handleWsMessage(msg) {
    if (!msg || !msg.event) return;
    if (msg.event === 'LABEL_CREATED' && msg.data) {
      if (!labels.value.some((l) => l.id === msg.data.id)) {
        labels.value.push(msg.data);
      }
    } else if (msg.event === 'LABEL_DELETED' && msg.data) {
      labels.value = labels.value.filter((l) => l.id !== msg.data.id);
    }
  }

  wsClient.subscribe(handleWsMessage);

  return {
    labels,
    loading,
    error,
    fetchLabels,
    createLabel,
    deleteLabel,
    setIssueLabels,
  };
});

