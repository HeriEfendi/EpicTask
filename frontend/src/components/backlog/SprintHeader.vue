<script setup>
import { ref } from 'vue';
import {
  ChevronDown,
  ChevronRight,
  Play,
  CheckCircle,
  MoreVertical,
  Calendar,
  Target,
  Edit2,
  Trash2
} from 'lucide-vue-next';

const props = defineProps({
  sprint: {
    type: Object,
    required: true,
  },
  isCollapsed: {
    type: Boolean,
    default: false,
  },
  issueCount: {
    type: Number,
    default: 0,
  },
});

const emit = defineEmits([
  'toggle-collapse',
  'start-sprint',
  'complete-sprint',
  'edit-sprint',
  'delete-sprint',
]);

const isMenuOpen = ref(false);

function closeMenu() {
  isMenuOpen.value = false;
}
</script>

<template>
  <div class="px-4 py-3 bg-slate-100/90 border-b border-slate-200 flex flex-wrap items-center justify-between gap-2 select-none">
    <!-- Left: Collapse toggle, Sprint title, Status badge, Date range, Goal -->
    <div class="flex items-center space-x-2.5 min-w-0 flex-1">
      <!-- Collapse toggle button -->
      <button
        @click="emit('toggle-collapse')"
        class="text-slate-500 hover:text-slate-800 p-0.5 rounded transition"
        :title="isCollapsed ? 'Buka Sprint' : 'Tutup Sprint'"
      >
        <ChevronDown v-if="!isCollapsed" class="w-4 h-4" />
        <ChevronRight v-else class="w-4 h-4" />
      </button>

      <!-- Sprint Title -->
      <span class="font-bold text-sm text-slate-900 truncate">
        {{ sprint.name }}
      </span>

      <!-- Status Badge -->
      <span
        v-if="sprint.status === 'ACTIVE'"
        class="inline-flex items-center gap-1 text-xs font-bold px-2 py-0.5 rounded-full bg-emerald-100 text-emerald-800 border border-emerald-300 animate-pulse"
      >
        <span class="w-1.5 h-1.5 rounded-full bg-emerald-600"></span>
        Aktif
      </span>
      <span
        v-else-if="sprint.status === 'COMPLETED'"
        class="inline-flex items-center text-xs font-semibold px-2 py-0.5 rounded-full bg-slate-200 text-slate-700"
      >
        Selesai
      </span>
      <span
        v-else
        class="inline-flex items-center text-xs font-semibold px-2 py-0.5 rounded-full bg-blue-50 text-blue-700 border border-blue-200"
      >
        Direncanakan
      </span>

      <!-- Date Range (if set) -->
      <div v-if="sprint.start_date && sprint.end_date" class="hidden sm:flex items-center space-x-1 text-xs text-slate-500 font-medium">
        <Calendar class="w-3 h-3 text-slate-400" />
        <span>{{ sprint.start_date }} — {{ sprint.end_date }}</span>
      </div>

      <!-- Goal Snippet (if set) -->
      <div v-if="sprint.goal" class="hidden lg:flex items-center space-x-1 text-xs text-slate-600 italic truncate max-w-xs">
        <Target class="w-3 h-3 text-indigo-500 shrink-0" />
        <span class="truncate">"{{ sprint.goal }}"</span>
      </div>

      <!-- Issue count -->
      <span class="text-xs text-slate-500 font-semibold">
        ({{ issueCount }} isu)
      </span>
    </div>

    <!-- Right: Jira 3-Pill Story Points Rollup & Action Buttons -->
    <div class="flex items-center space-x-3 shrink-0">
      <!-- Jira Story Point Estimation Badges (To Do, In Progress, Done) -->
      <div class="flex items-center space-x-1 text-xs font-mono font-bold" title="Estimasi Poin: Abu (To Do) | Biru (In Progress) | Hijau (Done)">
        <!-- To Do points (Grey) -->
        <span class="px-2 py-0.5 rounded-full bg-slate-200 text-slate-700 border border-slate-300">
          {{ sprint.todo_points || 0 }}
        </span>
        <!-- In Progress points (Blue) -->
        <span class="px-2 py-0.5 rounded-full bg-blue-100 text-blue-800 border border-blue-300">
          {{ sprint.in_progress_points || 0 }}
        </span>
        <!-- Done points (Green) -->
        <span class="px-2 py-0.5 rounded-full bg-emerald-100 text-emerald-800 border border-emerald-300">
          {{ sprint.done_points || 0 }}
        </span>
      </div>

      <!-- Sprint Action Button: Mulai Sprint or Selesaikan Sprint -->
      <button
        v-if="sprint.status === 'PLANNED'"
        @click="emit('start-sprint')"
        class="flex items-center space-x-1 px-3 py-1 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-lg text-xs font-bold transition shadow-2xs"
      >
        <Play class="w-3 h-3 fill-current" />
        <span>Mulai Sprint</span>
      </button>

      <button
        v-else-if="sprint.status === 'ACTIVE'"
        @click="emit('complete-sprint')"
        class="flex items-center space-x-1 px-3 py-1 bg-emerald-600 hover:bg-emerald-700 active:bg-emerald-800 text-white rounded-lg text-xs font-bold transition shadow-2xs"
      >
        <CheckCircle class="w-3.5 h-3.5" />
        <span>Selesaikan Sprint</span>
      </button>

      <!-- More Menu Dropdown -->
      <div class="relative">
        <button
          @click="isMenuOpen = !isMenuOpen"
          class="p-1 rounded-lg hover:bg-slate-200 text-slate-500 hover:text-slate-800 transition"
        >
          <MoreVertical class="w-4 h-4" />
        </button>

        <div
          v-if="isMenuOpen"
          class="absolute right-0 mt-1 w-44 glass-dropdown rounded-xl p-1.5 z-40 shadow-xl"
        >
          <button
            @click="closeMenu(); emit('edit-sprint')"
            class="w-full text-left px-2.5 py-1.5 rounded-lg text-xs text-slate-700 hover:bg-slate-100 flex items-center space-x-2 transition"
          >
            <Edit2 class="w-3.5 h-3.5 text-slate-500" />
            <span>Edit Sprint</span>
          </button>
          <button
            @click="closeMenu(); emit('delete-sprint')"
            class="w-full text-left px-2.5 py-1.5 rounded-lg text-xs text-rose-600 hover:bg-rose-50 flex items-center space-x-2 transition"
          >
            <Trash2 class="w-3.5 h-3.5 text-rose-500" />
            <span>Hapus Sprint</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

