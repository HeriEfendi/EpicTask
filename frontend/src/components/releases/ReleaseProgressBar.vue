<script setup>
import { computed } from 'vue';

const props = defineProps({
  doneIssues: {
    type: Number,
    default: 0,
  },
  inProgressIssues: {
    type: Number,
    default: 0,
  },
  todoIssues: {
    type: Number,
    default: 0,
  },
  totalIssues: {
    type: Number,
    default: 0,
  },
});

const donePercent = computed(() => {
  if (!props.totalIssues) return 0;
  return Math.round((props.doneIssues / props.totalIssues) * 100);
});

const inProgressPercent = computed(() => {
  if (!props.totalIssues) return 0;
  return Math.round((props.inProgressIssues / props.totalIssues) * 100);
});

const todoPercent = computed(() => {
  if (!props.totalIssues) return 0;
  return Math.max(0, 100 - donePercent.value - inProgressPercent.value);
});
</script>

<template>
  <div class="space-y-2">
    <!-- Progress Bar (3-segment Jira-style) -->
    <div class="w-full bg-slate-100 h-2.5 rounded-full overflow-hidden flex shadow-inner">
      <div
        v-if="donePercent > 0"
        :style="{ width: `${donePercent}%` }"
        class="bg-emerald-500 h-full transition-all duration-300"
        title="Selesai"
      ></div>
      <div
        v-if="inProgressPercent > 0"
        :style="{ width: `${inProgressPercent}%` }"
        class="bg-blue-500 h-full transition-all duration-300"
        title="Sedang Dikerjakan"
      ></div>
      <div
        v-if="todoPercent > 0"
        :style="{ width: `${todoPercent}%` }"
        class="bg-slate-300 h-full transition-all duration-300"
        title="Belum Dikerjakan"
      ></div>
    </div>

    <!-- Stats Breakdown Legend -->
    <div class="flex items-center justify-between text-xs text-slate-500 font-medium">
      <div class="flex items-center gap-3">
        <span class="flex items-center gap-1.5 text-emerald-700">
          <span class="w-2 h-2 rounded-full bg-emerald-500"></span>
          <span>{{ doneIssues }} Selesai ({{ donePercent }}%)</span>
        </span>
        <span class="flex items-center gap-1.5 text-blue-700">
          <span class="w-2 h-2 rounded-full bg-blue-500"></span>
          <span>{{ inProgressIssues }} Sedang Dikerjakan</span>
        </span>
        <span class="flex items-center gap-1.5 text-slate-600">
          <span class="w-2 h-2 rounded-full bg-slate-300"></span>
          <span>{{ todoIssues }} Menunggu</span>
        </span>
      </div>
      <span class="font-semibold text-slate-700 font-mono">
        Total: {{ totalIssues }} Tiket
      </span>
    </div>
  </div>
</template>
