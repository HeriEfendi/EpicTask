<script setup>
import { useProjectStore } from '@/stores/project';
import {
  Kanban,
  Rows3,
  User,
  FolderGit2,
  Flame,
  ChevronDown,
  Plus
} from 'lucide-vue-next';

const projectStore = useProjectStore();

const swimlaneOptions = [
  { id: 'none',     label: 'Tanpa Swimlane', icon: Kanban },
  { id: 'assignee', label: 'Berdasarkan Assignee', icon: User },
  { id: 'epic',     label: 'Berdasarkan Epic', icon: FolderGit2 },
  { id: 'priority', label: 'Berdasarkan Prioritas (Expedite)', icon: Flame },
];
</script>

<template>
  <div class="px-6 py-2.5 bg-white border-b border-slate-200/80 flex items-center justify-between gap-4 select-none">
    <!-- Left: Title & Board Statuses Summary -->
    <div class="flex items-center gap-3">
      <div class="flex items-center gap-2">
        <span class="text-xs font-bold text-slate-500 uppercase tracking-wider">
          Kanban Board
        </span>
        <span class="text-xs font-mono font-bold px-2 py-0.5 rounded-full bg-slate-100 text-slate-700 border border-slate-200">
          {{ projectStore.filteredIssues.length }} tiket
        </span>
      </div>
    </div>

    <!-- Right: Swimlane Switcher Dropdown (Jira Standard) -->
    <div class="flex items-center gap-3">
      <div class="flex items-center gap-2">
        <span class="text-xs font-semibold text-slate-500 flex items-center gap-1.5">
          <Rows3 class="w-3.5 h-3.5 text-slate-400" />
          <span>Swimlane:</span>
        </span>

        <div class="relative">
          <select
            v-model="projectStore.swimlaneMode"
            class="bg-white border border-slate-300 hover:border-slate-400 rounded-lg pl-3 pr-7 py-1.5 text-xs text-slate-800 focus:outline-none focus:border-blue-600 appearance-none cursor-pointer shadow-2xs font-semibold transition"
            :class="projectStore.swimlaneMode !== 'none' ? 'border-blue-500 bg-blue-50/50 text-blue-900 font-bold' : ''"
          >
            <option
              v-for="opt in swimlaneOptions"
              :key="opt.id"
              :value="opt.id"
            >
              {{ opt.label }}
            </option>
          </select>
          <ChevronDown class="w-3.5 h-3.5 text-slate-400 absolute right-2 top-1/2 -translate-y-1/2 pointer-events-none" />
        </div>
      </div>
    </div>
  </div>
</template>
