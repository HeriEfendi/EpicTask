<script setup>
import { computed } from 'vue';
import { useProjectStore } from '@/stores/project';
import {
  CheckSquare,
  Calendar
} from 'lucide-vue-next';

const props = defineProps({
  issue: {
    type: Object,
    required: true,
  },
});

const projectStore = useProjectStore();

const isOverdue = computed(() => {
  if (!props.issue.due_date) return false;
  if (props.issue.status_category === 'DONE') return false;
  const due = new Date(props.issue.due_date);
  const today = new Date();
  today.setHours(0, 0, 0, 0);
  return due < today;
});

const isDueToday = computed(() => {
  if (!props.issue.due_date) return false;
  if (props.issue.status_category === 'DONE') return false;
  const due = new Date(props.issue.due_date);
  const today = new Date();
  return (
    due.getFullYear() === today.getFullYear() &&
    due.getMonth() === today.getMonth() &&
    due.getDate() === today.getDate()
  );
});

const subtaskProgress = computed(() => {
  const total = props.issue.subtask_count || 0;
  const done = props.issue.subtask_done_count || 0;
  if (total === 0) return null;
  const percent = Math.round((done / total) * 100);
  return { total, done, percent };
});

function handleDragStart(e) {
  e.dataTransfer.setData('text/plain', String(props.issue.id));
  e.dataTransfer.effectAllowed = 'move';
  e.target.classList.add('dragging-card');
}

function handleDragEnd(e) {
  e.target.classList.remove('dragging-card');
}
</script>

<template>
  <div
    draggable="true"
    @dragstart="handleDragStart"
    @dragend="handleDragEnd"
    @click="projectStore.openIssueDetail(issue.id)"
    class="bg-white hover:bg-slate-50/90 border border-slate-200/90 hover:border-slate-300 rounded-xl p-3.5 cursor-grab active:cursor-grabbing transition-card group select-none shadow-2xs hover:shadow-md relative"
  >
    <!-- Epic Pill (if belongs to an Epic) -->
    <div v-if="issue.epic_summary && issue.issue_type !== 'EPIC'" class="mb-2">
      <span class="inline-flex items-center text-xs font-semibold px-2.5 py-0.5 rounded-full bg-purple-50 text-purple-700 border border-purple-200 max-w-[220px] truncate">
        🟣 {{ issue.epic_summary }}
      </span>
    </div>

    <!-- Title / Summary (Larger font: text-sm font-semibold) -->
    <h4 class="text-sm font-semibold text-slate-800 group-hover:text-blue-600 line-clamp-2 leading-relaxed mb-3 transition-colors">
      {{ issue.summary }}
    </h4>

    <!-- Subtasks Progress Bar (FR-4.4) -->
    <div v-if="subtaskProgress" class="mb-3 bg-slate-50 rounded-lg p-2 border border-slate-200">
      <div class="flex items-center justify-between text-xs text-slate-600 mb-1.5">
        <span class="flex items-center gap-1 font-medium">
          <CheckSquare class="w-3.5 h-3.5 text-emerald-600" /> Subtask
        </span>
        <span class="font-mono text-slate-700 font-semibold">{{ subtaskProgress.done }}/{{ subtaskProgress.total }} ({{ subtaskProgress.percent }}%)</span>
      </div>
      <div class="w-full bg-slate-200 h-2 rounded-full overflow-hidden">
        <div
          class="h-full transition-all duration-300 rounded-full"
          :class="subtaskProgress.percent === 100 ? 'bg-emerald-500' : 'bg-blue-600'"
          :style="{ width: `${subtaskProgress.percent}%` }"
        ></div>
      </div>
    </div>

    <!-- Bottom Footer Meta (Key, Type, Priority, Date, Avatar) -->
    <div class="flex items-center justify-between pt-2 border-t border-slate-100 text-slate-500">
      <div class="flex items-center space-x-2">
        <!-- Issue Type Icon -->
        <span
          class="text-xs font-semibold px-2 py-0.5 rounded uppercase"
          :class="`badge-type-${issue.issue_type}`"
          :title="issue.issue_type"
        >
          {{ issue.issue_type }}
        </span>

        <!-- Issue Key -->
        <span class="text-xs font-mono font-semibold text-slate-600 group-hover:text-blue-600 transition">
          {{ issue.key }}
        </span>

        <!-- Priority Indicator -->
        <span
          v-if="issue.priority"
          class="text-xs px-1.5 py-0.5 rounded font-bold"
          :class="`badge-priority-${issue.priority}`"
          :title="`Priority: ${issue.priority}`"
        >
          <span v-if="issue.priority === 'HIGHEST' || issue.priority === 'HIGH'">↑</span>
          <span v-else-if="issue.priority === 'LOW' || issue.priority === 'LOWEST'">↓</span>
          <span v-else>•</span>
        </span>
      </div>

      <div class="flex items-center space-x-2.5">
        <!-- Due Date Tag -->
        <span
          v-if="issue.due_date"
          class="flex items-center space-x-1 text-xs font-medium px-2 py-0.5 rounded"
          :class="isOverdue ? 'bg-red-50 text-red-600 border border-red-200 animate-pulse' : (isDueToday ? 'bg-amber-50 text-amber-700 border border-amber-200' : 'text-slate-600')"
          :title="`Tenggat: ${issue.due_date}`"
        >
          <Calendar class="w-3 h-3" />
          <span>{{ issue.due_date }}</span>
        </span>

        <!-- Story Points -->
        <span
          v-if="issue.story_points"
          class="text-xs font-mono font-bold bg-slate-100 border border-slate-200 text-slate-700 px-2 py-0.5 rounded-full"
          title="Story Points"
        >
          {{ issue.story_points }}
        </span>

        <!-- Assignee Avatar -->
        <img
          v-if="issue.assignee_avatar"
          :src="issue.assignee_avatar"
          :alt="issue.assignee_name || 'Assignee'"
          :title="issue.assignee_name"
          class="w-6 h-6 rounded-full ring-1 ring-slate-200 bg-slate-100"
        />
        <div
          v-else
          class="w-6 h-6 rounded-full bg-slate-100 border border-dashed border-slate-300 flex items-center justify-center text-xs text-slate-400 font-bold"
          title="Belum ditugaskan"
        >
          ?
        </div>
      </div>
    </div>
  </div>
</template>
