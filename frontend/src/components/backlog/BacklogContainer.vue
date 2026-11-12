<script setup>
import { ref, computed } from 'vue';
import { useSprintStore } from '@/stores/sprint';
import { useProjectStore } from '@/stores/project';
import { ChevronDown, ChevronRight, Plus, Package } from 'lucide-vue-next';
import BacklogIssueItem from './BacklogIssueItem.vue';

const props = defineProps({
  issues: {
    type: Array,
    default: () => [],
  },
  projectId: {
    type: Number,
    required: true,
  },
});

const emit = defineEmits(['create-issue', 'issue-moved']);

const sprintStore = useSprintStore();
const projectStore = useProjectStore();

const isCollapsed = ref(false);
const isDragOver = ref(false);

const totalPoints = computed(() => {
  return props.issues.reduce((acc, curr) => acc + (Number(curr.story_points) || 0), 0);
});

function handleDragOver(e) {
  e.preventDefault();
  isDragOver.value = true;
}

function handleDragLeave() {
  isDragOver.value = false;
}

async function handleDrop(e) {
  e.preventDefault();
  isDragOver.value = false;
  const issueIdStr = e.dataTransfer.getData('text/plain');
  if (!issueIdStr) return;
  const issueId = Number(issueIdStr);
  if (!issueId) return;

  await sprintStore.moveIssuesToSprint(props.projectId, [issueId], null);
  emit('issue-moved');
}

function handleQuickCreate() {
  projectStore.isCreateModalOpen = true;
}
</script>

<template>
  <div
    class="bg-white rounded-xl border border-slate-200 overflow-hidden shadow-2xs transition-all duration-150 mb-6"
    :class="isDragOver ? 'ring-2 ring-blue-500 bg-blue-50/20 border-blue-400' : ''"
    @dragover="handleDragOver"
    @dragleave="handleDragLeave"
    @drop="handleDrop"
  >
    <!-- Backlog Header -->
    <div class="px-4 py-3 bg-slate-100/90 border-b border-slate-200 flex items-center justify-between select-none">
      <div class="flex items-center space-x-2 min-w-0">
        <button
          @click="isCollapsed = !isCollapsed"
          class="text-slate-500 hover:text-slate-800 p-0.5 rounded transition"
          :title="isCollapsed ? 'Buka Backlog' : 'Tutup Backlog'"
        >
          <ChevronDown v-if="!isCollapsed" class="w-4 h-4" />
          <ChevronRight v-else class="w-4 h-4" />
        </button>

        <Package class="w-4 h-4 text-slate-600 shrink-0" />
        <span class="font-bold text-sm text-slate-900">Product Backlog</span>
        <span class="text-xs text-slate-500 font-semibold">
          ({{ issues.length }} isu)
        </span>
      </div>

      <div class="flex items-center space-x-3">
        <!-- Total story points badge -->
        <div class="text-xs font-mono font-bold px-2 py-0.5 rounded-full bg-slate-200 text-slate-700 border border-slate-300" title="Total Story Points di Backlog">
          {{ totalPoints }} poin
        </div>

        <!-- Quick Create Ticket Button -->
        <button
          @click="handleQuickCreate"
          class="flex items-center space-x-1 px-2.5 py-1 text-xs font-bold text-blue-600 hover:text-blue-700 hover:bg-blue-50 rounded-lg transition"
        >
          <Plus class="w-3.5 h-3.5" />
          <span>Buat Isu</span>
        </button>
      </div>
    </div>

    <!-- Backlog Issue List Body -->
    <div v-show="!isCollapsed" class="divide-y divide-slate-100">
      <!-- Empty State -->
      <div
        v-if="issues.length === 0"
        class="py-8 px-4 text-center text-xs text-slate-400 italic bg-slate-50/40"
      >
        Backlog kosong. Buat tiket baru untuk mulai mengumpulkan inisiatif dan kebutuhan pekerjaan.
      </div>

      <!-- Issues List -->
      <BacklogIssueItem
        v-for="issue in issues"
        :key="issue.id"
        :issue="issue"
      />
    </div>
  </div>
</template>

