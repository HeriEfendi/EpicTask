<script setup>
import { ref, computed } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useSprintStore } from '@/stores/sprint';
import { useComponentStore } from '@/stores/component';
import {
  Plus,
  Play,
  Boxes,
  Package,
  Layers,
  Filter,
  Search,
  CheckCircle2,
  Calendar,
  Sparkles
} from 'lucide-vue-next';

import SprintContainer from './SprintContainer.vue';
import BacklogContainer from './BacklogContainer.vue';
import CreateSprintModal from './CreateSprintModal.vue';
import StartSprintModal from './StartSprintModal.vue';
import CompleteSprintModal from './CompleteSprintModal.vue';
import ComponentManagerModal from './ComponentManagerModal.vue';

const projectStore = useProjectStore();
const sprintStore = useSprintStore();
const compStore = useComponentStore();

// Modals State
const isCreateSprintModalOpen = ref(false);
const isStartSprintModalOpen = ref(false);
const isCompleteSprintModalOpen = ref(false);
const isComponentModalOpen = ref(false);

const activeSprintToStart = ref(null);
const activeSprintToComplete = ref(null);

// Component Filter in Backlog View
const selectedComponentFilter = ref(null);

// Filtered issues by component if selected
const projectIssues = computed(() => {
  return projectStore.issues.filter((issue) => {
    // If component filter active
    if (selectedComponentFilter.value && issue.component_id !== selectedComponentFilter.value) {
      return false;
    }
    // Search query filter
    if (projectStore.searchQuery.trim()) {
      const q = projectStore.searchQuery.toLowerCase();
      const matchesSummary = issue.summary.toLowerCase().includes(q);
      const matchesKey = issue.key.toLowerCase().includes(q);
      if (!matchesSummary && !matchesKey) return false;
    }
    return true;
  });
});

// Issues belonging to Product Backlog (sprint_id === null)
const backlogIssues = computed(() => {
  return projectIssues.value.filter((i) => !i.sprint_id);
});

// Group issues by sprint_id
const issuesBySprintId = computed(() => {
  const map = {};
  sprintStore.sprints.forEach((s) => {
    map[s.id] = [];
  });

  projectIssues.value.forEach((i) => {
    if (i.sprint_id && map[i.sprint_id]) {
      map[i.sprint_id].push(i);
    }
  });

  return map;
});

// Modals opening handlers
function handleStartSprint(sprint) {
  activeSprintToStart.value = sprint;
  isStartSprintModalOpen.value = true;
}

function handleCompleteSprint(sprint) {
  activeSprintToComplete.value = sprint;
  isCompleteSprintModalOpen.value = true;
}

async function handleDeleteSprint(sprint) {
  if (confirm(`Hapus sprint "${sprint.name}"? Seluruh isu di dalamnya akan dikembalikan dengan aman ke Product Backlog.`)) {
    await sprintStore.deleteSprint(sprint.id);
    if (projectStore.currentProject) {
      projectStore.fetchIssues(projectStore.currentProject.id);
    }
  }
}

async function handleRefresh() {
  if (projectStore.currentProject) {
    await Promise.all([
      projectStore.fetchIssues(projectStore.currentProject.id),
      sprintStore.fetchSprints(projectStore.currentProject.id),
      compStore.fetchComponents(projectStore.currentProject.id),
    ]);
  }
}
</script>

