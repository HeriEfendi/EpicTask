<script setup>
import { ref, computed } from 'vue';
import { useReportStore } from '@/stores/report';

const reportStore = useReportStore();

function formatHours(h) {
  if (!h) return '0h';
  return `${Number(h).toFixed(1)}h`;
}

function formatDate(d) {
  if (!d) return '-';
  try {
    const dt = new Date(d);
    return dt.toLocaleDateString('id-ID', { year: 'numeric', month: 'short', day: 'numeric' });
  } catch (e) {
    return d;
  }
}

const worklogSearch = ref('');
const filteredWorklogs = computed(() => {
  if (!reportStore.timesheet?.recent_logs) return [];
  const q = worklogSearch.value.toLowerCase().trim();
  if (!q) return reportStore.timesheet.recent_logs;
  return reportStore.timesheet.recent_logs.filter((log) => {
    return (
      (log.issue_key && log.issue_key.toLowerCase().includes(q)) ||
      (log.issue_summary && log.issue_summary.toLowerCase().includes(q)) ||
      (log.user_name && log.user_name.toLowerCase().includes(q)) ||
      (log.description && log.description.toLowerCase().includes(q))
    );
  });
});
</script>

<template>
  <div class="space-y-6">
    <!-- Timesheet User Summary Cards -->
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
      <div
        v-for="u in reportStore.timesheet?.users"
        :key="u.user_id"
        class="bg-white rounded-xl p-4 border border-slate-200/90 shadow-2xs flex items-center space-x-3"
      >
        <img
          :src="u.avatar_url || `https://api.dicebear.com/7.x/avataaars/svg?seed=${u.user_name}`"
          class="w-12 h-12 rounded-full border border-slate-200 bg-slate-100 shrink-0"
          alt="Avatar"
        />
        <div class="truncate">
          <p class="text-xs text-slate-500 font-medium">Alokasi Waktu</p>
          <h4 class="text-sm font-bold text-slate-900 truncate">{{ u.user_name }}</h4>
          <p class="text-lg font-extrabold text-blue-600 font-mono mt-0.5">
            {{ formatHours(u.total_hours) }}
          </p>
          <p class="text-[11px] text-slate-400">{{ u.issue_count }} tiket dikerjakan</p>
        </div>
      </div>
    </div>

    <!-- Timesheet Worklog Table -->
    <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs space-y-4">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
        <div>
          <h3 class="text-sm font-bold text-slate-900">Rincian Log Waktu Aktual (Worklogs Detail)</h3>
          <p class="text-xs text-slate-500">Daftar entri kerja yang diinput oleh developer pada tiket.</p>
        </div>

        <!-- Search box -->
        <div class="relative w-full sm:w-64">
          <input
            v-model="worklogSearch"
            placeholder="Cari tiket, nama, atau deskripsi..."
            class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs"
          />
        </div>
      </div>

      <div class="overflow-x-auto max-h-96">
        <table class="w-full text-left text-xs">
          <thead class="bg-slate-50 text-slate-500 font-bold uppercase tracking-wider border-y border-slate-200 sticky top-0 bg-slate-50 z-10">
            <tr>
              <th class="py-2.5 px-3">Tiket Key</th>
              <th class="py-2.5 px-3">Judul Pekerjaan</th>
              <th class="py-2.5 px-3">Pengembang</th>
              <th class="py-2.5 px-3 text-right">Durasi</th>
              <th class="py-2.5 px-3">Tanggal Log</th>
              <th class="py-2.5 px-3">Deskripsi Aktivitas</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-100">
            <tr
              v-for="log in filteredWorklogs"
              :key="log.id"
              class="hover:bg-slate-50/80 transition"
            >
              <td class="py-2.5 px-3 font-bold text-blue-600 font-mono">
                {{ log.issue_key }}
              </td>
              <td class="py-2.5 px-3 font-semibold text-slate-800 max-w-xs truncate">
                {{ log.issue_summary }}
              </td>
              <td class="py-2.5 px-3">
                <div class="flex items-center space-x-1.5">
                  <img
                    :src="log.avatar_url || `https://api.dicebear.com/7.x/avataaars/svg?seed=${log.user_name}`"
                    class="w-5 h-5 rounded-full border border-slate-200"
                    alt=""
                  />
                  <span class="text-slate-700 font-medium">{{ log.user_name || 'Unassigned' }}</span>
                </div>
              </td>
              <td class="py-2.5 px-3 text-right font-extrabold text-slate-900 font-mono">
                {{ formatHours(log.hours) }}
              </td>
              <td class="py-2.5 px-3 text-slate-500 whitespace-nowrap">
                {{ formatDate(log.logged_at) }}
              </td>
              <td class="py-2.5 px-3 text-slate-600 italic max-w-sm truncate">
                {{ log.description || '-' }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

