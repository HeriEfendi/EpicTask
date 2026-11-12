import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { api } from '@/services/api';
import { wsClient } from '@/services/websocket';
import { toast } from '@/utils/toast';

export const useSprintStore = defineStore('sprint', () => {
  const sprints = ref([]);
  const loading = ref(false);
  const error = ref(null);

  const activeSprint = computed(() => {
    return sprints.value.find((s) => s.status === 'ACTIVE') || null;
  });

  const plannedSprints = computed(() => {
    return sprints.value.filter((s) => s.status === 'PLANNED');
  });

  const completedSprints = computed(() => {
    return sprints.value.filter((s) => s.status === 'COMPLETED');
  });

  async function fetchSprints(projectId) {
    if (!projectId) return;
    loading.value = true;
    error.value = null;
    try {
      const data = await api.get(`/projects/${projectId}/sprints`);
      sprints.value = data || [];
    } catch (err) {
      error.value = err.message;
      toast.error(err.message || 'Gagal memuat daftar sprint');
    } finally {
      loading.value = false;
    }
  }

  async function createSprint(projectId, payload) {
    try {
      const created = await api.post(`/projects/${projectId}/sprints`, payload);
      // Avoid duplicate if WS broadcast arrives quickly
      if (!sprints.value.some((s) => s.id === created.id)) {
        sprints.value.push(created);
      }
      toast.success(`Sprint "${created.name}" berhasil dibuat`);
      return created;
    } catch (err) {
      toast.error(err.message || 'Gagal membuat sprint');
      throw err;
    }
  }

  async function updateSprint(sprintId, payload) {
    try {
      await api.put(`/sprints/${sprintId}`, payload);
      const idx = sprints.value.findIndex((s) => s.id === sprintId);
      if (idx !== -1) {
        sprints.value[idx] = { ...sprints.value[idx], ...payload };
      }
      toast.success('Sprint berhasil diperbarui');
    } catch (err) {
      toast.error(err.message || 'Gagal memperbarui sprint');
      throw err;
    }
  }

  async function startSprint(sprintId, payload) {
    try {
      await api.post(`/sprints/${sprintId}/start`, payload);
      const idx = sprints.value.findIndex((s) => s.id === sprintId);
      if (idx !== -1) {
        sprints.value[idx].status = 'ACTIVE';
        if (payload.start_date) sprints.value[idx].start_date = payload.start_date;
        if (payload.end_date) sprints.value[idx].end_date = payload.end_date;
        if (payload.goal) sprints.value[idx].goal = payload.goal;
      }
      toast.success('Sprint resmi dimulai!');
    } catch (err) {
      toast.error(err.message || 'Gagal memulai sprint');
      throw err;
    }
  }

  async function completeSprint(sprintId, payload) {
    try {
      await api.post(`/sprints/${sprintId}/complete`, payload);
      const idx = sprints.value.findIndex((s) => s.id === sprintId);
      if (idx !== -1) {
        sprints.value[idx].status = 'COMPLETED';
      }
      toast.success('Sprint berhasil diselesaikan!');
    } catch (err) {
      toast.error(err.message || 'Gagal menyelesaikan sprint');
      throw err;
    }
  }

  async function deleteSprint(sprintId) {
    try {
      await api.delete(`/sprints/${sprintId}`);
      sprints.value = sprints.value.filter((s) => s.id !== sprintId);
      toast.success('Sprint berhasil dihapus');
    } catch (err) {
      toast.error(err.message || 'Gagal menghapus sprint');
      throw err;
    }
  }

  async function moveIssuesToSprint(projectId, issueIds, targetSprintId) {
    try {
      await api.post(`/projects/${projectId}/sprints/move-issues`, {
        issue_ids: issueIds,
        target_sprint_id: targetSprintId,
      });
      // Re-fetch sprints to recalculate points rollup
      fetchSprints(projectId);
    } catch (err) {
      toast.error(err.message || 'Gagal memindahkan tiket');
      throw err;
    }
  }

  // Real-time WebSocket listener
  function handleWsMessage(msg) {
    if (!msg || !msg.event) return;
    if (msg.event === 'SPRINT_CREATED' && msg.data) {
      if (!sprints.value.some((s) => s.id === msg.data.id)) {
        sprints.value.push(msg.data);
      }
    } else if (msg.event === 'SPRINT_UPDATED' || msg.event === 'SPRINT_STARTED' || msg.event === 'SPRINT_COMPLETED') {
      if (msg.project_id) {
        fetchSprints(msg.project_id);
      }
    } else if (msg.event === 'SPRINT_DELETED' && msg.data) {
      sprints.value = sprints.value.filter((s) => s.id !== msg.data.id);
    } else if (msg.event === 'SPRINT_ISSUES_MOVED' && msg.project_id) {
      fetchSprints(msg.project_id);
    }
  }

  wsClient.subscribe(handleWsMessage);

  return {
    sprints,
    activeSprint,
    plannedSprints,
    completedSprints,
    loading,
    error,
    fetchSprints,
    createSprint,
    updateSprint,
    startSprint,
    completeSprint,
    deleteSprint,
    moveIssuesToSprint,
  };
});