<template>
  <div class="flex-1 flex flex-col overflow-hidden bg-slate-50/60 select-none">
    <!-- Backlog Sub-Header Toolbar -->
    <div class="px-6 py-3 border-b border-slate-200 bg-white flex flex-wrap items-center justify-between gap-3 shrink-0">
      <!-- Left: Title & Quick Counts -->
      <div class="flex items-center space-x-3">
        <div class="flex items-center space-x-2">
          <Layers class="w-5 h-5 text-blue-600" />
          <h2 class="text-base font-bold text-slate-900">Perencanaan Backlog & Sprint</h2>
        </div>

        <div class="flex items-center space-x-1.5 text-xs text-slate-500 font-semibold border-l border-slate-200 pl-3">
          <span class="px-2 py-0.5 rounded-full bg-slate-100 text-slate-700">
            {{ sprintStore.sprints.length }} Sprint
          </span>
          <span class="px-2 py-0.5 rounded-full bg-slate-100 text-slate-700">
            {{ backlogIssues.length }} Isu Backlog
          </span>
        </div>
      </div>

      <!-- Right: Actions (Component Manager, Filter, Create Sprint) -->
      <div class="flex items-center space-x-2.5">
        <!-- Component Filter Select -->
        <select
          v-model="selectedComponentFilter"
          class="bg-slate-50 border border-slate-200 hover:border-slate-300 rounded-lg px-2.5 py-1.5 text-xs font-semibold text-slate-700 focus:outline-none focus:border-blue-600 transition shadow-2xs cursor-pointer"
        >
          <option :value="null">Semua Komponen</option>
          <option v-for="c in compStore.components" :key="c.id" :value="c.id">
            🧩 {{ c.name }}
          </option>
        </select>

        <!-- Kelola Komponen Button -->
        <button
          @click="isComponentModalOpen = true"
          class="flex items-center space-x-1.5 px-3 py-1.5 bg-white border border-slate-200 hover:border-slate-300 hover:bg-slate-50 text-slate-700 rounded-lg text-xs font-semibold transition shadow-2xs"
        >
          <Boxes class="w-3.5 h-3.5 text-cyan-700" />
          <span>Komponen</span>
        </button>

        <!-- Buat Sprint Button -->
        <button
          @click="isCreateSprintModalOpen = true"
          class="flex items-center space-x-1.5 px-3.5 py-1.5 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-lg text-xs font-bold transition shadow-xs"
        >
          <Plus class="w-3.5 h-3.5" />
          <span>Buat Sprint</span>
        </button>
      </div>
    </div>

    <!-- Main Scrollable Backlog Body -->
    <div class="flex-1 overflow-y-auto p-6 max-w-7xl mx-auto w-full">
      <!-- 1. Active & Planned Sprints Section -->
      <div v-if="sprintStore.sprints.length > 0">
        <SprintContainer
          v-for="sprint in sprintStore.sprints"
          :key="sprint.id"
          :sprint="sprint"
          :issues="issuesBySprintId[sprint.id] || []"
          :project-id="projectStore.currentProject ? projectStore.currentProject.id : 0"
          @start-sprint="handleStartSprint"
          @complete-sprint="handleCompleteSprint"
          @delete-sprint="handleDeleteSprint"
          @issue-moved="handleRefresh"
        />
      </div>

      <!-- 2. Product Backlog Section -->
      <BacklogContainer
        :issues="backlogIssues"
        :project-id="projectStore.currentProject ? projectStore.currentProject.id : 0"
        @issue-moved="handleRefresh"
      />
    </div>

    <!-- Modals -->
    <CreateSprintModal
      :is-open="isCreateSprintModalOpen"
      :project-id="projectStore.currentProject ? projectStore.currentProject.id : 0"
      :project-key="projectStore.currentProject ? projectStore.currentProject.key : 'PROJ'"
      :existing-sprint-count="sprintStore.sprints.length"
      @close="isCreateSprintModalOpen = false"
      @created="handleRefresh"
    />

    <StartSprintModal
      :is-open="isStartSprintModalOpen"
      :sprint="activeSprintToStart"
      :issue-count="activeSprintToStart ? (issuesBySprintId[activeSprintToStart.id]?.length || 0) : 0"
      @close="isStartSprintModalOpen = false"
      @started="handleRefresh"
    />

    <CompleteSprintModal
      :is-open="isCompleteSprintModalOpen"
      :sprint="activeSprintToComplete"
      :issues="activeSprintToComplete ? (issuesBySprintId[activeSprintToComplete.id] || []) : []"
      @close="isCompleteSprintModalOpen = false"
      @completed="handleRefresh"
    />

    <ComponentManagerModal
      :is-open="isComponentModalOpen"
      :project-id="projectStore.currentProject ? projectStore.currentProject.id : 0"
      @close="isComponentModalOpen = false"
    />
  </div>
</template>

