<script setup>
import { useReportStore } from '@/stores/report';

const reportStore = useReportStore();
</script>

<template>
  <div class="space-y-6">
    <div class="bg-white rounded-xl p-6 border border-slate-200/90 shadow-2xs">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-4 border-b border-slate-100 gap-3">
        <div>
          <div class="flex items-center space-x-2">
            <h2 class="text-base font-bold text-slate-900">Cumulative Flow Diagram (CFD)</h2>
            <span class="text-xs bg-blue-100 text-blue-800 px-2 py-0.5 rounded font-semibold">Kanban Health</span>
          </div>
          <p class="text-xs text-slate-500 mt-1">
            Visualisasi aliran kumulatif status issue dari Jan 2026 s.d. Okt 2026. Mengidentifikasi penumpukan bottleneck dan stabilitas alur kerja tim.
          </p>
        </div>

        <!-- CFD Legend -->
        <div class="flex items-center flex-wrap gap-3 text-xs font-semibold">
          <div class="flex items-center space-x-1.5"><span class="w-3 h-3 bg-emerald-500 rounded-sm"></span><span>Done</span></div>
          <div class="flex items-center space-x-1.5"><span class="w-3 h-3 bg-purple-500 rounded-sm"></span><span>In Review</span></div>
          <div class="flex items-center space-x-1.5"><span class="w-3 h-3 bg-amber-500 rounded-sm"></span><span>In Progress</span></div>
          <div class="flex items-center space-x-1.5"><span class="w-3 h-3 bg-blue-500 rounded-sm"></span><span>To Do</span></div>
          <div class="flex items-center space-x-1.5"><span class="w-3 h-3 bg-slate-400 rounded-sm"></span><span>Backlog</span></div>
        </div>
      </div>

      <!-- CFD Visual Representation (Stacked Progress Bars over Months) -->
      <div class="mt-6 space-y-3">
        <div
          v-for="p in reportStore.cfd?.timeline"
          :key="p.month"
          class="space-y-1"
        >
          <div class="flex items-center justify-between text-xs font-semibold">
            <span class="text-slate-800 font-mono">{{ p.month }}</span>
            <span class="text-slate-500 text-[11px]">
              Total Kumulatif: <strong class="text-slate-800">{{ p.cumulative_done }} Selesai</strong> / {{ p.cumulative_total }}
            </span>
          </div>

          <!-- Multi-band Stacked Bar -->
          <div class="h-6 w-full bg-slate-100 rounded-md overflow-hidden flex shadow-2xs">
            <div
              class="bg-emerald-500 transition-all duration-300"
              :style="{ width: `${(p.done / (p.total || 1)) * 100}%` }"
              :title="`Done: ${p.done}`"
            ></div>
            <div
              class="bg-purple-500 transition-all duration-300"
              :style="{ width: `${(p.in_review / (p.total || 1)) * 100}%` }"
              :title="`In Review: ${p.in_review}`"
            ></div>
            <div
              class="bg-amber-500 transition-all duration-300"
              :style="{ width: `${(p.in_progress / (p.total || 1)) * 100}%` }"
              :title="`In Progress: ${p.in_progress}`"
            ></div>
            <div
              class="bg-blue-500 transition-all duration-300"
              :style="{ width: `${(p.todo / (p.total || 1)) * 100}%` }"
              :title="`To Do: ${p.todo}`"
            ></div>
            <div
              class="bg-slate-400 transition-all duration-300"
              :style="{ width: `${(p.backlog / (p.total || 1)) * 100}%` }"
              :title="`Backlog: ${p.backlog}`"
            ></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

