import { defineStore } from 'pinia';
import { ref } from 'vue';
import { api } from '@/services/api';

export const useAutomationStore = defineStore('automation', () => {
  const rules = ref([]);
  const loading = ref(false);
  const isRuleModalOpen = ref(false);

  async function fetchRules(projectId) {
    if (!projectId) return;
    loading.value = true;
    try {
      const data = await api.get(`/projects/${projectId}/automations`);
      rules.value = data || [];
    } catch (e) {
      console.error('Failed to load automation rules', e);
    } finally {
      loading.value = false;
    }
  }

  async function createRule(projectId, payload) {
    const rule = await api.post(`/projects/${projectId}/automations`, payload);
    rules.value.unshift(rule);
    return rule;
  }

  async function toggleRule(projectId, ruleId, isActive) {
    const updated = await api.put(`/projects/${projectId}/automations/${ruleId}`, {
      is_active: isActive,
    });
    const idx = rules.value.findIndex((r) => r.id === ruleId);
    if (idx !== -1) {
      rules.value[idx] = updated;
    }
    return updated;
  }

  async function deleteRule(projectId, ruleId) {
    await api.delete(`/projects/${projectId}/automations/${ruleId}`);
    rules.value = rules.value.filter((r) => r.id !== ruleId);
  }

  return {
    rules,
    loading,
    isRuleModalOpen,
    fetchRules,
    createRule,
    toggleRule,
    deleteRule,
  };
});
