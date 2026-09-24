<script setup>
import { ref } from 'vue';
import { useProjectStore } from '@/stores/project';
import KanbanCard from './KanbanCard.vue';
import { Plus, AlertCircle, CheckCircle, ShieldAlert } from 'lucide-vue-next';

const projectStore = useProjectStore();

const dragOverColumnId = ref(null);
const errorMessage = ref(null);
let errorTimeout = null;

function showToast(msg) {
  errorMessage.value = msg;
  if (errorTimeout) clearTimeout(errorTimeout);
  errorTimeout = setTimeout(() => {
    errorMessage.value = null;
  }, 4500);
}

function handleDragOver(e, statusId) {
  e.preventDefault();
  e.dataTransfer.dropEffect = 'move';
  dragOverColumnId.value = statusId;
}

function handleDragLeave(e, statusId) {
  if (dragOverColumnId.value === statusId) {
    dragOverColumnId.value = null;
  }
}

async function handleDrop(e, statusId) {
  e.preventDefault();
  dragOverColumnId.value = null;
  const issueIdStr = e.dataTransfer.getData('text/plain');
  if (!issueIdStr) return;

  const issueId = Number(issueIdStr);
  try {
    await projectStore.moveIssue(issueId, statusId);
  } catch (err) {
    showToast(err.message || 'Cannot move issue to this status');
  }
}

function openCreateForStatus(statusId) {
  projectStore.defaultCreateStatusId = statusId;
  projectStore.isCreateModalOpen = true;
}
</script>

<template>
  <div class="flex-1 p-6 overflow-x-auto relative flex flex-col bg-slate-50">
    <!-- Rule Violation Toast Alert (FR-2.3, FR-4.1) -->
    <transition enter-active-class="transition duration-200 ease-out" enter-from-class="opacity-0 -translate-y-4" enter-to-class="opacity-100 translate-y-0" leave-active-class="transition duration-150 ease-in" leave-from-class="opacity-100 translate-y-0" leave-to-class="opacity-0 -translate-y-4">
      <div
        v-if="errorMessage"
        class="fixed top-16 right-6 z-50 max-w-md bg-white border border-red-300 rounded-xl p-3.5 shadow-2xl flex items-start space-x-3 text-red-700 text-xs"
      >
        <ShieldAlert class="w-5 h-5 text-red-500 shrink-0 mt-0.5" />
        <div class="flex-1">
          <p class="font-bold text-red-900 mb-0.5">Workflow Transition Guard</p>
          <p>{{ errorMessage }}</p>
        </div>
        <button @click="errorMessage = null" class="text-red-500 hover:text-red-800 font-bold ml-1">✕</button>
      </div>
    </transition>

    <!-- Empty Project State -->
    <div v-if="projectStore.statuses.length === 0" class="flex-1 flex flex-col items-center justify-center text-slate-400 py-16">
      <AlertCircle class="w-12 h-12 text-slate-400 mb-3" />
      <h3 class="text-base font-semibold text-slate-700 mb-1">No Columns Configured</h3>
      <p class="text-xs text-slate-500 mb-4">Add workflow statuses to get started with your Kanban board.</p>
    </div>

    <!-- Kanban Columns Container -->
    <div v-else class="flex gap-4 items-start pb-4 min-h-[calc(100vh-160px)]">
      <div
        v-for="status in projectStore.statuses"
        :key="status.id"
        class="w-80 shrink-0 bg-slate-100/90 rounded-xl border border-slate-200/90 flex flex-col max-h-[calc(100vh-170px)] transition-all kanban-column shadow-2xs"
        :class="{ 'drag-over': dragOverColumnId === status.id }"
        @dragover="(e) => handleDragOver(e, status.id)"
        @dragleave="(e) => handleDragLeave(e, status.id)"
        @drop="(e) => handleDrop(e, status.id)"
      >
        <!-- Column Header -->
        <div class="p-3 border-b border-slate-200/80 flex items-center justify-between bg-slate-200/40 rounded-t-xl select-none">
          <div class="flex items-center space-x-2">
            <span
              class="w-2.5 h-2.5 rounded-full"
              :style="{ backgroundColor: status.color || '#3b82f6' }"
            ></span>
            <span class="text-xs font-bold text-slate-800 tracking-wide uppercase">
              {{ status.name }}
            </span>
            <span class="text-[11px] font-mono font-semibold px-2 py-0.5 rounded-full bg-white text-slate-600 border border-slate-200">
              {{ (projectStore.issuesByStatus[status.id] || []).length }}
            </span>
          </div>

          <button
            @click="openCreateForStatus(status.id)"
            class="p-1 rounded hover:bg-slate-200 text-slate-500 hover:text-slate-800 transition"
            title="Create issue in this column"
          >
            <Plus class="w-4 h-4" />
          </button>
        </div>

        <!-- Cards List -->
        <div class="p-2.5 overflow-y-auto space-y-2.5 flex-1 min-h-[140px]">
          <KanbanCard
            v-for="issue in (projectStore.issuesByStatus[status.id] || [])"
            :key="issue.id"
            :issue="issue"
          />

          <!-- Empty Column Drop Zone Placeholder -->
          <div
            v-if="(projectStore.issuesByStatus[status.id] || []).length === 0"
            class="h-28 border border-dashed border-slate-300 rounded-lg flex flex-col items-center justify-center text-slate-400 text-[11px] select-none"
          >
            <span>No issues in this column</span>
            <span class="text-[10px] text-slate-400 mt-1">Drag tickets here</span>
          </div>
        </div>

        <!-- Column Footer: Quick Add Button -->
        <div class="p-2 border-t border-slate-200/60 bg-slate-100 rounded-b-xl">
          <button
            @click="openCreateForStatus(status.id)"
            class="w-full py-1.5 px-3 rounded-lg text-xs font-medium text-slate-600 hover:text-slate-900 hover:bg-slate-200/70 flex items-center justify-center space-x-1.5 transition"
          >
            <Plus class="w-3.5 h-3.5" />
            <span>Create Issue</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
