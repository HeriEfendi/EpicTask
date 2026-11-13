<script setup>
import { computed } from 'vue';
import { useProjectStore } from '@/stores/project';
import {
  User,
  Bug,
  Flame,
  UserX,
  Calendar,
  CircleDot,
  X,
  SlidersHorizontal,
  Check
} from 'lucide-vue-next';

const projectStore = useProjectStore();

const QUICK_FILTER_DEFINITIONS = [
  {
    key: 'my_issues',
    label: 'Tiket Saya',
    icon: User,
    activeClass: 'bg-blue-50 text-blue-700 border-blue-300 ring-1 ring-blue-300',
    tooltip: 'Tampilkan tiket yang ditugaskan ke saya',
  },
  {
    key: 'only_bugs',
    label: 'Hanya Bug',
    icon: Bug,
    activeClass: 'bg-rose-50 text-rose-700 border-rose-300 ring-1 ring-rose-300',
    tooltip: 'Hanya tampilkan isu bertipe Bug',
  },
  {
    key: 'highest_priority',
    label: 'Prioritas Tinggi',
    icon: Flame,
    activeClass: 'bg-amber-50 text-amber-700 border-amber-300 ring-1 ring-amber-300',
    tooltip: 'Tiket prioritas Highest atau High',
  },
  {
    key: 'unassigned',
    label: 'Belum Ditugaskan',
    icon: UserX,
    activeClass: 'bg-slate-100 text-slate-700 border-slate-300 ring-1 ring-slate-300',
    tooltip: 'Tiket yang belum memiliki assignee',
  },
  {
    key: 'due_this_week',
    label: 'Due Minggu Ini',
    icon: Calendar,
    activeClass: 'bg-purple-50 text-purple-700 border-purple-300 ring-1 ring-purple-300',
    tooltip: 'Tiket dengan tenggat waktu dalam minggu berjalan',
  },
  {
    key: 'unresolved',
    label: 'Belum Selesai',
    icon: CircleDot,
    activeClass: 'bg-emerald-50 text-emerald-700 border-emerald-300 ring-1 ring-emerald-300',
    tooltip: 'Tiket yang belum memiliki resolusi atau belum Done',
  },
];

const hasActiveQuickFilters = computed(() => projectStore.quickFilters.length > 0);
</script>

<template>
  <div class="flex items-center gap-1.5 flex-wrap">
    <span class="text-xs font-semibold text-slate-500 flex items-center gap-1 mr-1">
      <SlidersHorizontal class="w-3.5 h-3.5 text-slate-400" />
      <span>Quick Filters:</span>
    </span>

    <!-- Quick Filter Pills (Multi-Selectable) -->
    <button
      v-for="qf in QUICK_FILTER_DEFINITIONS"
      :key="qf.key"
      type="button"
      @click="projectStore.toggleQuickFilter(qf.key)"
      :title="qf.tooltip"
      class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-xs font-semibold border transition shadow-2xs select-none cursor-pointer"
      :class="[
        projectStore.isQuickFilterActive(qf.key)
          ? qf.activeClass
          : 'bg-white hover:bg-slate-50 text-slate-600 border-slate-200/90 hover:border-slate-300'
      ]"
    >
      <component :is="qf.icon" class="w-3.5 h-3.5 shrink-0" />
      <span>{{ qf.label }}</span>
      <Check
        v-if="projectStore.isQuickFilterActive(qf.key)"
        class="w-3 h-3 stroke-[3]"
      />
    </button>

    <!-- Clear / Reset Quick Filters Button -->
    <button
      v-if="hasActiveQuickFilters"
      type="button"
      @click="projectStore.clearQuickFilters"
      title="Reset semua Quick Filter"
      class="inline-flex items-center gap-1 px-2 py-1 rounded-lg text-xs font-bold text-red-600 hover:text-red-700 bg-red-50/70 hover:bg-red-100 border border-red-200 transition shadow-2xs cursor-pointer ml-1 animate-in fade-in duration-150"
    >
      <X class="w-3 h-3" />
      <span>Reset</span>
    </button>
  </div>
</template>
