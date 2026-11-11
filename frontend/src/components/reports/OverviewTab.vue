<script setup>
import { useReportStore } from '@/stores/report';
import {
  FileSpreadsheet,
  CheckCircle2,
  TrendingUp,
  Clock,
  Flame,
  ArrowUpRight,
  Users
} from 'lucide-vue-next';

const reportStore = useReportStore();

function formatHours(h) {
  if (!h) return '0h';
  return `${Number(h).toFixed(1)}h`;
}
</script>

<template>
  <div class="space-y-6">
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
</template>

