<script setup>
import { computed } from 'vue';
import {
  CheckCircle2,
  ShieldCheck,
  AlertTriangle,
  Award,
  Clock,
  TrendingUp,
  Percent
} from 'lucide-vue-next';

const props = defineProps({
  summary: {
    type: Object,
    default: () => ({
      done_count: 0,
      on_time_count: 0,
      overdue_count: 0,
      on_time_rate: 0
    })
  }
});

const doneCount = computed(() => props.summary?.done_count || 0);
const onTimeCount = computed(() => props.summary?.on_time_count || 0);
const overdueCount = computed(() => props.summary?.overdue_count || 0);
const onTimeRate = computed(() => Number(props.summary?.on_time_rate || 0));

const statusBadge = computed(() => {
  if (doneCount.value === 0) {
    return {
      text: 'Belum Ada Tiket Selesai',
      class: 'bg-slate-50 text-slate-600 border-slate-200'
    };
  }
  if (onTimeRate.value >= 85) {
    return {
      text: '✓ Performa Unggul (SLA Tercapai)',
      class: 'bg-emerald-50 text-emerald-700 border-emerald-200'
    };
  }
  if (onTimeRate.value >= 70) {
    return {
      text: '⚠️ Performa Standar',
      class: 'bg-amber-50 text-amber-700 border-amber-200'
    };
  }
  return {
    text: '🚨 Tingkat Keterlambatan Tinggi',
    class: 'bg-rose-50 text-rose-700 border-rose-200'
  };
});
</script>

