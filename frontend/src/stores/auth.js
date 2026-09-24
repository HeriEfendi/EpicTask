import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { api } from '@/services/api';
import { wsClient } from '@/services/websocket';

export const useAuthStore = defineStore('auth', () => {
  const user = ref(null);
  const token = ref(localStorage.getItem('epictask_token') || null);
  const workspaces = ref([]);
  const currentWorkspace = ref(null);
  const loading = ref(false);
  const error = ref(null);
  const allUsers = ref([]);

  const isAuthenticated = computed(() => !!token.value && !!user.value);

  async function login(email, password) {
    loading.value = true;
    error.value = null;
    try {
      const data = await api.post('/auth/login', { email, password });
      token.value = data.token;
      user.value = data.user;
      localStorage.setItem('epictask_token', data.token);
      wsClient.connect();
      await fetchMe();
      return true;
    } catch (err) {
      error.value = err.message || 'Login failed';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function register(email, password, fullName) {
    loading.value = true;
    error.value = null;
    try {
      const data = await api.post('/auth/register', { email, password, full_name: fullName });
      token.value = data.token;
      user.value = data.user;
      localStorage.setItem('epictask_token', data.token);
      wsClient.connect();
      await fetchMe();
      return true;
    } catch (err) {
      error.value = err.message || 'Registration failed';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function fetchMe() {
    if (!token.value) return false;
    try {
      const data = await api.get('/auth/me');
      user.value = data.user;
      workspaces.value = data.workspaces || [];

      // Restore active workspace or select first
      const savedWsId = localStorage.getItem('epictask_current_ws');
      if (savedWsId && workspaces.value.some((w) => w.id === Number(savedWsId))) {
        currentWorkspace.value = workspaces.value.find((w) => w.id === Number(savedWsId));
      } else if (workspaces.value.length > 0) {
        currentWorkspace.value = workspaces.value[0];
        localStorage.setItem('epictask_current_ws', currentWorkspace.value.id);
      }

      await fetchAllUsers();
      return true;
    } catch (err) {
      logout();
      return false;
    }
  }

  async function fetchAllUsers() {
    try {
      const users = await api.get('/users');
      allUsers.value = users || [];
    } catch (e) {
      console.warn('Failed to load users', e);
    }
  }

  function selectWorkspace(ws) {
    currentWorkspace.value = ws;
    localStorage.setItem('epictask_current_ws', ws.id);
  }

  async function createWorkspace(name) {
    const ws = await api.post('/workspaces', { name });
    workspaces.value.push(ws);
    selectWorkspace(ws);
    return ws;
  }

  function logout() {
    token.value = null;
    user.value = null;
    workspaces.value = [];
    currentWorkspace.value = null;
    localStorage.removeItem('epictask_token');
    localStorage.removeItem('epictask_current_ws');
    wsClient.disconnect();
  }

  return {
    user,
    token,
    workspaces,
    currentWorkspace,
    loading,
    error,
    allUsers,
    isAuthenticated,
    login,
    register,
    fetchMe,
    fetchAllUsers,
    selectWorkspace,
    createWorkspace,
    logout,
  };
});
