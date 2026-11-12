import { defineStore } from 'pinia';
import { ref } from 'vue';
import { api } from '@/services/api';
import { wsClient } from '@/services/websocket';
import { toast } from '@/utils/toast';

export const useComponentStore = defineStore('component', () => {
  const components = ref([]);
  const loading = ref(false);
  const error = ref(null);

  async function fetchComponents(projectId) {
    if (!projectId) return;
    loading.value = true;
    error.value = null;
    try {
      const data = await api.get(`/projects/${projectId}/components`);
      components.value = data || [];
    } catch (err) {
      error.value = err.message;
      toast.error(err.message || 'Gagal memuat komponen proyek');
    } finally {
      loading.value = false;
    }
  }

  async function createComponent(projectId, payload) {
    try {
      const created = await api.post(`/projects/${projectId}/components`, payload);
      if (!components.value.some((c) => c.id === created.id)) {
        components.value.push(created);
      }
      toast.success(`Komponen "${created.name}" berhasil ditambahkan`);
      return created;
    } catch (err) {
      toast.error(err.message || 'Gagal membuat komponen');
      throw err;
    }
  }

  async function updateComponent(componentId, payload) {
    try {
      await api.put(`/components/${componentId}`, payload);
      const idx = components.value.findIndex((c) => c.id === componentId);
      if (idx !== -1) {
        components.value[idx] = { ...components.value[idx], ...payload };
      }
      toast.success('Komponen berhasil diperbarui');
    } catch (err) {
      toast.error(err.message || 'Gagal memperbarui komponen');
      throw err;
    }
  }

  async function deleteComponent(componentId) {
    try {
      await api.delete(`/components/${componentId}`);
      components.value = components.value.filter((c) => c.id !== componentId);
      toast.success('Komponen berhasil dihapus');
    } catch (err) {
      toast.error(err.message || 'Gagal menghapus komponen');
      throw err;
    }
  }

  function handleWsMessage(msg) {
    if (!msg || !msg.event) return;
    if (msg.event === 'COMPONENT_CREATED' && msg.data) {
      if (!components.value.some((c) => c.id === msg.data.id)) {
        components.value.push(msg.data);
      }
    } else if (msg.event === 'COMPONENT_UPDATED' && msg.project_id) {
      fetchComponents(msg.project_id);
    } else if (msg.event === 'COMPONENT_DELETED' && msg.data) {
      components.value = components.value.filter((c) => c.id !== msg.data.id);
    }
  }

  wsClient.subscribe(handleWsMessage);

  return {
    components,
    loading,
    error,
    fetchComponents,
    createComponent,
    updateComponent,
    deleteComponent,
  };
});