<template>
  <div class="bg-white rounded-xl border border-slate-200/90 shadow-2xs p-5">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-4 border-b border-slate-100 gap-3">
      <div class="flex items-center space-x-3">
        <div class="w-9 h-9 rounded-lg bg-indigo-50 text-indigo-600 flex items-center justify-center shrink-0">
          <Award class="w-5 h-5" />
        </div>
        <div>
          <div class="flex items-center space-x-2">
            <h3 class="text-sm font-bold text-slate-900 tracking-tight">Performance Summary</h3>
            <span class="text-[10px] font-bold px-2 py-0.5 rounded-full uppercase tracking-wider bg-indigo-100 text-indigo-700 border border-indigo-200">
              Evaluasi Ketepatan Waktu
            </span>
          </div>
          <p class="text-xs text-slate-500 mt-0.5">
            Menghitung seluruh kartu yang diselesaikan (Done) dalam periode ini berdasarkan target tanggal (Due Date).
          </p>
        </div>
      </div>

      <!-- SLA Status Badge -->
      <div>
        <span
          class="inline-flex items-center text-xs font-bold px-3 py-1 rounded-full border shadow-2xs"
          :class="statusBadge.class"
        >
          {{ statusBadge.text }}
        </span>
      </div>
    </div>

    <!-- 3 Key Metric Cards -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-5">
      <!-- 1. Total Done -->
      <div class="bg-slate-50/70 rounded-xl p-4 border border-slate-200/80 flex items-center justify-between">
        <div>
          <span class="text-xs font-bold uppercase tracking-wider text-slate-500 block mb-1">
            Total Kartu Selesai (Done)
          </span>
          <div class="flex items-baseline space-x-1.5">
            <span class="text-3xl font-extrabold text-slate-900 font-mono">
              {{ doneCount }}
            </span>
            <span class="text-xs text-slate-500 font-medium">tiket</span>
          </div>
          <p class="text-[11px] text-slate-500 mt-1">
            Total kartu berstatus Done di periode ini
          </p>
        </div>
        <div class="w-10 h-10 rounded-xl bg-emerald-100/80 text-emerald-600 flex items-center justify-center shrink-0">
          <CheckCircle2 class="w-5 h-5" />
        </div>
      </div>

      <!-- 2. Selesai Sesuai Target (On-Time) -->
      <div class="bg-slate-50/70 rounded-xl p-4 border border-slate-200/80 flex items-center justify-between">
        <div>
          <span class="text-xs font-bold uppercase tracking-wider text-slate-500 block mb-1">
            Selesai Sesuai Target (On-Time)
          </span>
          <div class="flex items-baseline space-x-1.5">
            <span class="text-3xl font-extrabold text-blue-600 font-mono">
              {{ onTimeCount }}
            </span>
            <span class="text-xs text-slate-500 font-medium">tiket</span>
          </div>
          <p class="text-[11px] text-slate-500 mt-1">
            Terselesaikan tepat waktu / sebelum tenggat
          </p>
        </div>
        <div class="w-10 h-10 rounded-xl bg-blue-100/80 text-blue-600 flex items-center justify-center shrink-0">
          <ShieldCheck class="w-5 h-5" />
        </div>
      </div>

      <!-- 3. Terlambat (Overdue) -->
      <div class="bg-slate-50/70 rounded-xl p-4 border border-slate-200/80 flex items-center justify-between">
        <div>
          <span class="text-xs font-bold uppercase tracking-wider text-slate-500 block mb-1">
            Terlambat (Overdue)
          </span>
          <div class="flex items-baseline space-x-1.5">
            <span
              class="text-3xl font-extrabold font-mono"
              :class="overdueCount > 0 ? 'text-rose-600' : 'text-slate-700'"
            >
              {{ overdueCount }}
            </span>
            <span class="text-xs text-slate-500 font-medium">tiket</span>
          </div>
          <p class="text-[11px] text-slate-500 mt-1">
            Diselesaikan melewati batas tenggat (Due Date)
          </p>
        </div>
        <div
          class="w-10 h-10 rounded-xl flex items-center justify-center shrink-0"
          :class="overdueCount > 0 ? 'bg-rose-100/80 text-rose-600' : 'bg-slate-100 text-slate-400'"
        >
          <AlertTriangle class="w-5 h-5" />
        </div>
      </div>
    </div>

    <!-- On-Time Rate Bar & Progress Visual -->
    <div class="mt-5 p-4 rounded-xl bg-gradient-to-r from-slate-50 to-blue-50/40 border border-slate-200/80">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2 mb-2">
        <div class="flex items-center space-x-2">
          <TrendingUp class="w-4 h-4 text-blue-600 shrink-0" />
          <span class="text-xs font-bold text-slate-800">
            Tingkat Ketepatan Waktu (On-Time Delivery Rate):
          </span>
        </div>
        <div class="flex items-baseline space-x-1.5">
          <span
            class="text-2xl font-black font-mono tracking-tight"
            :class="onTimeRate >= 85 ? 'text-emerald-600' : (onTimeRate >= 70 ? 'text-amber-600' : 'text-rose-600')"
          >
            {{ onTimeRate.toFixed(1) }}%
          </span>
          <span class="text-xs font-semibold text-slate-500">
            ({{ onTimeCount }} dari {{ doneCount }} tiket tepat waktu)
          </span>
        </div>
      </div>

      <!-- Segmented Bar -->
      <div class="w-full bg-slate-200/80 rounded-full h-3 overflow-hidden flex shadow-inner">
        <!-- On-time portion -->
        <div
          class="h-full bg-emerald-500 transition-all duration-500"
          :style="{ width: `${doneCount > 0 ? (onTimeCount / doneCount) * 100 : 0}%` }"
          :title="`Tepat Waktu: ${onTimeCount} tiket`"
        ></div>
        <!-- Overdue portion -->
        <div
          class="h-full bg-rose-500 transition-all duration-500"
          :style="{ width: `${doneCount > 0 ? (overdueCount / doneCount) * 100 : 0}%` }"
          :title="`Overdue: ${overdueCount} tiket`"
        ></div>
      </div>

      <div class="flex items-center justify-between mt-2 text-[11px] text-slate-500 font-medium">
        <div class="flex items-center space-x-4">
          <span class="flex items-center space-x-1">
            <span class="w-2.5 h-2.5 rounded-full bg-emerald-500 inline-block"></span>
            <span>Tepat Waktu: {{ onTimeCount }} tiket</span>
          </span>
          <span class="flex items-center space-x-1">
            <span class="w-2.5 h-2.5 rounded-full bg-rose-500 inline-block"></span>
            <span>Overdue: {{ overdueCount }} tiket</span>
          </span>
        </div>
        <span class="text-slate-400">Target Benchmark SLA: <strong>85.0%</strong></span>
      </div>
    </div>
  </div>
</template>
