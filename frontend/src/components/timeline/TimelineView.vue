<script setup>
import { ref, computed } from 'vue';
import { useProjectStore } from '@/stores/project';
import {
  CalendarRange,
  ChevronLeft,
  ChevronRight,
  Clock,
  Layers,
  Calendar,
  AlertCircle
} from 'lucide-vue-next';

const projectStore = useProjectStore();

// View window: days offset from today
const baseDate = ref(new Date());

const daysToShow = 28; // 4 weeks

const timelineDays = computed(() => {
  const days = [];
  const start = new Date(baseDate.value);
  start.setDate(start.getDate() - 3); // 3 days in the past

  for (let i = 0; i < daysToShow; i++) {
    const d = new Date(start);
    d.setDate(start.getDate() + i);
    days.push({
      date: d,
      dateString: d.toISOString().split('T')[0],
      dayName: d.toLocaleDateString('en-US', { weekday: 'short' }),
      dayNumber: d.getDate(),
      monthName: d.toLocaleDateString('en-US', { month: 'short' }),
      isToday: isSameDay(d, new Date()),
      isWeekend: d.getDay() === 0 || d.getDay() === 6,
    });
  }
  return days;
});

function isSameDay(d1, d2) {
  return (
    d1.getFullYear() === d2.getFullYear() &&
    d1.getMonth() === d2.getMonth() &&
    d1.getDate() === d2.getDate()
  );
}

function shiftDays(amount) {
  const d = new Date(baseDate.value);
  d.setDate(d.getDate() + amount);
  baseDate.value = d;
}

function resetToToday() {
  baseDate.value = new Date();
}

// Calculate horizontal position and width for each issue bar
function getBarStyle(issue) {
  if (!issue.start_date && !issue.due_date) return null;

  const startDate = new Date(issue.start_date || issue.due_date);
  const dueDate = new Date(issue.due_date || issue.start_date);

  const windowStart = timelineDays.value[0].date;
  const windowEnd = timelineDays.value[timelineDays.value.length - 1].date;

  const totalWindowMs = windowEnd.getTime() - windowStart.getTime() + 86400000;
  const startOffsetMs = Math.max(0, startDate.getTime() - windowStart.getTime());
  const durationMs = Math.max(86400000, dueDate.getTime() - startDate.getTime() + 86400000);

  const leftPercent = (startOffsetMs / totalWindowMs) * 100;
  const widthPercent = Math.min(100 - leftPercent, (durationMs / totalWindowMs) * 100);

  // If outside visible range
  if (dueDate < windowStart || startDate > windowEnd) {
    return null;
  }

  return {
    left: `${Math.max(0, leftPercent)}%`,
    width: `${Math.max(2.5, widthPercent)}%`,
  };
}

function getBarColor(issue) {
  if (issue.status_category === 'DONE') {
    return 'bg-emerald-600 hover:bg-emerald-500 border-emerald-500 text-white';
  }
  if (issue.status_category === 'IN_PROGRESS') {
    return 'bg-amber-500 hover:bg-amber-400 border-amber-400 text-white';
  }
  return 'bg-blue-600 hover:bg-blue-500 border-blue-500 text-white';
}
</script>

