<script setup>
import { ref } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { useProjectStore } from '@/stores/project';
import { useAutomationStore } from '@/stores/automation';
import { wsClient } from '@/services/websocket';
import { isTauri } from '@/services/tauri';
import {
  Kanban,
  CalendarRange,
  ListTodo,
  BarChart3,
  Bot,
  Workflow,
  Building2,
  FolderKanban,
  Plus,
  CheckCircle2,
  ChevronDown,
  UserCheck,
  Sparkles
} from 'lucide-vue-next';
import { toast } from '@/utils/toast';

const props = defineProps({
  isOpen: {
    type: Boolean,
    default: true,
  }
});

const emit = defineEmits(['open-workflow-modal']);

const authStore = useAuthStore();
const projectStore = useProjectStore();
const autoStore = useAutomationStore();

// Dropdowns inside sidebar
const isWorkspaceMenuOpen = ref(false);
const isProjectMenuOpen = ref(false);

const newWorkspaceName = ref('');
const isCreatingWorkspace = ref(false);

const newProjectName = ref('');
const newProjectKey = ref('');
const isCreatingProject = ref(false);

async function handleCreateWorkspace() {
  if (!newWorkspaceName.value.trim()) return;
  try {
    const ws = await authStore.createWorkspace(newWorkspaceName.value.trim());
    toast.success(`Workspace "${newWorkspaceName.value.trim()}" berhasil dibuat`);
    newWorkspaceName.value = '';
    isCreatingWorkspace.value = false;
    isWorkspaceMenuOpen.value = false;
    await projectStore.fetchProjects(ws.id);
  } catch (e) {
    toast.error(e.message || 'Gagal membuat workspace');
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
    toast.success(`Project "${newProjectName.value.trim()}" [${newProjectKey.value.trim().toUpperCase()}] berhasil dibuat`);
    newProjectName.value = '';
    newProjectKey.value = '';
    isCreatingProject.value = false;
    isProjectMenuOpen.value = false;
  } catch (e) {
    toast.error(e.message || 'Gagal membuat project');
  }
}

function handleSelectProject(p) {
  projectStore.selectProject(p);
  isProjectMenuOpen.value = false;
}

function toggleMyIssues() {
  if (projectStore.filterAssignee === authStore.user?.id) {
    projectStore.filterAssignee = null;
  } else {
    projectStore.filterAssignee = authStore.user?.id;
  }
}
</script>

