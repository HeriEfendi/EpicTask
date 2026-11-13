import { defineStore } from 'pinia';
import { ref } from 'vue';
import { api } from '@/services/api';
import { wsClient } from '@/services/websocket';
import { toast } from '@/utils/toast';

export const useVersionStore = defineStore('version', () => {
  const versions = ref([]);
  const loading = ref(false);
  const error = ref(null);
  let currentProjectId = null;

  async function fetchVersions(projectId) {
    if (!projectId) return;
    currentProjectId = projectId;
    loading.value = true;
    error.value = null;
    try {
      const data = await api.get(`/projects/${projectId}/versions`);
      versions.value = data || [];
    } catch (err) {
      error.value = err.message;
      toast.error(err.message || 'Gagal memuat rilis/versi proyek');
    } finally {
      loading.value = false;
    }
  }

  async function createVersion(projectId, payload) {
    try {
      const created = await api.post(`/projects/${projectId}/versions`, payload);
      if (!versions.value.some((v) => v.id === created.id)) {
        versions.value.unshift(created);
      }
      toast.success(`Versi "${created.name}" berhasil dibuat`);
      return created;
    } catch (err) {
      toast.error(err.message || 'Gagal membuat versi baru');
      throw err;
    }
  }

  async function updateVersion(versionId, payload) {
    try {
      await api.put(`/versions/${versionId}`, payload);
      const idx = versions.value.findIndex((v) => v.id === versionId);
      if (idx !== -1) {
        versions.value[idx] = { ...versions.value[idx], ...payload };
      }
      toast.success('Versi berhasil diperbarui');
    } catch (err) {
      toast.error(err.message || 'Gagal memperbarui versi');
      throw err;
    }
  }

  async function releaseVersion(versionId, payload) {
    try {
      const res = await api.post(`/versions/${versionId}/release`, payload || {});
      toast.success('Versi berhasil dirilis!');
      if (currentProjectId) {
        await fetchVersions(currentProjectId);
      }
      return res;
    } catch (err) {
      toast.error(err.message || 'Gagal merilis versi');
      throw err;
    }
  }

  async function archiveVersion(versionId) {
    try {
      const res = await api.post(`/versions/${versionId}/archive`, {});
      const idx = versions.value.findIndex((v) => v.id === versionId);
      if (idx !== -1 && res.version_status) {
        versions.value[idx].status = res.version_status;
      }
      toast.success(`Versi berhasil di-${res.version_status === 'ARCHIVED' ? 'arsipkan' : 'buka kembali'}`);
      return res;
    } catch (err) {
      toast.error(err.message || 'Gagal mengubah status arsip');
      throw err;
    }
  }

  async function deleteVersion(versionId) {
    try {
      await api.delete(`/versions/${versionId}`);
      versions.value = versions.value.filter((v) => v.id !== versionId);
      toast.success('Versi berhasil dihapus');
    } catch (err) {
      toast.error(err.message || 'Gagal menghapus versi');
      throw err;
    }
  }

  function handleWsMessage(msg) {
    if (!msg || !msg.event) return;
    if (msg.event === 'VERSION_CREATED' && msg.data) {
      if (!versions.value.some((v) => v.id === msg.data.id)) {
        versions.value.unshift(msg.data);
      }
    } else if (
      (msg.event === 'VERSION_UPDATED' || msg.event === 'VERSION_RELEASED') &&
      msg.project_id &&
      currentProjectId === msg.project_id
    ) {
      fetchVersions(msg.project_id);
    } else if (msg.event === 'VERSION_DELETED' && msg.data) {
      versions.value = versions.value.filter((v) => v.id !== msg.data.id);
    }
  }

  wsClient.subscribe(handleWsMessage);

  return {
    versions,
    loading,
    error,
    fetchVersions,
    createVersion,
    updateVersion,
    releaseVersion,
    archiveVersion,
    deleteVersion,
  };
});

