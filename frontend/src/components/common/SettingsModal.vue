<script setup>
import { ref, watch } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { api } from '@/services/api';
import {
  Settings,
  X,
  Building,
  User,
  Sliders,
  Database
} from 'lucide-vue-next';

import WorkspaceTab from './settings/WorkspaceTab.vue';
import ProfileTab from './settings/ProfileTab.vue';
import PreferencesTab from './settings/PreferencesTab.vue';
import SystemTab from './settings/SystemTab.vue';

const props = defineProps({
  isOpen: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['close']);

const authStore = useAuthStore();

const activeTab = ref('workspace'); // 'workspace' | 'profile' | 'preferences' | 'system'
const loadingMembers = ref(false);
const members = ref([]);
const currentUserRole = ref('MEMBER');

async function fetchWorkspaceDetails() {
  if (!authStore.currentWorkspace?.id) return;
  loadingMembers.value = true;
  try {
    const data = await api.get(`/workspaces/${authStore.currentWorkspace.id}`);
    members.value = data.members || [];
    currentUserRole.value = data.current_user_role || 'MEMBER';
  } catch (e) {
    console.warn('Failed to load workspace members', e);
  } finally {
    loadingMembers.value = false;
  }
}

watch(
  () => props.isOpen,
  (val) => {
    if (val) {
      fetchWorkspaceDetails();
    }
  }
);
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/40 backdrop-blur-xs animate-fade-in"
    @click.self="emit('close')"
  >
    <div
      class="bg-white rounded-2xl border border-slate-200 shadow-2xl w-full max-w-2xl overflow-hidden flex flex-col max-h-[85vh] animate-scale-up"
    >
      <!-- Modal Header -->
      <div class="px-6 py-4 border-b border-slate-200 flex items-center justify-between bg-slate-50/70">
        <div class="flex items-center space-x-3">
          <div class="p-2 rounded-xl bg-blue-50 text-blue-600 border border-blue-100">
            <Settings class="w-5 h-5" />
          </div>
          <div>
            <h2 class="text-base font-bold text-slate-900">Pengaturan Sistem & Workspace</h2>
            <p class="text-xs text-slate-500">Kelola preferensi akun, workspace, profil, dan data aplikasi</p>
          </div>
        </div>

        <button
          @click="emit('close')"
          class="p-1.5 text-slate-400 hover:text-slate-600 hover:bg-slate-100 rounded-lg transition"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Settings Tabs Navigation -->
      <div class="flex items-center space-x-1 px-6 border-b border-slate-200 bg-white text-xs font-bold select-none overflow-x-auto">
        <button
          @click="activeTab = 'workspace'"
          class="py-3 px-3.5 border-b-2 flex items-center space-x-2 transition"
          :class="
            activeTab === 'workspace'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-slate-500 hover:text-slate-800'
          "
        >
          <Building class="w-4 h-4" />
          <span>Workspace & Tim</span>
        </button>

        <button
          @click="activeTab = 'profile'"
          class="py-3 px-3.5 border-b-2 flex items-center space-x-2 transition"
          :class="
            activeTab === 'profile'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-slate-500 hover:text-slate-800'
          "
        >
          <User class="w-4 h-4" />
          <span>Profil Pengguna</span>
        </button>

        <button
          @click="activeTab = 'preferences'"
          class="py-3 px-3.5 border-b-2 flex items-center space-x-2 transition"
          :class="
            activeTab === 'preferences'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-slate-500 hover:text-slate-800'
          "
        >
          <Sliders class="w-4 h-4" />
          <span>Preferensi Tampilan</span>
        </button>

        <button
          @click="activeTab = 'system'"
          class="py-3 px-3.5 border-b-2 flex items-center space-x-2 transition"
          :class="
            activeTab === 'system'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-slate-500 hover:text-slate-800'
          "
        >
          <Database class="w-4 h-4" />
          <span>Sistem & Data</span>
        </button>
      </div>

      <!-- Tab Content Area -->
      <div class="p-6 overflow-y-auto flex-1 space-y-6">
        <WorkspaceTab
          v-if="activeTab === 'workspace'"
          :members="members"
          :loading-members="loadingMembers"
          :current-user-role="currentUserRole"
          @refresh-members="fetchWorkspaceDetails"
        />

        <ProfileTab
          v-else-if="activeTab === 'profile'"
        />

        <PreferencesTab
          v-else-if="activeTab === 'preferences'"
        />

        <SystemTab
          v-else-if="activeTab === 'system'"
        />
      </div>

      <!-- Modal Footer -->
      <div class="px-6 py-3 border-t border-slate-200 bg-slate-50 flex items-center justify-end">
        <button
          @click="emit('close')"
          class="px-4 py-2 rounded-lg text-sm font-semibold bg-white border border-slate-200 text-slate-700 hover:bg-slate-100 transition shadow-2xs"
        >
          Tutup
        </button>
      </div>
    </div>
  </div>
</template>
