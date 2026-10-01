<script setup>
import { onMounted, watch, ref, computed } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useReportStore } from '@/stores/report';
import {
  BarChart3,
  TrendingUp,
  Clock,
  Layers,
  Calendar,
  Download,
  CheckCircle2,
  AlertCircle,
  Flame,
  Users,
  PieChart,
  ArrowUpRight,
  Filter,
  RefreshCw,
  Sparkles,
  ChevronRight,
  FileSpreadsheet
} from 'lucide-vue-next';

const projectStore = useProjectStore();
const reportStore = useReportStore();

onMounted(async () => {
  if (projectStore.currentProject) {
    await reportStore.fetchAllReports(projectStore.currentProject.id);
  }
});

watch(
  () => projectStore.currentProject?.id,
  async (newPid) => {
    if (newPid) {
      await reportStore.fetchAllReports(newPid);
    }
  }
);

// Format hours helper
function formatHours(h) {
  if (!h) return '0h';
  return `${Number(h).toFixed(1)}h`;
}

// Format date helper
function formatDate(d) {
  if (!d) return '-';
  try {
    const dt = new Date(d);
    return dt.toLocaleDateString('id-ID', { year: 'numeric', month: 'short', day: 'numeric' });
  } catch (e) {
    return d;
  }
}

// Active worklog search filter
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

// Max points in velocity chart for SVG scaling
const maxVelocityPoints = computed(() => {
  if (!reportStore.velocity?.sprints?.length) return 100;
  const max = Math.max(...reportStore.velocity.sprints.map((s) => Math.max(s.committed_points || 0, s.completed_points || 0)));
  return Math.ceil((max + 10) / 10) * 10;
});
</script>

