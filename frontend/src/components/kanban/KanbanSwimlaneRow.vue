<script setup>
import { ref, computed } from 'vue';
import { ChevronDown, ChevronRight, Plus } from 'lucide-vue-next';
import KanbanCard from './KanbanCard.vue';

const props = defineProps({
  title: {
    type: String,
    required: true,
  },
  subtitle: {
    type: String,
    default: '',
  },
  badgeColor: {
    type: String,
    default: 'bg-slate-100 text-slate-700',
  },
  avatarUrl: {
    type: String,
    default: '',
  },
  icon: {
    type: Object,
    default: null,
  },
  isExpedite: {
    type: Boolean,
    default: false,
  },
  statuses: {
    type: Array,
    required: true,
  },
  issues: {
    type: Array,
    default: () => [],
  },
  dragOverColumnId: {
    type: [Number, String],
    default: null,
  },
  swimlaneKey: {
    type: String,
    required: true,
  },
});

const emit = defineEmits(['dragover', 'dragleave', 'drop', 'create-issue']);

const isExpanded = ref(true);

function toggleExpanded() {
  isExpanded.value = !isExpanded.value;
}

// Group issues by statusId for this swimlane
const issuesByStatus = computed(() => {
  const map = {};
  props.statuses.forEach((s) => {
    map[s.id] = [];
  });
  props.issues.forEach((issue) => {
    if (map[issue.status_id]) {
      map[issue.status_id].push(issue);
    }
  });
  // Sort by position
  Object.keys(map).forEach((sid) => {
    map[sid].sort((a, b) => (a.position || 0) - (b.position || 0));
  });
  return map;
});

// Story points sum
const totalStoryPoints = computed(() => {
  return props.issues.reduce((sum, i) => sum + (Number(i.story_points) || 0), 0);
});
</script>

<template>
  <div
    class="rounded-2xl border transition-all duration-200 overflow-hidden shadow-2xs mb-5"
    :class="isExpedite
      ? 'bg-rose-50/40 border-rose-300 ring-2 ring-rose-200/50'
      : 'bg-slate-100/60 border-slate-200'"
  >
    <!-- Swimlane Header Bar -->
    <div
      @click="toggleExpanded"
      class="px-4 py-3 cursor-pointer select-none flex items-center justify-between border-b transition-colors"
      :class="isExpedite
        ? 'bg-rose-100/60 border-rose-200 text-rose-900 hover:bg-rose-100'
        : 'bg-white/80 border-slate-200/80 text-slate-800 hover:bg-white'"
    >
      <div class="flex items-center gap-3">
        <!-- Collapse / Expand Icon -->
        <button
          type="button"
          class="p-1 rounded-lg text-slate-500 hover:text-slate-800 hover:bg-slate-200/60 transition"
        >
          <ChevronDown v-if="isExpanded" class="w-4 h-4" />
          <ChevronRight v-else class="w-4 h-4" />
        </button>

        <!-- Avatar / Icon -->
        <div v-if="avatarUrl" class="w-7 h-7 rounded-full overflow-hidden border border-slate-200 shrink-0">
          <img :src="avatarUrl" class="w-full h-full object-cover" alt="" />
        </div>
        <div
          v-else-if="icon"
          class="w-7 h-7 rounded-lg flex items-center justify-center shrink-0"
          :class="isExpedite ? 'bg-rose-500 text-white' : 'bg-slate-200 text-slate-700'"
        >
          <component :is="icon" class="w-4 h-4" />
        </div>

        <!-- Title -->
        <div class="flex items-center gap-2">
          <span class="text-sm font-bold tracking-tight">
            {{ title }}
          </span>
          <span
            v-if="subtitle"
            class="text-xs px-2 py-0.5 rounded-full font-medium"
            :class="badgeColor"
          >
            {{ subtitle }}
          </span>
        </div>
      </div>

      <!-- Right Meta: Issue Count & Points -->
      <div class="flex items-center gap-3 text-xs font-semibold">
        <span
          class="px-2.5 py-0.5 rounded-full font-mono border"
          :class="isExpedite
            ? 'bg-rose-200/80 text-rose-900 border-rose-300'
            : 'bg-white text-slate-700 border-slate-200'"
        >
          {{ issues.length }} tiket
        </span>
        <span
          v-if="totalStoryPoints > 0"
          class="px-2 py-0.5 rounded-md font-mono bg-blue-100 text-blue-800 border border-blue-200"
          title="Total Story Points"
        >
          {{ totalStoryPoints }} pts
        </span>
      </div>
    </div>

    <!-- Swimlane Column Cells (When Expanded) -->
    <div v-if="isExpanded" class="p-3 overflow-x-auto">
      <div class="flex gap-5 items-start">
        <div
          v-for="status in statuses"
          :key="status.id"
          class="w-80 shrink-0 bg-white/70 rounded-xl border border-slate-200/80 p-3 min-h-[120px] flex flex-col space-y-3 transition kanban-column"
          :class="{
            'drag-over': dragOverColumnId === `${swimlaneKey}-${status.id}`,
          }"
          @dragover="(e) => emit('dragover', { event: e, cellKey: `${swimlaneKey}-${status.id}`, statusId: status.id })"
          @dragleave="(e) => emit('dragleave', { event: e, cellKey: `${swimlaneKey}-${status.id}`, statusId: status.id })"
          @drop="(e) => emit('drop', { event: e, statusId: status.id })"
        >
          <!-- Cards -->
          <KanbanCard
            v-for="issue in (issuesByStatus[status.id] || [])"
            :key="issue.id"
            :issue="issue"
          />

          <!-- Empty Cell Placeholder -->
          <div
            v-if="(issuesByStatus[status.id] || []).length === 0"
            class="h-20 border border-dashed border-slate-200 rounded-lg flex items-center justify-center text-slate-400 text-xs select-none"
          >
            <span class="font-medium text-[11px]">Tarik tiket ke sini</span>
          </div>

          <!-- Quick Add in Cell -->
          <button
            @click="emit('create-issue', status.id)"
            class="w-full py-1 text-center text-xs text-slate-400 hover:text-slate-700 hover:bg-slate-100/60 rounded-lg transition font-medium flex items-center justify-center gap-1 mt-auto"
          >
            <Plus class="w-3 h-3" />
            <span>Tambah</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

