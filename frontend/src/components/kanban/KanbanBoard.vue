<script setup>
import { ref } from 'vue';
import { useProjectStore } from '@/stores/project';
import KanbanCard from './KanbanCard.vue';
import KanbanColumn from './KanbanColumn.vue';
import KanbanToolbar from './KanbanToolbar.vue';
import KanbanSwimlaneView from './KanbanSwimlaneView.vue';
import { AlertCircle, ShieldAlert } from 'lucide-vue-next';

const projectStore = useProjectStore();

const dragOverColumnId = ref(null);
const dragOverCellKey = ref(null);
const errorMessage = ref(null);
let errorTimeout = null;

function showToast(msg) {
  errorMessage.value = msg;
  if (errorTimeout) clearTimeout(errorTimeout);
  errorTimeout = setTimeout(() => {
    errorMessage.value = null;
  }, 4500);
}

// Drag & Drop handlers for Standard Columns
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
  dragOverCellKey.value = null;
  const issueIdStr = e.dataTransfer.getData('text/plain');
  if (!issueIdStr) return;

  const issueId = Number(issueIdStr);
  try {
    await projectStore.moveIssue(issueId, statusId);
  } catch (err) {
    showToast(err.message || 'Tidak dapat memindahkan tiket ke status ini');
  }
}

// Drag & Drop handlers for Swimlane View
function handleSwimlaneDragOver({ event, cellKey, statusId }) {
  event.preventDefault();
  event.dataTransfer.dropEffect = 'move';
  dragOverCellKey.value = cellKey;
}

function handleSwimlaneDragLeave({ cellKey }) {
  if (dragOverCellKey.value === cellKey) {
    dragOverCellKey.value = null;
  }
}

function handleSwimlaneDrop({ event, statusId }) {
  handleDrop(event, statusId);
}

function openCreateForStatus(statusId) {
  projectStore.defaultCreateStatusId = statusId;
  projectStore.isCreateModalOpen = true;
}
</script>

<template>
  <div class="flex-1 flex flex-col h-full bg-slate-50 overflow-hidden relative">
    <!-- Top Kanban Toolbar (Swimlane Switcher & Counter) -->
    <KanbanToolbar />

    <!-- Rule Violation Toast Alert (FR-2.3, FR-4.1) -->
    <transition
      enter-active-class="transition duration-200 ease-out"
      enter-from-class="opacity-0 -translate-y-4"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="transition duration-150 ease-in"
      leave-from-class="opacity-100 translate-y-0"
      leave-to-class="opacity-0 -translate-y-4"
    >
      <div
        v-if="errorMessage"
        class="fixed top-16 right-6 z-50 max-w-md bg-white border border-red-200 rounded-xl p-4 shadow-xl flex items-start space-x-3 text-red-700 text-sm"
      >
        <ShieldAlert class="w-5 h-5 text-red-500 shrink-0 mt-0.5" />
        <div class="flex-1">
          <p class="font-bold text-red-900 mb-0.5">Workflow Transition Guard</p>
          <p>{{ errorMessage }}</p>
        </div>
        <button
          @click="errorMessage = null"
          class="text-red-400 hover:text-red-700 font-bold ml-1"
        >
          ✕
        </button>
      </div>
    </transition>

    <!-- Empty Project State -->
    <div
      v-if="projectStore.statuses.length === 0"
      class="flex-1 flex flex-col items-center justify-center text-slate-400 py-20"
    >
      <AlertCircle class="w-14 h-14 text-slate-300 mb-3" />
      <h3 class="text-lg font-bold text-slate-700 mb-1">Belum Ada Kolom Alur Kerja</h3>
      <p class="text-sm text-slate-500 mb-4">
        Tambahkan status workflow untuk mulai menggunakan Kanban board.
      </p>
    </div>

    <!-- Main Board Area -->
    <div v-else class="flex-1 p-6 overflow-auto relative">
      <!-- 1. Standard Vertical Columns Mode (swimlaneMode === 'none') -->
      <div
        v-if="projectStore.swimlaneMode === 'none'"
        class="flex gap-5 items-start pb-4 min-h-[calc(100vh-190px)] min-w-max"
      >
        <KanbanColumn
          v-for="status in projectStore.statuses"
          :key="status.id"
          :status="status"
          :issues="projectStore.issuesByStatus[status.id] || []"
          :is-drag-over="dragOverColumnId === status.id"
          @dragover="(e) => handleDragOver(e, status.id)"
          @dragleave="(e) => handleDragLeave(e, status.id)"
          @drop="(e) => handleDrop(e, status.id)"
          @create-issue="openCreateForStatus"
        />
      </div>

      <!-- 2. Horizontal Swimlane Mode (assignee | epic | priority) -->
      <KanbanSwimlaneView
        v-else
        :statuses="projectStore.statuses"
        :issues="projectStore.filteredIssues"
        :drag-over-cell-key="dragOverCellKey"
        @dragover="handleSwimlaneDragOver"
        @dragleave="handleSwimlaneDragLeave"
        @drop="handleSwimlaneDrop"
        @create-issue="openCreateForStatus"
      />
    </div>
  </div>
</template>
