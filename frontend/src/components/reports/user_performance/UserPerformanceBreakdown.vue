<script setup>
import { computed } from 'vue';
import {
  PieChart,
  Layers,
  AlertCircle,
  CheckCircle2,
  Bookmark
} from 'lucide-vue-next';

const props = defineProps({
  byType: {
    type: Array,
    default: () => []
  },
  byPriority: {
    type: Array,
    default: () => []
  }
});

function getTypeColor(type) {
  switch (type?.toUpperCase()) {
    case 'BUG':
      return 'bg-red-500 text-red-700 bg-red-50 border-red-200';
    case 'STORY':
      return 'bg-emerald-500 text-emerald-700 bg-emerald-50 border-emerald-200';
    case 'EPIC':
      return 'bg-purple-500 text-purple-700 bg-purple-50 border-purple-200';
    case 'SUBTASK':
      return 'bg-cyan-500 text-cyan-700 bg-cyan-50 border-cyan-200';
    default:
      return 'bg-blue-500 text-blue-700 bg-blue-50 border-blue-200';
  }
}

function getPriorityColor(prio) {
  switch (prio?.toUpperCase()) {
    case 'HIGHEST':
      return 'text-red-600 bg-red-50 border-red-200';
    case 'HIGH':
      return 'text-amber-600 bg-amber-50 border-amber-200';
    case 'MEDIUM':
      return 'text-blue-600 bg-blue-50 border-blue-200';
    case 'LOW':
    case 'LOWEST':
      return 'text-slate-600 bg-slate-50 border-slate-200';
    default:
      return 'text-slate-600 bg-slate-50 border-slate-200';
  }
}
</script>

<template>
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
    <!-- 1. Breakdown by Issue Type -->
    <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs">
      <div class="flex items-center justify-between pb-3 border-b border-slate-100">
        <div class="flex items-center space-x-2">
          <Layers class="w-4 h-4 text-blue-600" />
          <h3 class="text-sm font-bold text-slate-900">Distribusi Tipe Pekerjaan</h3>
        </div>
        <span class="text-xs text-slate-500 font-medium">Berdasarkan Tipe Tiket</span>
      </div>

      <div class="mt-4 space-y-3">
        <div
          v-for="item in byType"
          :key="item.issue_type"
          class="flex items-center justify-between p-2.5 rounded-lg hover:bg-slate-50 border border-slate-100 transition"
        >
          <div class="flex items-center space-x-3">
            <span
              class="text-xs font-bold px-2 py-0.5 rounded-md border"
              :class="getTypeColor(item.issue_type)"
            >
              {{ item.issue_type }}
            </span>
            <span class="text-xs text-slate-600 font-medium">
              {{ item.total }} tiket
            </span>
          </div>

          <div class="flex items-center space-x-4 text-xs">
            <span class="text-emerald-600 font-bold flex items-center">
              <CheckCircle2 class="w-3.5 h-3.5 mr-1" />
              {{ item.completed }} selesai
            </span>
            <span v-if="item.total_points > 0" class="text-purple-700 font-bold">
              ⚡ {{ item.completed_points }}/{{ item.total_points }} pts
            </span>
          </div>
        </div>

        <div v-if="byType.length === 0" class="py-6 text-center text-xs text-slate-400">
          Belum ada data tipe tiket pada periode ini.
        </div>
      </div>
    </div>

    <!-- 2. Breakdown by Priority -->
    <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs">
      <div class="flex items-center justify-between pb-3 border-b border-slate-100">
        <div class="flex items-center space-x-2">
          <AlertCircle class="w-4 h-4 text-amber-600" />
          <h3 class="text-sm font-bold text-slate-900">Kinerja Berdasarkan Prioritas</h3>
        </div>
        <span class="text-xs text-slate-500 font-medium">Tingkat Urgensi</span>
      </div>

      <div class="mt-4 space-y-3">
        <div
          v-for="item in byPriority"
          :key="item.priority"
          class="flex items-center justify-between p-2.5 rounded-lg hover:bg-slate-50 border border-slate-100 transition"
        >
          <div class="flex items-center space-x-3">
            <span
              class="text-xs font-bold px-2 py-0.5 rounded-md border"
              :class="getPriorityColor(item.priority)"
            >
              {{ item.priority }}
            </span>
            <span class="text-xs text-slate-600 font-medium">
              {{ item.total }} tiket
            </span>
          </div>

          <div class="flex items-center space-x-3 text-xs">
            <span class="text-emerald-600 font-bold flex items-center">
              <CheckCircle2 class="w-3.5 h-3.5 mr-1" />
              {{ item.completed }} selesai
            </span>
            <span class="text-slate-400">
              ({{ item.total > 0 ? Math.round((item.completed / item.total) * 100) : 0 }}%)
            </span>
          </div>
        </div>

        <div v-if="byPriority.length === 0" class="py-6 text-center text-xs text-slate-400">
          Belum ada data prioritas tiket pada periode ini.
        </div>
      </div>
    </div>
  </div>
</template>

