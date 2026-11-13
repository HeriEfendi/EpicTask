<script setup>
import { computed } from 'vue';
import { useProjectStore } from '@/stores/project';
import {
  GripVertical,
  Calendar,
  User,
  CheckCircle2,
  Bookmark,
  Bug,
  ClipboardList,
  Layers
} from 'lucide-vue-next';

const props = defineProps({
  issue: {
    type: Object,
    required: true,
  },
});

const projectStore = useProjectStore();

function handleDragStart(e) {
  e.dataTransfer.setData('text/plain', String(props.issue.id));
  e.dataTransfer.effectAllowed = 'move';
}

const typeIcon = computed(() => {
  switch (props.issue.issue_type) {
    case 'BUG':
      return Bug;
    case 'STORY':
      return Bookmark;
    case 'EPIC':
      return Layers;
    default:
      return ClipboardList;
  }
});

const typeColorClass = computed(() => {
  switch (props.issue.issue_type) {
    case 'BUG':
      return 'text-rose-600 bg-rose-50 border-rose-200';
    case 'STORY':
      return 'text-emerald-700 bg-emerald-50 border-emerald-200';
    case 'EPIC':
      return 'text-purple-700 bg-purple-50 border-purple-200';
    default:
      return 'text-blue-700 bg-blue-50 border-blue-200';
  }
});

const labelList = computed(() => {
  if (!props.issue.labels) return [];
  return props.issue.labels
    .split(',')
    .map((l) => l.trim())
    .filter(Boolean);
});
</script>

<template>
  <div
    draggable="true"
    @dragstart="handleDragStart"
    @click="projectStore.openIssueDetail(issue.id)"
    class="flex items-center justify-between px-3.5 py-2.5 bg-white hover:bg-slate-50 border-b border-slate-100 last:border-b-0 cursor-pointer group transition-colors select-none text-sm"
  >
    <!-- Left side: Drag handle, Type, Key, Title, Tags -->
    <div class="flex items-center space-x-2.5 min-w-0 flex-1 pr-4">
      <!-- Drag handle -->
      <div class="cursor-grab active:cursor-grabbing text-slate-300 group-hover:text-slate-500 transition-colors shrink-0">
        <GripVertical class="w-4 h-4" />
      </div>

      <!-- Issue Type Icon -->
      <div
        class="w-6 h-6 rounded flex items-center justify-center shrink-0 border text-xs"
        :class="typeColorClass"
        :title="issue.issue_type"
      >
        <component :is="typeIcon" class="w-3.5 h-3.5" />
      </div>

      <!-- Issue Key -->
      <span class="font-mono text-xs font-bold text-slate-600 group-hover:text-blue-600 transition-colors shrink-0">
        {{ issue.key }}
      </span>

      <!-- Summary -->
      <span class="font-medium text-slate-800 group-hover:text-blue-600 truncate transition-colors">
        {{ issue.summary }}
      </span>

      <!-- Epic Tag -->
      <span
        v-if="issue.epic_summary && issue.issue_type !== 'EPIC'"
        class="hidden sm:inline-flex items-center text-2xs font-semibold px-2 py-0.5 rounded-full bg-purple-50 text-purple-700 border border-purple-200 max-w-[140px] truncate shrink-0"
      >
        🟣 {{ issue.epic_summary }}
      </span>

      <!-- Component Tag -->
      <span
        v-if="issue.component_name"
        class="hidden md:inline-flex items-center text-2xs font-semibold px-1.5 py-0.5 rounded bg-cyan-50 text-cyan-800 border border-cyan-200 max-w-[120px] truncate shrink-0"
      >
        🧩 {{ issue.component_name }}
      </span>

      <!-- Labels Tag -->
      <span
        v-for="lbl in labelList"
        :key="lbl"
        class="hidden lg:inline-flex items-center text-2xs font-semibold px-1.5 py-0.5 rounded bg-slate-100 text-slate-700 border border-slate-200 shrink-0"
      >
        🏷️ {{ lbl }}
      </span>
    </div>

    <!-- Right side: Status, Priority, Points, Assignee -->
    <div class="flex items-center space-x-2.5 shrink-0">
      <!-- Status Pill -->
      <span
        class="text-xs font-semibold px-2 py-0.5 rounded uppercase border"
        :style="{
          backgroundColor: `${issue.status_color || '#64748b'}15`,
          color: issue.status_color || '#475569',
          borderColor: `${issue.status_color || '#64748b'}40`,
        }"
      >
        {{ issue.status_name || issue.status_category }}
      </span>

      <!-- Priority Indicator -->
      <span
        v-if="issue.priority"
        class="text-xs px-1.5 py-0.5 rounded font-bold"
        :class="`badge-priority-${issue.priority}`"
        :title="`Prioritas: ${issue.priority}`"
      >
        <span v-if="issue.priority === 'HIGHEST' || issue.priority === 'HIGH'">↑</span>
        <span v-else-if="issue.priority === 'LOW' || issue.priority === 'LOWEST'">↓</span>
        <span v-else>•</span>
      </span>

      <!-- Story Points Badge -->
      <span
        v-if="issue.story_points > 0"
        class="w-6 h-6 rounded-full bg-slate-100 border border-slate-200 text-slate-700 text-xs font-mono font-bold flex items-center justify-center shrink-0"
        title="Story Points"
      >
        {{ issue.story_points }}
      </span>
      <span
        v-else
        class="w-6 h-6 rounded-full bg-slate-50 border border-dashed border-slate-200 text-slate-400 text-xs font-mono flex items-center justify-center shrink-0"
        title="Belum diestimasi"
      >
        -
      </span>

      <!-- Assignee Avatar -->
      <div class="w-6 h-6 rounded-full bg-slate-200 flex items-center justify-center text-xs font-bold text-slate-700 border border-white shadow-2xs shrink-0" :title="issue.assignee_name || 'Belum Ditugaskan'">
        <img
          v-if="issue.assignee_avatar"
          :src="issue.assignee_avatar"
          class="w-full h-full rounded-full object-cover"
        />
        <span v-else-if="issue.assignee_name">
          {{ issue.assignee_name.charAt(0).toUpperCase() }}
        </span>
        <User v-else class="w-3.5 h-3.5 text-slate-400" />
      </div>
    </div>
  </div>
</template>

