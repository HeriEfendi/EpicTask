<script setup>
import { computed } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useAuthStore } from '@/stores/auth';
import { useAutomationStore } from '@/stores/automation';
import {
  Kanban,
  CalendarRange,
  ListTodo,
  Bot,
  Workflow,
  Filter,
  X,
  UserCheck,
  Flag,
  Layers,
  ChevronDown
} from 'lucide-vue-next';

const emit = defineEmits(['open-workflow-modal']);

const projectStore = useProjectStore();
const authStore = useAuthStore();
const autoStore = useAutomationStore();

const hasActiveFilters = computed(() => {
  return (
    !!projectStore.searchQuery ||
    !!projectStore.filterAssignee ||
    !!projectStore.filterPriority ||
    !!projectStore.filterEpic ||
    !!projectStore.filterType
  );
});

function clearFilters() {
  projectStore.searchQuery = '';
  projectStore.filterAssignee = null;
  projectStore.filterPriority = null;
  projectStore.filterEpic = null;
  projectStore.filterType = null;
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
  <div class="px-6 py-2.5 border-b border-slate-200 bg-white flex flex-wrap items-center justify-between gap-3">
    <!-- Left: Views Switcher (Kanban, Timeline, List) -->
    <div class="inline-flex items-center bg-slate-100 p-0.5 rounded-lg border border-slate-200/80">
      <button
        @click="projectStore.activeView = 'kanban'"
        class="flex items-center space-x-1.5 px-3 py-1.5 rounded-md text-xs transition"
        :class="projectStore.activeView === 'kanban' ? 'bg-white text-slate-900 shadow-2xs font-semibold' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-200/50 font-medium'"
      >
        <Kanban class="w-3.5 h-3.5" :class="projectStore.activeView === 'kanban' ? 'text-blue-600' : 'text-slate-500'" />
        <span>Kanban</span>
      </button>

      <button
        @click="projectStore.activeView = 'timeline'"
        class="flex items-center space-x-1.5 px-3 py-1.5 rounded-md text-xs transition"
        :class="projectStore.activeView === 'timeline' ? 'bg-white text-slate-900 shadow-2xs font-semibold' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-200/50 font-medium'"
      >
        <CalendarRange class="w-3.5 h-3.5" :class="projectStore.activeView === 'timeline' ? 'text-blue-600' : 'text-slate-500'" />
        <span>Timeline</span>
      </button>

      <button
        @click="projectStore.activeView = 'list'"
        class="flex items-center space-x-1.5 px-3 py-1.5 rounded-md text-xs transition"
        :class="projectStore.activeView === 'list' ? 'bg-white text-slate-900 shadow-2xs font-semibold' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-200/50 font-medium'"
      >
        <ListTodo class="w-3.5 h-3.5" :class="projectStore.activeView === 'list' ? 'text-blue-600' : 'text-slate-500'" />
        <span>List</span>
      </button>
    </div>

    <!-- Center/Right: Filters & Management Buttons -->
    <div class="flex items-center flex-wrap gap-2">
      <!-- "Only My Issues" Quick Filter -->
      <button
        @click="toggleMyIssues"
        class="flex items-center space-x-1.5 px-2.5 py-1.5 rounded-lg text-xs transition shadow-2xs"
        :class="projectStore.filterAssignee === authStore.user?.id ? 'bg-blue-50 border border-blue-500 text-blue-700 font-semibold' : 'bg-white border border-slate-300 text-slate-700 hover:bg-slate-50 font-medium'"
      >
        <UserCheck class="w-3.5 h-3.5" :class="projectStore.filterAssignee === authStore.user?.id ? 'text-blue-600' : 'text-slate-500'" />
        <span>Only My Issues</span>
      </button>

      <!-- Priority Filter Dropdown -->
      <div class="relative">
        <select
          v-model="projectStore.filterPriority"
          class="bg-white border border-slate-300 hover:border-slate-400 rounded-lg px-2.5 py-1.5 text-xs text-slate-700 focus:outline-none focus:border-blue-500 appearance-none pr-7 cursor-pointer shadow-2xs transition font-medium"
        >
          <option :value="null">Priority: All</option>
          <option value="HIGHEST">🔴 Highest</option>
          <option value="HIGH">🟠 High</option>
          <option value="MEDIUM">🟡 Medium</option>
          <option value="LOW">🔵 Low</option>
        </select>
        <ChevronDown class="w-3 h-3 text-slate-400 absolute right-2.5 top-1/2 -translate-y-1/2 pointer-events-none" />
      </div>

      <!-- Epic Filter Dropdown -->
      <div class="relative" v-if="projectStore.epics.length > 0">
        <select
          v-model="projectStore.filterEpic"
          class="bg-white border border-slate-300 hover:border-slate-400 rounded-lg px-2.5 py-1.5 text-xs text-slate-700 focus:outline-none focus:border-blue-500 appearance-none pr-7 cursor-pointer shadow-2xs transition font-medium max-w-[150px] truncate"
        >
          <option :value="null">Epic: All</option>
          <option v-for="epic in projectStore.epics" :key="epic.id" :value="epic.id">
            🟣 {{ epic.summary }}
          </option>
        </select>
        <ChevronDown class="w-3 h-3 text-slate-400 absolute right-2.5 top-1/2 -translate-y-1/2 pointer-events-none" />
      </div>

      <!-- Clear Filters -->
      <button
        v-if="hasActiveFilters"
        @click="clearFilters"
        class="flex items-center space-x-1 px-2.5 py-1.5 text-xs text-red-600 hover:text-red-700 hover:bg-red-50 rounded-lg border border-red-200 transition font-medium"
        title="Clear all filters"
      >
        <X class="w-3 h-3" />
        <span>Clear</span>
      </button>

      <div class="h-4 w-[1px] bg-slate-200 hidden sm:block mx-1"></div>

      <!-- Automations Engine Button (EPIC 5) -->
      <button
        @click="autoStore.isRuleModalOpen = true"
        class="flex items-center space-x-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-white hover:bg-slate-50 border border-slate-300 hover:border-slate-400 text-slate-700 hover:text-slate-900 transition shadow-2xs"
        title="Configure No-Code Automation Rules"
      >
        <Bot class="w-3.5 h-3.5 text-purple-600" />
        <span>Automation</span>
      </button>

      <!-- Workflow Rules Button (FR-2.3) -->
      <button
        @click="emit('open-workflow-modal')"
        class="flex items-center space-x-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-white hover:bg-slate-50 border border-slate-300 hover:border-slate-400 text-slate-700 hover:text-slate-900 transition shadow-2xs"
        title="Manage Statuses and Workflow Transition Guards"
      >
        <Workflow class="w-3.5 h-3.5 text-blue-600" />
        <span>Workflow Rules</span>
      </button>
    </div>
  </div>
</template>
