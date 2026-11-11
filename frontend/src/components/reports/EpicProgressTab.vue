<script setup>
import { useReportStore } from '@/stores/report';

const reportStore = useReportStore();

function formatDate(d) {
  if (!d) return '-';
  try {
    const dt = new Date(d);
    return dt.toLocaleDateString('id-ID', { year: 'numeric', month: 'short', day: 'numeric' });
  } catch (e) {
    return d;
  }
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-base font-bold text-slate-900">Epic Burndown & Inisiatif Roadmap</h2>
        <p class="text-xs text-slate-500">Status penyelesaian fitur-fitur besar kuartal (Epics) terhadap stories dan subtasks terkait.</p>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div
        v-for="ep in reportStore.epicProgress?.epics"
        :key="ep.id"
        class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs flex flex-col justify-between hover:border-purple-300 transition"
      >
        <div>
          <div class="flex items-center justify-between mb-2">
            <span class="px-2 py-0.5 rounded font-mono font-bold text-xs bg-purple-100 text-purple-800 border border-purple-200">
              {{ ep.key }}
            </span>
            <span
              class="px-2 py-0.5 rounded text-[11px] font-bold"
              :style="{ backgroundColor: `${ep.status_color}20`, color: ep.status_color }"
            >
              {{ ep.status_name }}
            </span>
          </div>

          <h3 class="text-sm font-bold text-slate-900 mb-1">
            {{ ep.summary }}
          </h3>

          <div class="flex items-center space-x-3 text-xs text-slate-500 mb-3 font-medium">
            <span>{{ formatDate(ep.start_date) }} → {{ formatDate(ep.due_date) }}</span>
            <span>•</span>
            <span class="font-bold text-purple-700">{{ ep.epic_points }} pts</span>
          </div>
        </div>

        <div>
          <!-- Progress Bar -->
          <div class="flex items-center justify-between text-xs font-semibold mb-1">
            <span class="text-slate-600">
              {{ ep.done_children }} / {{ ep.total_children }} Tiket Anak Selesai
            </span>
            <span class="font-bold text-purple-700 font-mono">{{ ep.progress_pct }}%</span>
          </div>
          <div class="bg-slate-100 rounded-full h-2.5 overflow-hidden">
            <div
              class="bg-purple-600 h-2.5 rounded-full transition-all duration-500"
              :style="{ width: `${ep.progress_pct}%` }"
            ></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

