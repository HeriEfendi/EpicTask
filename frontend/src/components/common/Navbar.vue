<script setup>
import { ref, onMounted } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { useProjectStore } from '@/stores/project';
import { useNotificationStore } from '@/stores/notification';
import { wsClient } from '@/services/websocket';
import { isTauri } from '@/services/tauri';
import {
  Plus,
  Bell,
  Search,
  CheckCircle2,
  FolderKanban,
  Building2,
  LogOut,
  Sparkles,
  Layers,
  Sliders,
  ChevronDown,
  Monitor,
  Wifi,
  WifiOff
} from 'lucide-vue-next';

const authStore = useAuthStore();
const projectStore = useProjectStore();
const notifStore = useNotificationStore();

const isWsConnected = ref(false);
const isUserMenuOpen = ref(false);
const isWorkspaceMenuOpen = ref(false);
const isProjectMenuOpen = ref(false);

const newWorkspaceName = ref('');
const isCreatingWorkspace = ref(false);

const newProjectName = ref('');
const newProjectKey = ref('');
const isCreatingProject = ref(false);

onMounted(() => {
  wsClient.onStateChange((state) => {
    isWsConnected.value = state;
  });
  notifStore.fetchNotifications();
});

async function handleCreateWorkspace() {
  if (!newWorkspaceName.value.trim()) return;
  try {
    const ws = await authStore.createWorkspace(newWorkspaceName.value.trim());
    newWorkspaceName.value = '';
    isCreatingWorkspace.value = false;
    isWorkspaceMenuOpen.value = false;
    await projectStore.fetchProjects(ws.id);
  } catch (e) {
    alert(e.message || 'Failed to create workspace');
  }
}

async function handleSelectWorkspace(ws) {
  authStore.selectWorkspace(ws);
  isWorkspaceMenuOpen.value = false;
  await projectStore.fetchProjects(ws.id);
}

async function handleCreateProject() {
  if (!newProjectName.value.trim() || !newProjectKey.value.trim()) return;
  try {
    await projectStore.createProject(authStore.currentWorkspace.id, {
      name: newProjectName.value.trim(),
      key: newProjectKey.value.trim().toUpperCase(),
      project_type: 'KANBAN'
    });
    newProjectName.value = '';
    newProjectKey.value = '';
    isCreatingProject.value = false;
    isProjectMenuOpen.value = false;
  } catch (e) {
    alert(e.message || 'Failed to create project');
  }
}

function handleSelectProject(p) {
  projectStore.selectProject(p);
  isProjectMenuOpen.value = false;
}
</script>