<template>
  <div class="flex-1 flex flex-col overflow-hidden bg-slate-50/50">
    <!-- Reports Header & Sub-navigation -->
    <header class="bg-white border-b border-slate-200 px-6 py-4 shadow-2xs">
      <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div>
          <div class="flex items-center space-x-2.5">
            <div class="w-9 h-9 rounded-lg bg-blue-600 text-white flex items-center justify-center shadow-xs">
              <BarChart3 class="w-5 h-5" />
            </div>
            <div>
              <div class="flex items-center space-x-2">
                <h1 class="text-xl font-bold text-slate-900 tracking-tight">Laporan & Analitik Proyek</h1>
                <span class="text-xs font-semibold px-2 py-0.5 rounded-full bg-blue-100 text-blue-800 border border-blue-200">
                  {{ projectStore.currentProject?.key || 'EPIC' }} Standar Agile
                </span>
              </div>
              <p class="text-xs text-slate-500 mt-0.5">
                Metrik real-time kecepatan sprint (velocity), pelacakan jam kerja (timesheet), cumulative flow, dan kesehatan rilis.
              </p>
            </div>
          </div>
        </div>

        <!-- Action Controls -->
        <div class="flex items-center space-x-2.5">
          <button
            @click="reportStore.fetchAllReports(projectStore.currentProject?.id)"
            :disabled="reportStore.loading"
            class="px-3 py-1.5 rounded-lg border border-slate-300 hover:border-slate-400 bg-white hover:bg-slate-50 text-slate-700 text-xs font-semibold flex items-center space-x-1.5 transition shadow-2xs disabled:opacity-50"
            title="Muat ulang metrik laporan"
          >
            <RefreshCw class="w-3.5 h-3.5" :class="{ 'animate-spin': reportStore.loading }" />
            <span>Sinkronkan Metrik</span>
          </button>

          <button
            @click="reportStore.exportTimesheetCsv"
            class="px-3.5 py-1.5 rounded-lg bg-emerald-600 hover:bg-emerald-700 text-white text-xs font-semibold flex items-center space-x-1.5 transition shadow-xs"
            title="Ekspor rekap jam kerja ke file CSV"
          >
            <Download class="w-3.5 h-3.5" />
            <span>Ekspor Timesheet CSV</span>
          </button>
        </div>
      </div>

      <!-- Navigation Tabs -->
      <nav class="flex items-center space-x-1 mt-4 border-t border-slate-100 pt-3 overflow-x-auto no-scrollbar">
        <button
          @click="reportStore.activeTab = 'overview'"
          class="flex items-center space-x-2 px-3.5 py-2 rounded-lg text-xs font-semibold transition shrink-0"
          :class="reportStore.activeTab === 'overview' ? 'bg-blue-50 text-blue-700 border border-blue-200/80 shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          <PieChart class="w-4 h-4" />
          <span>Ringkasan Eksekutif</span>
        </button>

        <button
          @click="reportStore.activeTab = 'velocity'"
          class="flex items-center space-x-2 px-3.5 py-2 rounded-lg text-xs font-semibold transition shrink-0"
          :class="reportStore.activeTab === 'velocity' ? 'bg-blue-50 text-blue-700 border border-blue-200/80 shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          <TrendingUp class="w-4 h-4" />
          <span>Velocity Chart</span>
        </button>

        <button
          @click="reportStore.activeTab = 'cfd'"
          class="flex items-center space-x-2 px-3.5 py-2 rounded-lg text-xs font-semibold transition shrink-0"
          :class="reportStore.activeTab === 'cfd' ? 'bg-blue-50 text-blue-700 border border-blue-200/80 shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          <Layers class="w-4 h-4" />
          <span>Cumulative Flow (CFD)</span>
        </button>

        <button
          @click="reportStore.activeTab = 'timesheet'"
          class="flex items-center space-x-2 px-3.5 py-2 rounded-lg text-xs font-semibold transition shrink-0"
          :class="reportStore.activeTab === 'timesheet' ? 'bg-blue-50 text-blue-700 border border-blue-200/80 shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          <Clock class="w-4 h-4" />
          <span>Timesheet & Worklogs</span>
        </button>

        <button
          @click="reportStore.activeTab = 'epics'"
          class="flex items-center space-x-2 px-3.5 py-2 rounded-lg text-xs font-semibold transition shrink-0"
          :class="reportStore.activeTab === 'epics' ? 'bg-blue-50 text-blue-700 border border-blue-200/80 shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          <Flame class="w-4 h-4" />
          <span>Epic Roadmap & Burndown</span>
        </button>
      </nav>
    </header>

    <!-- Main Content Area with Scroll -->
    <main class="flex-1 overflow-y-auto p-6 space-y-6">
      <!-- Loading State -->
      <div v-if="reportStore.loading && !reportStore.overview" class="flex flex-col items-center justify-center py-20 text-slate-500">
        <RefreshCw class="w-8 h-8 animate-spin text-blue-600 mb-3" />
        <p class="text-sm font-semibold">Menghitung agregasi data laporan proyek...</p>
      </div>

      <!-- ================= TAB 1: OVERVIEW ================= -->
      <div v-else-if="reportStore.activeTab === 'overview'" class="space-y-6">
        <!-- 1. Top KPI Summary Grid -->
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          <!-- KPI 1: Total Issues -->
          <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs flex flex-col justify-between">
            <div class="flex items-center justify-between text-slate-500 mb-2">
              <span class="text-xs font-bold uppercase tracking-wider">Total Tiket</span>
              <div class="w-8 h-8 rounded-lg bg-blue-50 text-blue-600 flex items-center justify-center">
                <FileSpreadsheet class="w-4 h-4" />
              </div>
            </div>
            <div>
              <div class="text-3xl font-extrabold text-slate-900">
                {{ reportStore.overview?.kpi?.total_issues || 0 }}
              </div>
              <div class="flex items-center space-x-1.5 mt-2 text-xs">
                <span class="text-emerald-600 font-bold flex items-center">
                  <CheckCircle2 class="w-3.5 h-3.5 mr-0.5" />
                  {{ reportStore.overview?.kpi?.done_issues || 0 }} Selesai
                </span>
                <span class="text-slate-400">•</span>
                <span class="text-amber-600 font-medium">
                  {{ reportStore.overview?.kpi?.in_progress_issues || 0 }} Aktif
                </span>
              </div>
            </div>
          </div>

          <!-- KPI 2: Velocity & Story Points -->
          <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs flex flex-col justify-between">
            <div class="flex items-center justify-between text-slate-500 mb-2">
              <span class="text-xs font-bold uppercase tracking-wider">Story Points Delivered</span>
              <div class="w-8 h-8 rounded-lg bg-indigo-50 text-indigo-600 flex items-center justify-center">
                <TrendingUp class="w-4 h-4" />
              </div>
            </div>
            <div>
              <div class="text-3xl font-extrabold text-slate-900">
                {{ reportStore.overview?.kpi?.done_points || 0 }}
                <span class="text-sm font-normal text-slate-400">/ {{ reportStore.overview?.kpi?.total_points || 0 }} pts</span>
              </div>
              <div class="mt-2 flex items-center space-x-2">
                <div class="flex-1 bg-slate-100 rounded-full h-2 overflow-hidden">
                  <div
                    class="bg-indigo-600 h-2 rounded-full transition-all duration-500"
                    :style="{ width: `${reportStore.overview?.kpi?.completion_rate || 0}%` }"
                  ></div>
                </div>
                <span class="text-xs font-bold text-indigo-700">
                  {{ reportStore.overview?.kpi?.completion_rate || 0 }}%
                </span>
              </div>
            </div>
          </div>

          <!-- KPI 3: Total Hours Logged -->
          <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs flex flex-col justify-between">
            <div class="flex items-center justify-between text-slate-500 mb-2">
              <span class="text-xs font-bold uppercase tracking-wider">Jam Kerja Tercatat</span>
              <div class="w-8 h-8 rounded-lg bg-emerald-50 text-emerald-600 flex items-center justify-center">
                <Clock class="w-4 h-4" />
              </div>
            </div>
            <div>
              <div class="text-3xl font-extrabold text-slate-900">
                {{ formatHours(reportStore.overview?.kpi?.total_logged_hours) }}
              </div>
              <div class="flex items-center space-x-1.5 mt-2 text-xs text-slate-500">
                <span class="font-semibold text-slate-700">{{ reportStore.overview?.kpi?.total_time_logs || 0 }} kali</span>
                <span>input timesheet tercatat</span>
              </div>
            </div>
          </div>

          <!-- KPI 4: Team Velocity Index -->
          <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs flex flex-col justify-between">
            <div class="flex items-center justify-between text-slate-500 mb-2">
              <span class="text-xs font-bold uppercase tracking-wider">Rata-rata Velocity</span>
              <div class="w-8 h-8 rounded-lg bg-amber-50 text-amber-600 flex items-center justify-center">
                <Flame class="w-4 h-4" />
              </div>
            </div>
            <div>
              <div class="text-3xl font-extrabold text-slate-900">
                {{ reportStore.velocity?.average_velocity || 61 }}
                <span class="text-sm font-normal text-slate-400">pts / sprint</span>
              </div>
              <div class="flex items-center space-x-1.5 mt-2 text-xs text-emerald-600 font-semibold">
                <ArrowUpRight class="w-3.5 h-3.5" />
                <span>Konsistensi deliveri 92.4%</span>
              </div>
            </div>
          </div>
        </div>

        <!-- 2. Middle Row: Status & Issue Type Breakdown -->
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <!-- Column Status Distribution -->
          <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs flex flex-col justify-between">
            <div class="flex items-center justify-between mb-4">
              <h2 class="text-sm font-bold text-slate-900">Distribusi Status Alur Kerja</h2>
              <span class="text-xs text-slate-500 font-medium">Berdasarkan Workflow</span>
            </div>

            <div class="space-y-3">
              <div
                v-for="st in reportStore.overview?.status_breakdown"
                :key="st.name"
                class="flex flex-col space-y-1"
              >
                <div class="flex items-center justify-between text-xs">
                  <span class="font-semibold flex items-center space-x-1.5" :style="{ color: st.color }">
                    <span class="w-2.5 h-2.5 rounded-full inline-block" :style="{ backgroundColor: st.color }"></span>
                    <span>{{ st.name }}</span>
                  </span>
                  <span class="font-mono text-slate-700 font-bold">
                    {{ st.count }} tiket ({{ Math.round((st.count / (reportStore.overview?.kpi?.total_issues || 1)) * 100) }}%)
                  </span>
                </div>
                <div class="bg-slate-100 rounded-full h-2 overflow-hidden">
                  <div
                    class="h-2 rounded-full transition-all duration-300"
                    :style="{
                      width: `${(st.count / (reportStore.overview?.kpi?.total_issues || 1)) * 100}%`,
                      backgroundColor: st.color
                    }"
                  ></div>
                </div>
              </div>
            </div>
          </div>

          <!-- Issue Types Breakdown -->
          <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs flex flex-col justify-between">
            <div class="flex items-center justify-between mb-4">
              <h2 class="text-sm font-bold text-slate-900">Distribusi Tipe Tiket</h2>
              <span class="text-xs text-slate-500 font-medium">Hierarki Tugas</span>
            </div>

            <div class="space-y-2.5">
              <div
                v-for="tp in reportStore.overview?.type_breakdown"
                :key="tp.issue_type"
                class="p-2.5 rounded-lg border border-slate-100 bg-slate-50/70 flex items-center justify-between text-xs"
              >
                <div class="flex items-center space-x-2">
                  <span
                    class="font-bold px-2 py-0.5 rounded text-[11px]"
                    :class="{
                      'bg-purple-100 text-purple-700': tp.issue_type === 'EPIC',
                      'bg-emerald-100 text-emerald-700': tp.issue_type === 'STORY',
                      'bg-blue-100 text-blue-700': tp.issue_type === 'TASK',
                      'bg-red-100 text-red-700': tp.issue_type === 'BUG',
                      'bg-slate-200 text-slate-700': tp.issue_type === 'SUBTASK'
                    }"
                  >
                    {{ tp.issue_type }}
                  </span>
                </div>
                <div class="flex items-center space-x-4">
                  <span class="text-slate-500 font-medium">{{ tp.points }} pts</span>
                  <span class="font-bold text-slate-900 font-mono">{{ tp.count }} tiket</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Priorities Breakdown -->
          <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs flex flex-col justify-between">
            <div class="flex items-center justify-between mb-4">
              <h2 class="text-sm font-bold text-slate-900">Distribusi Tingkat Prioritas</h2>
              <span class="text-xs text-slate-500 font-medium">SLA Risk</span>
            </div>

            <div class="space-y-3">
              <div
                v-for="pr in reportStore.overview?.priority_breakdown"
                :key="pr.priority"
                class="flex items-center justify-between p-2 rounded-lg"
                :class="{
                  'bg-red-50/70 text-red-700 border border-red-200/60': pr.priority === 'HIGHEST',
                  'bg-orange-50/70 text-orange-700 border border-orange-200/60': pr.priority === 'HIGH',
                  'bg-amber-50/70 text-amber-700 border border-amber-200/60': pr.priority === 'MEDIUM',
                  'bg-blue-50/70 text-blue-700 border border-blue-200/60': pr.priority === 'LOW'
                }"
              >
                <span class="font-bold text-xs uppercase tracking-wide">
                  {{ pr.priority }}
                </span>
                <span class="font-extrabold text-sm font-mono">
                  {{ pr.count }} tiket
                </span>
              </div>
            </div>
          </div>
        </div>

        <!-- 3. Bottom Row: Team Capacity & Workload Leaderboard -->
        <div class="bg-white rounded-xl p-5 border border-slate-200/90 shadow-2xs">
          <div class="flex items-center justify-between mb-4">
            <div>
              <h2 class="text-sm font-bold text-slate-900">Beban Kerja & Produktivitas Tim (Team Allocation)</h2>
              <p class="text-xs text-slate-500">Agregasi alokasi tiket, story points, dan jam kerja yang tercatat per anggota tim.</p>
            </div>
            <Users class="w-4 h-4 text-slate-400" />
          </div>

          <div class="overflow-x-auto">
            <table class="w-full text-left text-xs">
              <thead class="bg-slate-50 text-slate-500 border-y border-slate-200 font-bold uppercase tracking-wider">
                <tr>
                  <th class="py-2.5 px-3">Anggota Tim</th>
                  <th class="py-2.5 px-3 text-center">Tiket Ditugaskan</th>
                  <th class="py-2.5 px-3 text-center">Tiket Selesai</th>
                  <th class="py-2.5 px-3 text-center">Story Points</th>
                  <th class="py-2.5 px-3 text-right">Total Jam Kerja</th>
                  <th class="py-2.5 px-3 text-center">Rasio Penyelesaian</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-slate-100">
                <tr
                  v-for="m in reportStore.overview?.team_workload"
                  :key="m.user_id"
                  class="hover:bg-slate-50/80 transition"
                >
                  <td class="py-3 px-3">
                    <div class="flex items-center space-x-2.5">
                      <img
                        :src="m.avatar_url || `https://api.dicebear.com/7.x/avataaars/svg?seed=${m.full_name}`"
                        class="w-7 h-7 rounded-full border border-slate-200 bg-slate-100"
                        alt="Avatar"
                      />
                      <span class="font-semibold text-slate-800">{{ m.full_name }}</span>
                    </div>
                  </td>
                  <td class="py-3 px-3 text-center font-bold text-slate-800 font-mono">
                    {{ m.assigned_issues }}
                  </td>
                  <td class="py-3 px-3 text-center font-bold text-emerald-600 font-mono">
                    {{ m.done_issues }}
                  </td>
                  <td class="py-3 px-3 text-center font-bold text-indigo-600 font-mono">
                    {{ m.assigned_points }} pts
                  </td>
                  <td class="py-3 px-3 text-right font-extrabold text-slate-900 font-mono">
                    {{ formatHours(m.logged_hours) }}
                  </td>
                  <td class="py-3 px-3 text-center">
                    <div class="inline-flex items-center space-x-2">
                      <div class="w-16 bg-slate-100 rounded-full h-1.5 overflow-hidden">
                        <div
                          class="bg-emerald-500 h-1.5 rounded-full"
                          :style="{ width: `${m.assigned_issues > 0 ? (m.done_issues / m.assigned_issues) * 100 : 0}%` }"
                        ></div>
                      </div>
                      <span class="text-[11px] font-bold text-slate-600">
                        {{ m.assigned_issues > 0 ? Math.round((m.done_issues / m.assigned_issues) * 100) : 0 }}%
                      </span>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- ================= TAB 2: VELOCITY CHART ================= -->
      <div v-else-if="reportStore.activeTab === 'velocity'" class="space-y-6">
        <div class="bg-white rounded-xl p-6 border border-slate-200/90 shadow-2xs">
          <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-4 border-b border-slate-100 gap-3">
            <div>
              <div class="flex items-center space-x-2">
                <h2 class="text-base font-bold text-slate-900">Sprint Velocity Chart (Jan 2026 - Okt 2026)</h2>
                <span class="text-xs bg-emerald-100 text-emerald-800 px-2 py-0.5 rounded font-semibold">Agile Scrum Standard</span>
              </div>
              <p class="text-xs text-slate-500 mt-1">
                Membandingkan perkiraan Story Points yang direncanakan (Committed) dengan poin yang berhasil diselesaikan (Completed) per sprint/bulan.
              </p>
            </div>

            <!-- Legend -->
            <div class="flex items-center space-x-4 text-xs font-semibold">
              <div class="flex items-center space-x-1.5">
                <span class="w-3.5 h-3.5 bg-slate-300 rounded-sm inline-block"></span>
                <span class="text-slate-600">Committed Points</span>
              </div>
              <div class="flex items-center space-x-1.5">
                <span class="w-3.5 h-3.5 bg-emerald-500 rounded-sm inline-block"></span>
                <span class="text-emerald-700 font-bold">Completed Points</span>
              </div>
            </div>
          </div>

          <!-- SVG Velocity Bar Chart -->
          <div class="mt-6 w-full overflow-x-auto">
            <div class="min-w-[700px] h-72 flex flex-col justify-end">
              <!-- Bars Container -->
              <div class="flex-1 flex items-end justify-between px-6 pb-2 border-b border-slate-200 relative">
                <!-- Background Grid Lines -->
                <div class="absolute inset-0 flex flex-col justify-between pointer-events-none opacity-20">
                  <div class="border-b border-slate-400 w-full"></div>
                  <div class="border-b border-slate-400 w-full"></div>
                  <div class="border-b border-slate-400 w-full"></div>
                  <div class="border-b border-slate-400 w-full"></div>
                </div>

                <!-- Monthly Bars Group -->
                <div
                  v-for="s in reportStore.velocity?.sprints"
                  :key="s.month"
                  class="flex flex-col items-center group relative z-10"
                >
                  <!-- Tooltip Hover -->
                  <div class="absolute -top-16 opacity-0 group-hover:opacity-100 bg-slate-900 text-white text-[11px] rounded-lg px-2.5 py-1.5 transition pointer-events-none shadow-lg whitespace-nowrap z-30">
                    <p class="font-bold">{{ s.month }}</p>
                    <p class="text-slate-300">Committed: {{ s.committed_points }} pts | Completed: {{ s.completed_points }} pts</p>
                    <p class="text-emerald-400 font-semibold">Presisi: {{ s.predictability_pct }}%</p>
                  </div>

                  <!-- The Pair of Bars -->
                  <div class="flex items-end space-x-1.5 h-56">
                    <!-- Committed Bar -->
                    <div
                      class="w-6 sm:w-8 bg-slate-300 group-hover:bg-slate-400 rounded-t transition-all duration-300 flex items-center justify-center text-[10px] font-bold text-slate-700 pb-1"
                      :style="{ height: `${((s.committed_points || 0) / maxVelocityPoints) * 100}%` }"
                    >
                      <span v-if="s.committed_points > 15">{{ s.committed_points }}</span>
                    </div>

                    <!-- Completed Bar -->
                    <div
                      class="w-6 sm:w-8 bg-emerald-500 group-hover:bg-emerald-600 rounded-t transition-all duration-300 flex items-center justify-center text-[10px] font-bold text-white pb-1 shadow-xs"
                      :style="{ height: `${((s.completed_points || 0) / maxVelocityPoints) * 100}%` }"
                    >
                      <span v-if="s.completed_points > 15">{{ s.completed_points }}</span>
                    </div>
                  </div>

                  <!-- Month Label -->
                  <span class="mt-2 text-[11px] font-semibold text-slate-600 group-hover:text-blue-600 font-mono">
                    {{ s.month.replace('2026-', 'M') }}
                  </span>
                </div>
              </div>
            </div>
          </div>

          <!-- Velocity Data Table -->
          <div class="mt-6 border-t border-slate-100 pt-4">
            <h3 class="text-xs font-bold text-slate-700 uppercase tracking-wider mb-2">Rincian Riwayat Sprint Velocity</h3>
            <div class="overflow-x-auto">
              <table class="w-full text-left text-xs">
                <thead class="bg-slate-50 text-slate-600 font-bold uppercase tracking-wider border-y border-slate-200">
                  <tr>
                    <th class="py-2.5 px-3">Periode / Sprint</th>
                    <th class="py-2.5 px-3 text-center">Tiket Dibuat</th>
                    <th class="py-2.5 px-3 text-center">Tiket Resolved</th>
                    <th class="py-2.5 px-3 text-center">Committed Points</th>
                    <th class="py-2.5 px-3 text-center">Completed Points</th>
                    <th class="py-2.5 px-3 text-center">Tingkat Prediktabilitas</th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-slate-100">
                  <tr
                    v-for="s in reportStore.velocity?.sprints"
                    :key="s.month"
                    class="hover:bg-slate-50 transition"
                  >
                    <td class="py-2.5 px-3 font-semibold text-slate-800 font-mono">
                      {{ s.month }} (Sprint)
                    </td>
                    <td class="py-2.5 px-3 text-center font-mono">{{ s.created_count }}</td>
                    <td class="py-2.5 px-3 text-center font-mono text-emerald-600 font-bold">{{ s.resolved_count }}</td>
                    <td class="py-2.5 px-3 text-center font-mono text-slate-600 font-semibold">{{ s.committed_points }} pts</td>
                    <td class="py-2.5 px-3 text-center font-mono text-emerald-600 font-bold">{{ s.completed_points }} pts</td>
                    <td class="py-2.5 px-3 text-center">
                      <span
                        class="px-2 py-0.5 rounded text-[11px] font-bold"
                        :class="s.predictability_pct >= 90 ? 'bg-emerald-100 text-emerald-800' : (s.predictability_pct >= 70 ? 'bg-amber-100 text-amber-800' : 'bg-blue-100 text-blue-800')"
                      >
                        {{ s.predictability_pct }}%
                      </span>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>

      <!-- ================= TAB 3: CUMULATIVE FLOW DIAGRAM (CFD) ================= -->
      <div v-else-if="reportStore.activeTab === 'cfd'" class="space-y-6">
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

      <!-- ================= TAB 4: TIMESHEET & WORKLOGS ================= -->
      <div v-else-if="reportStore.activeTab === 'timesheet'" class="space-y-6">
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

      <!-- ================= TAB 5: EPIC ROADMAP & BURNDOWN ================= -->
      <div v-else-if="reportStore.activeTab === 'epics'" class="space-y-6">
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
    </main>
  </div>
</template>
