<script setup>
import { ref, computed } from 'vue';
import { useSprintStore } from '@/stores/sprint';
import SprintHeader from './SprintHeader.vue';
import BacklogIssueItem from './BacklogIssueItem.vue';

const props = defineProps({
  sprint: {
    type: Object,
    required: true,
  },
  issues: {
    type: Array,
    default: () => [],
  },
  projectId: {
    type: Number,
    required: true,
  },
});

const emit = defineEmits([
  'start-sprint',
  'complete-sprint',
  'edit-sprint',
  'delete-sprint',
  'issue-moved',
]);

const sprintStore = useSprintStore();
const isCollapsed = ref(false);
const isDragOver = ref(false);

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

  await sprintStore.moveIssuesToSprint(props.projectId, [issueId], props.sprint.id);
  emit('issue-moved');
}
</script>

<template>
  <div
    class="bg-white rounded-xl border border-slate-200 overflow-hidden shadow-2xs transition-all duration-150 mb-4"
    :class="isDragOver ? 'ring-2 ring-blue-500 bg-blue-50/20 border-blue-400' : ''"
    @dragover="handleDragOver"
    @dragleave="handleDragLeave"
    @drop="handleDrop"
  >
    <!-- Sprint Header -->
    <SprintHeader
      :sprint="sprint"
      :is-collapsed="isCollapsed"
      :issue-count="issues.length"
      @toggle-collapse="isCollapsed = !isCollapsed"
      @start-sprint="emit('start-sprint', sprint)"
      @complete-sprint="emit('complete-sprint', sprint)"
      @edit-sprint="emit('edit-sprint', sprint)"
      @delete-sprint="emit('delete-sprint', sprint)"
    />

    <!-- Sprint Issue List Body -->
    <div v-show="!isCollapsed" class="divide-y divide-slate-100">
      <!-- Empty State -->
      <div
        v-if="issues.length === 0"
        class="py-6 px-4 text-center text-xs text-slate-400 italic bg-slate-50/40"
      >
        Tarik tiket dari Backlog ke sini untuk merencanakan pekerjaan sprint ini.
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

