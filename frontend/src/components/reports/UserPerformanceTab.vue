<script setup>
import { onMounted, ref, watch } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useReportStore } from '@/stores/report';
import { RefreshCw, UserCheck } from 'lucide-vue-next';

import UserPerformanceFilterBar from './user_performance/UserPerformanceFilterBar.vue';
import UserPerformanceKpiGrid from './user_performance/UserPerformanceKpiGrid.vue';
import UserPerformanceBreakdown from './user_performance/UserPerformanceBreakdown.vue';
import UserPerformanceIssueList from './user_performance/UserPerformanceIssueList.vue';

const projectStore = useProjectStore();
const reportStore = useReportStore();

const currentUserId = ref(null);
const currentStartDate = ref(null);
const currentEndDate = ref(null);

onMounted(async () => {
  if (projectStore.currentProject?.id) {
    await loadReport();
  }
});

watch(
  () => projectStore.currentProject?.id,
  async (newPid) => {
    if (newPid) {
      await loadReport();
    }
  }
);

async function loadReport() {
  if (!projectStore.currentProject?.id) return;
  await reportStore.fetchUserPerformance(
    projectStore.currentProject.id,
    currentUserId.value,
    currentStartDate.value,
    currentEndDate.value
  );
}

function handleFilterChanged({ userId, startDate, endDate }) {
  currentUserId.value = userId;
  currentStartDate.value = startDate;
  currentEndDate.value = endDate;
  loadReport();
}
</script>

<template>
  <div class="space-y-6">
    <!-- Top Filter Controls -->
    <UserPerformanceFilterBar
      :available-users="reportStore.userPerformance?.available_users || []"
      :selected-user="reportStore.userPerformance?.selected_user"
      :loading="reportStore.userPerformanceLoading"
      @filter-changed="handleFilterChanged"
      @refresh="loadReport"
      @export-csv="reportStore.exportUserPerformanceCsv"
    />

    <!-- Loading State -->
    <div
      v-if="reportStore.userPerformanceLoading && !reportStore.userPerformance"
      class="flex flex-col items-center justify-center py-16 text-slate-500 bg-white rounded-xl border border-slate-200"
    >
      <RefreshCw class="w-7 h-7 animate-spin text-blue-600 mb-2" />
      <p class="text-xs font-semibold">Memuat metrik kinerja anggota tim...</p>
    </div>

    <!-- Main Content -->
    <template v-else-if="reportStore.userPerformance">
      <!-- 1. KPI Metric Grid -->
      <UserPerformanceKpiGrid
        :stats="reportStore.userPerformance.stats || {}"
      />

      <!-- 2. Visual Breakdowns by Type & Priority -->
      <UserPerformanceBreakdown
        :by-type="reportStore.userPerformance.by_type || []"
        :by-priority="reportStore.userPerformance.by_priority || []"
      />

      <!-- 3. Worked-on & Completed Issues Cards List -->
      <UserPerformanceIssueList
        :issues="reportStore.userPerformance.issues || []"
      />
    </template>
  </div>
</template>