<template>
  <header class="h-14 border-b border-slate-200 bg-white px-4 flex items-center justify-between z-30 sticky top-0 shadow-2xs">
    <!-- Left: Logo & Selectors -->
    <div class="flex items-center space-x-3.5">
      <div class="flex items-center space-x-2.5">
        <img src="/src/assets/logo.svg" alt="EpicTask" class="w-7 h-7 rounded-md" />
        <div class="flex items-center space-x-2">
          <span class="font-bold text-base tracking-tight text-slate-900">
            Epic<span class="text-blue-600">Task</span>
          </span>
          <span v-if="isTauri()" class="text-[10px] font-semibold text-slate-600 bg-slate-100 border border-slate-200 px-1.5 py-0.5 rounded flex items-center gap-1">
            <Monitor class="w-3 h-3 text-slate-500" /> Desktop
          </span>
        </div>
      </div>

      <div class="h-5 w-[1px] bg-slate-200 mx-0.5"></div>

      <!-- Workspace Selector -->
      <div class="relative">
        <button
          @click="isWorkspaceMenuOpen = !isWorkspaceMenuOpen; isProjectMenuOpen = false"
          class="flex items-center space-x-2 px-2.5 py-1.5 rounded-lg border border-slate-200 bg-white hover:bg-slate-50 text-xs font-medium text-slate-700 shadow-2xs transition"
        >
          <Building2 class="w-3.5 h-3.5 text-slate-500" />
          <span class="max-w-[130px] truncate">{{ authStore.currentWorkspace?.name || 'Select Workspace' }}</span>
          <ChevronDown class="w-3 h-3 text-slate-400" />
        </button>

        <!-- Workspace Dropdown -->
        <div
          v-if="isWorkspaceMenuOpen"
          class="absolute left-0 mt-1 w-64 glass-dropdown rounded-xl p-2 z-50 animate-slide-up"
        >
          <div class="text-[11px] font-semibold text-slate-500 uppercase tracking-wider px-2 py-1">Workspaces</div>
          <div class="max-h-48 overflow-y-auto space-y-0.5">
            <button
              v-for="ws in authStore.workspaces"
              :key="ws.id"
              @click="handleSelectWorkspace(ws)"
              class="w-full text-left px-2.5 py-1.5 rounded-lg text-xs flex items-center justify-between hover:bg-slate-100 transition"
              :class="authStore.currentWorkspace?.id === ws.id ? 'text-blue-600 font-semibold bg-blue-50' : 'text-slate-700'"
            >
              <span class="truncate">{{ ws.name }}</span>
              <CheckCircle2 v-if="authStore.currentWorkspace?.id === ws.id" class="w-3.5 h-3.5 text-blue-600" />
            </button>
          </div>

          <div class="border-t border-slate-100 mt-2 pt-2">
            <div v-if="!isCreatingWorkspace">
              <button
                @click="isCreatingWorkspace = true"
                class="w-full text-left px-2 py-1.5 rounded-lg text-xs text-blue-600 hover:bg-blue-50 flex items-center space-x-1.5 font-medium transition"
              >
                <Plus class="w-3.5 h-3.5" />
                <span>Create Workspace</span>
              </button>
            </div>
            <div v-else class="space-y-2 p-1">
              <input
                v-model="newWorkspaceName"
                placeholder="Workspace name..."
                class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs"
                @keyup.enter="handleCreateWorkspace"
              />
              <div class="flex justify-end space-x-1.5">
                <button
                  @click="isCreatingWorkspace = false"
                  class="px-2.5 py-1 text-xs text-slate-600 hover:text-slate-900 rounded"
                >
                  Cancel
                </button>
                <button
                  @click="handleCreateWorkspace"
                  class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-xs font-semibold shadow-xs"
                >
                  Save
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Project Selector -->
      <div class="relative" v-if="authStore.currentWorkspace">
        <button
          @click="isProjectMenuOpen = !isProjectMenuOpen; isWorkspaceMenuOpen = false"
          class="flex items-center space-x-2 px-2.5 py-1.5 rounded-lg border border-slate-200 bg-white hover:bg-slate-50 text-xs font-medium text-slate-700 shadow-2xs transition"
        >
          <FolderKanban class="w-3.5 h-3.5 text-slate-500" />
          <span class="max-w-[140px] truncate font-semibold">
            {{ projectStore.currentProject ? `${projectStore.currentProject.name} (${projectStore.currentProject.key})` : 'Select Project' }}
          </span>
          <ChevronDown class="w-3 h-3 text-slate-400" />
        </button>

        <!-- Project Dropdown -->
        <div
          v-if="isProjectMenuOpen"
          class="absolute left-0 mt-1 w-72 glass-dropdown rounded-xl p-2 z-50 animate-slide-up"
        >
          <div class="text-[11px] font-semibold text-slate-500 uppercase tracking-wider px-2 py-1">Projects</div>
          <div class="max-h-48 overflow-y-auto space-y-0.5">
            <button
              v-for="p in projectStore.projects"
              :key="p.id"
              @click="handleSelectProject(p)"
              class="w-full text-left px-2.5 py-1.5 rounded-lg text-xs flex items-center justify-between hover:bg-slate-100 transition"
              :class="projectStore.currentProject?.id === p.id ? 'text-blue-600 font-semibold bg-blue-50' : 'text-slate-700'"
            >
              <div class="truncate">
                <span class="font-bold text-[11px] px-1.5 py-0.5 rounded bg-slate-100 border border-slate-200 text-slate-700 mr-1.5">{{ p.key }}</span>
                <span>{{ p.name }}</span>
              </div>
              <CheckCircle2 v-if="projectStore.currentProject?.id === p.id" class="w-3.5 h-3.5 text-blue-600 shrink-0" />
            </button>
          </div>

          <div class="border-t border-slate-100 mt-2 pt-2">
            <div v-if="!isCreatingProject">
              <button
                @click="isCreatingProject = true"
                class="w-full text-left px-2 py-1.5 rounded-lg text-xs text-blue-600 hover:bg-blue-50 flex items-center space-x-1.5 font-medium transition"
              >
                <Plus class="w-3.5 h-3.5" />
                <span>Create New Project</span>
              </button>
            </div>
            <div v-else class="space-y-2 p-1">
              <input
                v-model="newProjectName"
                placeholder="Project name..."
                class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs"
              />
              <input
                v-model="newProjectKey"
                placeholder="Key (e.g. PROJ)..."
                maxlength="10"
                class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 uppercase focus:outline-none focus:border-blue-600 shadow-2xs"
                @keyup.enter="handleCreateProject"
              />
              <div class="flex justify-end space-x-1.5">
                <button
                  @click="isCreatingProject = false"
                  class="px-2.5 py-1 text-xs text-slate-600 hover:text-slate-900 rounded"
                >
                  Cancel
                </button>
                <button
                  @click="handleCreateProject"
                  class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-xs font-semibold shadow-xs"
                >
                  Create
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Quick "+ Create" Action Button -->
      <button
        @click="projectStore.isCreateModalOpen = true"
        class="bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white font-medium text-xs px-3 py-1.5 rounded-lg shadow-xs flex items-center space-x-1.5 transition ml-1"
      >
        <Plus class="w-3.5 h-3.5" />
        <span>Create</span>
      </button>
    </div>

    <!-- Center: Search Input -->
    <div class="flex-1 max-w-md mx-6">
      <div class="relative">
        <Search class="w-3.5 h-3.5 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
        <input
          v-model="projectStore.searchQuery"
          type="text"
          placeholder="Search issues, keys (e.g. EPIC-2)..."
          class="w-full bg-slate-100 hover:bg-slate-100/70 border border-slate-200 rounded-lg pl-9 pr-4 py-1.5 text-xs text-slate-800 placeholder-slate-400 focus:outline-none focus:border-blue-500 focus:bg-white focus:ring-1 focus:ring-blue-500 transition"
        />
        <span class="absolute right-3 top-1/2 -translate-y-1/2 text-[10px] text-slate-400 font-mono border border-slate-200 bg-white rounded px-1 shadow-2xs">
          /
        </span>
      </div>
    </div>

    <!-- Right: WebSocket status, Notifications, User -->
    <div class="flex items-center space-x-2.5">
      <!-- Real-time WS Sync Indicator (FR-6.1) -->
      <div
        class="flex items-center space-x-1.5 px-2.5 py-1 rounded-lg text-[11px] font-medium transition cursor-default"
        :class="isWsConnected ? 'bg-emerald-50 text-emerald-700 border border-emerald-200' : 'bg-amber-50 text-amber-700 border border-amber-200'"
        :title="isWsConnected ? 'Real-time sync active (WebSocket connected)' : 'Reconnecting to real-time server...'"
      >
        <span class="relative flex h-2 w-2">
          <span v-if="isWsConnected" class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
          <span class="relative inline-flex rounded-full h-2 w-2" :class="isWsConnected ? 'bg-emerald-500' : 'bg-amber-500'"></span>
        </span>
        <span class="hidden sm:inline">{{ isWsConnected ? 'Live' : 'Connecting' }}</span>
      </div>

      <!-- Notification Bell with Badge (FR-6.3) -->
      <button
        @click="notifStore.isOpen = !notifStore.isOpen"
        class="relative p-2 rounded-lg text-slate-600 hover:text-slate-900 hover:bg-slate-100 border border-transparent hover:border-slate-200 transition"
        title="Notifications"
      >
        <Bell class="w-4 h-4" />
        <span
          v-if="notifStore.unreadCount > 0"
          class="absolute top-1.5 right-1.5 w-2 h-2 bg-red-500 rounded-full"
        ></span>
      </button>

      <!-- User Avatar / Profile Menu -->
      <div class="relative">
        <button
          @click="isUserMenuOpen = !isUserMenuOpen"
          class="flex items-center space-x-2 pl-1 pr-2 py-1 rounded-lg hover:bg-slate-100 border border-transparent hover:border-slate-200 transition"
        >
          <img
            :src="authStore.user?.avatar_url || 'https://api.dicebear.com/7.x/avataaars/svg?seed=user'"
            alt="Avatar"
            class="w-6 h-6 rounded-full bg-slate-200 ring-1 ring-slate-300"
          />
          <span class="text-xs font-medium text-slate-700 hidden md:inline max-w-[120px] truncate">
            {{ authStore.user?.full_name || 'User' }}
          </span>
          <ChevronDown class="w-3 h-3 text-slate-400" />
        </button>

        <!-- User Menu Dropdown -->
        <div
          v-if="isUserMenuOpen"
          class="absolute right-0 mt-1 w-56 glass-dropdown rounded-xl p-2 z-50 animate-slide-up"
        >
          <div class="px-3 py-2 border-b border-slate-100">
            <p class="text-xs font-bold text-slate-900 truncate">{{ authStore.user?.full_name }}</p>
            <p class="text-[11px] text-slate-500 truncate">{{ authStore.user?.email }}</p>
          </div>

          <div class="py-1">
            <button
              @click="authStore.logout(); isUserMenuOpen = false"
              class="w-full text-left px-3 py-2 rounded-lg text-xs text-red-600 hover:bg-red-50 flex items-center space-x-2 transition font-medium"
            >
              <LogOut class="w-3.5 h-3.5" />
              <span>Log Out</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>