<template>
  <div class="flex-1 p-6 flex flex-col overflow-hidden bg-slate-50">
    <!-- Header Controls -->
    <div class="flex items-center justify-between pb-4 border-b border-slate-200">
      <div class="flex items-center space-x-3">
        <h2 class="text-sm font-bold text-slate-800 flex items-center gap-2">
          <CalendarRange class="w-4 h-4 text-blue-600" />
          <span>Project Timeline & Gantt Schedule</span>
        </h2>
        <span class="text-xs text-slate-500 font-medium hidden sm:inline">
          {{ timelineDays[0].monthName }} {{ timelineDays[0].dayNumber }} - {{ timelineDays[timelineDays.length - 1].monthName }} {{ timelineDays[timelineDays.length - 1].dayNumber }}
        </span>
      </div>

      <div class="flex items-center space-x-2">
        <button
          @click="shiftDays(-7)"
          class="p-1.5 rounded-lg bg-white border border-slate-200 text-slate-700 hover:bg-slate-100 transition shadow-xs"
          title="Previous week"
        >
          <ChevronLeft class="w-4 h-4" />
        </button>

        <button
          @click="resetToToday"
          class="px-2.5 py-1 rounded-lg bg-white border border-slate-200 text-xs font-semibold text-slate-700 hover:bg-slate-100 transition shadow-xs"
        >
          Today
        </button>

        <button
          @click="shiftDays(7)"
          class="p-1.5 rounded-lg bg-white border border-slate-200 text-slate-700 hover:bg-slate-100 transition shadow-xs"
          title="Next week"
        >
          <ChevronRight class="w-4 h-4" />
        </button>
      </div>
    </div>

    <!-- Timeline Grid Area -->
    <div class="flex-1 overflow-auto mt-4 border border-slate-200 rounded-xl bg-white shadow-xs">
      <!-- Calendar Dates Header -->
      <div class="flex border-b border-slate-200 sticky top-0 bg-slate-100 z-20">
        <!-- Issues Label Column -->
        <div class="w-72 shrink-0 p-3 font-semibold text-xs text-slate-600 border-r border-slate-200 bg-slate-100">
          Issue / Milestone
        </div>

        <!-- Days Grid -->
        <div class="flex-1 flex">
          <div
            v-for="d in timelineDays"
            :key="d.dateString"
            class="flex-1 min-w-[34px] py-2 px-1 text-center border-r border-slate-200 flex flex-col items-center justify-center transition"
            :class="{
              'bg-blue-50 border-b-2 border-b-blue-600': d.isToday,
              'bg-slate-50/80': d.isWeekend && !d.isToday,
            }"
          >
            <span class="text-[10px] uppercase font-bold" :class="d.isToday ? 'text-blue-600 font-extrabold' : 'text-slate-500'">
              {{ d.dayName[0] }}
            </span>
            <span class="text-[11px] font-mono mt-0.5" :class="d.isToday ? 'text-blue-700 font-bold' : 'text-slate-700'">
              {{ d.dayNumber }}
            </span>
          </div>
        </div>
      </div>

      <!-- Issues Rows -->
      <div class="divide-y divide-slate-100">
        <div
          v-for="issue in projectStore.filteredIssues"
          :key="issue.id"
          class="flex items-center hover:bg-slate-50/80 group transition"
        >
          <!-- Left: Issue Label -->
          <div
            @click="projectStore.openIssueDetail(issue.id)"
            class="w-72 shrink-0 p-3 border-r border-slate-200 flex items-center space-x-2.5 cursor-pointer bg-white group-hover:bg-slate-50 transition"
          >
            <span
              class="text-[9px] font-bold px-1.5 py-0.5 rounded uppercase"
              :class="`badge-type-${issue.issue_type}`"
            >
              {{ issue.issue_type[0] }}
            </span>
            <span class="font-mono text-xs font-semibold text-blue-600">{{ issue.key }}</span>
            <span class="text-xs text-slate-800 truncate max-w-[140px]" :title="issue.summary">{{ issue.summary }}</span>
          </div>

          <!-- Right: Horizontal Bar Track -->
          <div class="flex-1 relative h-12 flex items-center px-1">
            <!-- Timeline Grid lines background -->
            <div class="absolute inset-0 flex pointer-events-none">
              <div
                v-for="d in timelineDays"
                :key="d.dateString"
                class="flex-1 min-w-[34px] border-r border-slate-100"
                :class="{ 'bg-blue-50/40': d.isToday }"
              ></div>
            </div>

            <!-- The Horizontal Gantt Bar -->
            <div
              v-if="getBarStyle(issue)"
              :style="getBarStyle(issue)"
              @click="projectStore.openIssueDetail(issue.id)"
              class="absolute h-7 rounded-lg border shadow-xs px-2 flex items-center justify-between text-xs font-medium cursor-pointer transition-all hover:scale-[1.01] active:scale-[0.99] z-10 overflow-hidden"
              :class="getBarColor(issue)"
              :title="`${issue.key}: ${issue.summary}\n${issue.start_date || 'Start'} -> ${issue.due_date || 'End'}`"
            >
              <div class="flex items-center space-x-1.5 truncate">
                <span class="font-mono font-bold text-[11px]">{{ issue.key }}</span>
                <span class="truncate text-[11px] hidden sm:inline">{{ issue.summary }}</span>
              </div>
              <img
                v-if="issue.assignee_avatar"
                :src="issue.assignee_avatar"
                :alt="issue.assignee_name || 'Assignee'"
                class="w-4 h-4 rounded-full ring-1 ring-white/60 shrink-0 ml-1"
              />
            </div>

            <!-- Unscheduled Warning if no dates -->
            <div
              v-else
              class="text-[11px] text-slate-400 italic pl-3 flex items-center space-x-1"
            >
              <AlertCircle class="w-3 h-3 text-slate-400" />
              <span>Dates not scheduled</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
