<script setup>
import { onMounted, watch } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useReportStore } from '@/stores/report';
import {
  BarChart3,
  TrendingUp,
  Clock,
  Layers,
  Calendar,
  Download,
  RefreshCw,
  UserCheck
} from 'lucide-vue-next';

import OverviewTab from './OverviewTab.vue';
import VelocityTab from './VelocityTab.vue';
import CumulativeFlowTab from './CumulativeFlowTab.vue';
import TimesheetTab from './TimesheetTab.vue';
import EpicProgressTab from './EpicProgressTab.vue';
import UserPerformanceTab from './UserPerformanceTab.vue';

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
            <span>Ekspor CSV</span>
          </button>
        </div>
      </div>

      <!-- Navigation Tabs -->
      <nav class="flex items-center space-x-1 mt-5 border-t border-slate-100 pt-3 text-xs font-bold overflow-x-auto select-none">
        <button
          @click="reportStore.activeTab = 'overview'"
          class="px-3.5 py-2 rounded-lg flex items-center space-x-1.5 transition whitespace-nowrap"
          :class="reportStore.activeTab === 'overview' ? 'bg-blue-50 text-blue-700 border border-blue-200/80 shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          <BarChart3 class="w-3.5 h-3.5" />
          <span>Executive Overview</span>
        </button>

        <button
          @click="reportStore.activeTab = 'velocity'"
          class="px-3.5 py-2 rounded-lg flex items-center space-x-1.5 transition whitespace-nowrap"
          :class="reportStore.activeTab === 'velocity' ? 'bg-blue-50 text-blue-700 border border-blue-200/80 shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          <TrendingUp class="w-3.5 h-3.5" />
          <span>Sprint Velocity</span>
        </button>

        <button
          @click="reportStore.activeTab = 'cfd'"
          class="px-3.5 py-2 rounded-lg flex items-center space-x-1.5 transition whitespace-nowrap"
          :class="reportStore.activeTab === 'cfd' ? 'bg-blue-50 text-blue-700 border border-blue-200/80 shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          <Layers class="w-3.5 h-3.5" />
          <span>Cumulative Flow (CFD)</span>
        </button>

        <button
          @click="reportStore.activeTab = 'timesheet'"
          class="px-3.5 py-2 rounded-lg flex items-center space-x-1.5 transition whitespace-nowrap"
          :class="reportStore.activeTab === 'timesheet' ? 'bg-blue-50 text-blue-700 border border-blue-200/80 shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          <Clock class="w-3.5 h-3.5" />
          <span>Timesheet & Work Logs</span>
        </button>

        <button
          @click="reportStore.activeTab = 'epics'"
          class="px-3.5 py-2 rounded-lg flex items-center space-x-1.5 transition whitespace-nowrap"
          :class="reportStore.activeTab === 'epics' ? 'bg-blue-50 text-blue-700 border border-blue-200/80 shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          <Calendar class="w-3.5 h-3.5" />
          <span>Epic Progress</span>
        </button>

        <button
          @click="reportStore.activeTab = 'user-performance'"
          class="px-3.5 py-2 rounded-lg flex items-center space-x-1.5 transition whitespace-nowrap"
          :class="reportStore.activeTab === 'user-performance' ? 'bg-blue-50 text-blue-700 border border-blue-200/80 shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          <UserCheck class="w-3.5 h-3.5" />
          <span>Performa Anggota</span>
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

      <!-- Tab Content Components -->
      <OverviewTab v-else-if="reportStore.activeTab === 'overview'" />
      <VelocityTab v-else-if="reportStore.activeTab === 'velocity'" />
      <CumulativeFlowTab v-else-if="reportStore.activeTab === 'cfd'" />
      <TimesheetTab v-else-if="reportStore.activeTab === 'timesheet'" />
      <EpicProgressTab v-else-if="reportStore.activeTab === 'epics'" />
      <UserPerformanceTab v-else-if="reportStore.activeTab === 'user-performance'" />
    </main>
  </div>
</template>