<template>
  <aside
    class="bg-white border-r border-slate-200 flex flex-col shrink-0 select-none shadow-2xs transition-all duration-200 overflow-hidden"
    :class="isOpen ? 'w-64' : 'w-0'"
  >
    <!-- Scrollable Sidebar Body -->
    <div class="flex-1 overflow-y-auto px-3 py-3 space-y-4">
      <!-- 1. Workspace Selector Section -->
      <div>
        <div class="text-xs font-bold text-slate-500 uppercase tracking-wider px-2 mb-1.5">
          Workspace
        </div>

        <div class="relative">
          <button
            @click="isWorkspaceMenuOpen = !isWorkspaceMenuOpen; isProjectMenuOpen = false"
            class="w-full flex items-center justify-between p-2 rounded-lg border border-slate-200 hover:border-slate-300 hover:bg-slate-50 text-slate-800 transition shadow-2xs text-left"
            :title="authStore.currentWorkspace?.name || 'Workspace'"
          >
            <div class="flex items-center space-x-2.5 min-w-0">
              <div class="w-7 h-7 rounded-md bg-blue-50 border border-blue-200 flex items-center justify-center shrink-0">
                <Building2 class="w-4 h-4 text-blue-600" />
              </div>
              <div class="truncate">
                <p class="text-sm font-semibold text-slate-900 truncate">
                  {{ authStore.currentWorkspace?.name || 'Pilih Workspace' }}
                </p>
                <p class="text-xs text-slate-500">Ruang Kerja</p>
              </div>
            </div>
            <ChevronDown class="w-3.5 h-3.5 text-slate-400 shrink-0 ml-1" />
          </button>

          <!-- Workspace Dropdown Menu -->
          <div
            v-if="isWorkspaceMenuOpen"
            class="absolute left-0 mt-1 w-64 glass-dropdown rounded-xl p-2 z-50 animate-slide-up shadow-xl"
          >
            <div class="text-xs font-bold text-slate-500 uppercase tracking-wider px-2 py-1">
              Daftar Workspace
            </div>
            <div class="max-h-48 overflow-y-auto space-y-1 my-1">
              <button
                v-for="ws in authStore.workspaces"
                :key="ws.id"
                @click="handleSelectWorkspace(ws)"
                class="w-full text-left px-2.5 py-1.5 rounded-lg text-sm flex items-center justify-between hover:bg-slate-100 transition"
                :class="authStore.currentWorkspace?.id === ws.id ? 'text-blue-600 font-semibold bg-blue-50' : 'text-slate-700'"
              >
                <span class="truncate">{{ ws.name }}</span>
                <CheckCircle2 v-if="authStore.currentWorkspace?.id === ws.id" class="w-4 h-4 text-blue-600 shrink-0" />
              </button>
            </div>

            <div class="border-t border-slate-100 pt-2">
              <div v-if="!isCreatingWorkspace">
                <button
                  @click="isCreatingWorkspace = true"
                  class="w-full text-left px-2 py-1.5 rounded-lg text-xs text-blue-600 hover:bg-blue-50 flex items-center space-x-1.5 font-semibold transition"
                >
                  <Plus class="w-4 h-4" />
                  <span>Tambah Workspace Baru</span>
                </button>
              </div>
              <div v-else class="space-y-2 p-1">
                <input
                  v-model="newWorkspaceName"
                  placeholder="Nama workspace..."
                  class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs"
                  @keyup.enter="handleCreateWorkspace"
                />
                <div class="flex justify-end space-x-1.5">
                  <button
                    @click="isCreatingWorkspace = false"
                    class="px-2 py-1 text-xs text-slate-500 hover:text-slate-800"
                  >
                    Batal
                  </button>
                  <button
                    @click="handleCreateWorkspace"
                    class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-xs font-semibold shadow-xs"
                  >
                    Simpan
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 2. Project Selector Section -->
      <div v-if="authStore.currentWorkspace">
        <div class="text-xs font-bold text-slate-500 uppercase tracking-wider px-2 mb-1.5">
          Projek
        </div>

        <div class="relative">
          <button
            @click="isProjectMenuOpen = !isProjectMenuOpen; isWorkspaceMenuOpen = false"
            class="w-full flex items-center justify-between p-2 rounded-lg border border-slate-200 hover:border-slate-300 hover:bg-slate-50 text-slate-800 transition shadow-2xs text-left"
            :title="projectStore.currentProject?.name || 'Project'"
          >
            <div class="flex items-center space-x-2.5 min-w-0">
              <div class="w-7 h-7 rounded-md bg-indigo-50 border border-indigo-200 flex items-center justify-center shrink-0">
                <FolderKanban class="w-4 h-4 text-indigo-600" />
              </div>
              <div class="truncate">
                <p class="text-sm font-semibold text-slate-900 truncate">
                  {{ projectStore.currentProject?.name || 'Pilih Projek' }}
                </p>
                <p class="text-xs text-slate-500 font-mono">
                  {{ projectStore.currentProject?.key || 'Key' }}
                </p>
              </div>
            </div>
            <ChevronDown class="w-3.5 h-3.5 text-slate-400 shrink-0 ml-1" />
          </button>

          <!-- Project Dropdown Menu -->
          <div
            v-if="isProjectMenuOpen"
            class="absolute left-0 mt-1 w-72 glass-dropdown rounded-xl p-2 z-50 animate-slide-up shadow-xl"
          >
            <div class="text-xs font-bold text-slate-500 uppercase tracking-wider px-2 py-1">
              Daftar Projek
            </div>
            <div class="max-h-48 overflow-y-auto space-y-1 my-1">
              <button
                v-for="p in projectStore.projects"
                :key="p.id"
                @click="handleSelectProject(p)"
                class="w-full text-left px-2.5 py-1.5 rounded-lg text-sm flex items-center justify-between hover:bg-slate-100 transition"
                :class="projectStore.currentProject?.id === p.id ? 'text-blue-600 font-semibold bg-blue-50' : 'text-slate-700'"
              >
                <div class="truncate">
                  <span class="font-bold text-xs px-1.5 py-0.5 rounded bg-slate-100 border border-slate-200 text-slate-700 mr-1.5">{{ p.key }}</span>
                  <span>{{ p.name }}</span>
                </div>
                <CheckCircle2 v-if="projectStore.currentProject?.id === p.id" class="w-4 h-4 text-blue-600 shrink-0" />
              </button>
            </div>

            <div class="border-t border-slate-100 pt-2">
              <div v-if="!isCreatingProject">
                <button
                  @click="isCreatingProject = true"
                  class="w-full text-left px-2 py-1.5 rounded-lg text-xs text-blue-600 hover:bg-blue-50 flex items-center space-x-1.5 font-semibold transition"
                >
                  <Plus class="w-4 h-4" />
                  <span>Buat Projek Baru</span>
                </button>
              </div>
              <div v-else class="space-y-2 p-1">
                <input
                  v-model="newProjectName"
                  placeholder="Nama projek..."
                  class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs"
                />
                <input
                  v-model="newProjectKey"
                  placeholder="Key (misal: EPIC)..."
                  maxlength="10"
                  class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 uppercase focus:outline-none focus:border-blue-600 shadow-2xs"
                  @keyup.enter="handleCreateProject"
                />
                <div class="flex justify-end space-x-1.5">
                  <button
                    @click="isCreatingProject = false"
                    class="px-2 py-1 text-xs text-slate-500 hover:text-slate-800"
                  >
                    Batal
                  </button>
                  <button
                    @click="handleCreateProject"
                    class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-xs font-semibold shadow-xs"
                  >
                    Buat
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="border-t border-slate-200 pt-3"></div>

      <!-- 3. Navigation Views Section -->
      <div class="space-y-1">
        <div class="text-xs font-bold text-slate-500 uppercase tracking-wider px-2 mb-1.5">
          Tampilan Board
        </div>

        <button
          @click="projectStore.activeView = 'kanban'"
          class="w-full flex items-center space-x-3 px-2.5 py-2 rounded-lg text-sm transition"
          :class="projectStore.activeView === 'kanban' ? 'bg-blue-50 text-blue-700 font-semibold border border-blue-200/80 shadow-2xs' : 'text-slate-700 hover:bg-slate-100 hover:text-slate-900 font-medium'"
          title="Kanban Board"
        >
          <Kanban class="w-4 h-4 shrink-0" :class="projectStore.activeView === 'kanban' ? 'text-blue-600' : 'text-slate-500'" />
          <span class="flex-1 text-left truncate">Kanban Board</span>
          <span
            v-if="projectStore.filteredIssues.length > 0"
            class="text-xs font-semibold px-2 py-0.5 rounded-full"
            :class="projectStore.activeView === 'kanban' ? 'bg-blue-200/80 text-blue-800' : 'bg-slate-100 text-slate-600'"
          >
            {{ projectStore.filteredIssues.length }}
          </span>
        </button>

        <button
          @click="projectStore.activeView = 'timeline'"
          class="w-full flex items-center space-x-3 px-2.5 py-2 rounded-lg text-sm transition"
          :class="projectStore.activeView === 'timeline' ? 'bg-blue-50 text-blue-700 font-semibold border border-blue-200/80 shadow-2xs' : 'text-slate-700 hover:bg-slate-100 hover:text-slate-900 font-medium'"
          title="Timeline / Gantt"
        >
          <CalendarRange class="w-4 h-4 shrink-0" :class="projectStore.activeView === 'timeline' ? 'text-blue-600' : 'text-slate-500'" />
          <span class="flex-1 text-left truncate">Timeline / Gantt</span>
        </button>

        <button
          @click="projectStore.activeView = 'list'"
          class="w-full flex items-center space-x-3 px-2.5 py-2 rounded-lg text-sm transition"
          :class="projectStore.activeView === 'list' ? 'bg-blue-50 text-blue-700 font-semibold border border-blue-200/80 shadow-2xs' : 'text-slate-700 hover:bg-slate-100 hover:text-slate-900 font-medium'"
          title="List View"
        >
          <ListTodo class="w-4 h-4 shrink-0" :class="projectStore.activeView === 'list' ? 'text-blue-600' : 'text-slate-500'" />
          <span class="flex-1 text-left truncate">List Spreadsheet</span>
        </button>

        <button
          @click="projectStore.activeView = 'reports'"
          class="w-full flex items-center space-x-3 px-2.5 py-2 rounded-lg text-sm transition"
          :class="projectStore.activeView === 'reports' ? 'bg-blue-50 text-blue-700 font-semibold border border-blue-200/80 shadow-2xs' : 'text-slate-700 hover:bg-slate-100 hover:text-slate-900 font-medium'"
          title="Laporan & Analitik"
        >
          <BarChart3 class="w-4 h-4 shrink-0" :class="projectStore.activeView === 'reports' ? 'text-blue-600' : 'text-slate-500'" />
          <span class="flex-1 text-left truncate">Laporan & Analitik</span>
        </button>
      </div>

      <div class="border-t border-slate-200 pt-3"></div>

      <!-- 4. Tools & Configuration Section -->
      <div class="space-y-1">
        <div class="text-xs font-bold text-slate-500 uppercase tracking-wider px-2 mb-1.5">
          Otomasi & Aturan
        </div>

        <button
          @click="autoStore.isRuleModalOpen = true"
          class="w-full flex items-center space-x-3 px-2.5 py-2 rounded-lg text-sm text-slate-700 hover:bg-slate-100 hover:text-slate-900 font-medium transition"
          title="Automation Engine"
        >
          <Bot class="w-4 h-4 text-purple-600 shrink-0" />
          <span class="truncate">No-Code Automation</span>
        </button>

        <button
          @click="emit('open-workflow-modal')"
          class="w-full flex items-center space-x-3 px-2.5 py-2 rounded-lg text-sm text-slate-700 hover:bg-slate-100 hover:text-slate-900 font-medium transition"
          title="Workflow Rules"
        >
          <Workflow class="w-4 h-4 text-blue-600 shrink-0" />
          <span class="truncate">Workflow Guard Rules</span>
        </button>
      </div>
    </div>

    <!-- 5. Bottom Filter Toggle -->
    <div class="p-3 border-t border-slate-200 bg-slate-50/70">
      <button
        @click="toggleMyIssues"
        class="w-full flex items-center space-x-2 px-2.5 py-1.5 rounded-lg text-xs font-semibold border transition shadow-2xs"
        :class="projectStore.filterAssignee === authStore.user?.id ? 'bg-blue-50 border-blue-400 text-blue-700' : 'bg-white border-slate-300 text-slate-700 hover:bg-slate-100'"
        title="Filter issue saya"
      >
        <UserCheck class="w-4 h-4 shrink-0" :class="projectStore.filterAssignee === authStore.user?.id ? 'text-blue-600' : 'text-slate-500'" />
        <span class="truncate">Filter Tiket Saya</span>
      </button>
    </div>
  </aside>
</template>
