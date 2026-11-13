<script setup>
import { computed } from 'vue';
import {
  CheckCircle2,
  Zap,
  Clock,
  Gauge,
  Flame,
  ArrowUpRight
} from 'lucide-vue-next';

const props = defineProps({
  stats: {
    type: Object,
    default: () => ({})
  }
});

function formatNumber(n, decimals = 1) {
  if (n === undefined || n === null) return '0';
  return Number(n).toFixed(decimals);
}

function formatCycleTime(hours, days) {
  if (!hours || hours <= 0) return '0 jam';
  if (days && days >= 1) {
    return `${days.toFixed(1)} hari (${hours.toFixed(0)} jam)`;
  }
  return `${hours.toFixed(1)} jam`;
}
</script>

<template>
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
    <!-- Card 1: Completed Issues -->
    <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs flex flex-col justify-between">
      <div class="flex items-center justify-between text-slate-500 mb-2">
        <span class="text-xs font-bold uppercase tracking-wider">Tiket Selesai (Done)</span>
        <div class="w-8 h-8 rounded-lg bg-emerald-50 text-emerald-600 flex items-center justify-center">
          <CheckCircle2 class="w-4 h-4" />
        </div>
      </div>
      <div>
        <div class="flex items-baseline space-x-2">
          <span class="text-3xl font-extrabold text-slate-900">
            {{ stats.completed_count || 0 }}
          </span>
          <span class="text-xs font-medium text-slate-500">
            dari {{ stats.total_assigned || 0 }} tiket ditugaskan
          </span>
        </div>

        <!-- Progress bar -->
        <div class="w-full bg-slate-100 rounded-full h-2 mt-3 overflow-hidden">
          <div
            class="bg-emerald-500 h-2 rounded-full transition-all duration-500"
            :style="{ width: `${Math.min(100, Math.max(0, stats.completion_rate || 0))}%` }"
          ></div>
        </div>

        <div class="flex items-center justify-between mt-2 text-xs">
          <span class="text-emerald-700 font-bold">
            {{ formatNumber(stats.completion_rate, 1) }}% Selesai
          </span>
          <span class="text-slate-400">
            {{ stats.in_progress_count || 0 }} aktif, {{ stats.todo_count || 0 }} todo
          </span>
        </div>
      </div>
    </div>

    <!-- Card 2: Story Points Completed -->
    <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs flex flex-col justify-between">
      <div class="flex items-center justify-between text-slate-500 mb-2">
        <span class="text-xs font-bold uppercase tracking-wider">Story Points Terkirim</span>
        <div class="w-8 h-8 rounded-lg bg-purple-50 text-purple-600 flex items-center justify-center">
          <Zap class="w-4 h-4" />
        </div>
      </div>
      <div>
        <div class="flex items-baseline space-x-2">
          <span class="text-3xl font-extrabold text-slate-900">
            {{ stats.completed_story_points || 0 }}
          </span>
          <span class="text-xs font-medium text-slate-500">
            / {{ stats.total_story_points || 0 }} total poin
          </span>
        </div>

        <!-- Progress bar -->
        <div class="w-full bg-slate-100 rounded-full h-2 mt-3 overflow-hidden">
          <div
            class="bg-purple-600 h-2 rounded-full transition-all duration-500"
            :style="{ width: `${Math.min(100, Math.max(0, stats.points_completion_rate || 0))}%` }"
          ></div>
        </div>

        <div class="flex items-center justify-between mt-2 text-xs">
          <span class="text-purple-700 font-bold">
            {{ formatNumber(stats.points_completion_rate, 1) }}% Delivery Rate
          </span>
          <span class="text-slate-400">Agile Story Estimation</span>
        </div>
      </div>
    </div>

    <!-- Card 3: Total Logged Hours -->
    <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs flex flex-col justify-between">
      <div class="flex items-center justify-between text-slate-500 mb-2">
        <span class="text-xs font-bold uppercase tracking-wider">Jam Kerja Tercatat</span>
        <div class="w-8 h-8 rounded-lg bg-blue-50 text-blue-600 flex items-center justify-center">
          <Clock class="w-4 h-4" />
        </div>
      </div>
      <div>
        <div class="flex items-baseline space-x-2">
          <span class="text-3xl font-extrabold text-blue-600 font-mono">
            {{ formatNumber(stats.total_logged_hours, 1) }}h
          </span>
          <span class="text-xs font-medium text-slate-500">
            ({{ formatNumber((stats.total_logged_hours || 0) / 8, 1) }} hari kerja)
          </span>
        </div>

        <div class="mt-3 text-xs text-slate-500 flex items-center justify-between">
          <span>Total Entri Timesheet:</span>
          <span class="font-bold text-slate-800">{{ stats.total_worklogs || 0 }} log</span>
        </div>

        <div class="mt-2 text-[11px] text-slate-400 flex items-center space-x-1">
          <Flame class="w-3.5 h-3.5 text-amber-500 shrink-0" />
          <span>Berdasarkan pencatatan worklog real-time</span>
        </div>
      </div>
    </div>

    <!-- Card 4: Average Cycle / Resolution Time -->
    <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs flex flex-col justify-between">
      <div class="flex items-center justify-between text-slate-500 mb-2">
        <span class="text-xs font-bold uppercase tracking-wider">Rata-rata Siklus Tiket</span>
        <div class="w-8 h-8 rounded-lg bg-amber-50 text-amber-600 flex items-center justify-center">
          <Gauge class="w-4 h-4" />
        </div>
      </div>
      <div>
        <div class="text-2xl font-extrabold text-slate-900 tracking-tight">
          {{ formatCycleTime(stats.avg_cycle_hours, stats.avg_cycle_days) }}
        </div>
        <p class="text-xs text-slate-500 mt-1">
          Durasi rata-rata dari tiket dibuat sampai terselesaikan (Done).
        </p>

        <div class="mt-3 flex items-center space-x-1 text-xs text-slate-500">
          <span class="inline-block w-2 h-2 rounded-full bg-emerald-500"></span>
          <span>Kecepatan Resolusi: Sesuai SLA</span>
        </div>
      </div>
    </div>
  </div>
</template>

