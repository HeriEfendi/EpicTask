import { defineStore } from 'pinia';
import { ref } from 'vue';
import { api } from '@/services/api';

export const useReportStore = defineStore('report', () => {
  const loading = ref(false);
  const error = ref(null);

  // Active Report sub-tab
  // 'overview' | 'velocity' | 'cfd' | 'timesheet' | 'epics'
  const activeTab = ref('overview');

  // Filter states
  const dateRange = ref('all'); // 'all', 'q1', 'q2', 'q3', 'q4', 'last30'
  const userFilter = ref(null);

  // Data states
  const overview = ref(null);
  const velocity = ref(null);
  const cfd = ref(null);
  const timesheet = ref(null);
  const epicProgress = ref(null);

  async function fetchAllReports(projectId) {
    if (!projectId) return;
    loading.value = true;
    error.value = null;

    try {
      const [ovData, velData, cfdData, tsData, epData] = await Promise.all([
        api.get(`/projects/${projectId}/reports/overview`),
        api.get(`/projects/${projectId}/reports/velocity`),
        api.get(`/projects/${projectId}/reports/cumulative-flow`),
        api.get(`/projects/${projectId}/reports/timesheet`),
        api.get(`/projects/${projectId}/reports/epic-progress`),
      ]);

      overview.value = ovData;
      velocity.value = velData;
      cfd.value = cfdData;
      timesheet.value = tsData;
      epicProgress.value = epData;
    } catch (err) {
      console.error('Failed to load project reports', err);
      error.value = err.message || 'Gagal memuat data laporan';
    } finally {
      loading.value = false;
    }
  }

  function exportTimesheetCsv() {
    if (!timesheet.value || !timesheet.value.recent_logs) return;
    const logs = timesheet.value.recent_logs;
    const headers = ['ID', 'Tiket Key', 'Judul Tiket', 'User', 'Durasi (Jam)', 'Tanggal Log', 'Deskripsi'];
    const rows = logs.map((l) => [
      l.id,
      `"${l.issue_key}"`,
      `"${(l.issue_summary || '').replace(/"/g, '""')}"`,
      `"${l.user_name || 'Unassigned'}"`,
      l.hours.toFixed(2),
      l.logged_at ? new Date(l.logged_at).toISOString().split('T')[0] : '',
      `"${(l.description || '').replace(/"/g, '""')}"`,
    ]);

    const csvContent = 'data:text/csv;charset=utf-8,' + [headers.join(','), ...rows.map((e) => e.join(','))].join('\n');
    const encodedUri = encodeURI(csvContent);
    const link = document.createElement('a');
    link.setAttribute('href', encodedUri);
    link.setAttribute('download', `timesheet_report_${new Date().toISOString().split('T')[0]}.csv`);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  }

  return {
    loading,
    error,
    activeTab,
    dateRange,
    userFilter,
    overview,
    velocity,
    cfd,
    timesheet,
    epicProgress,
    fetchAllReports,
    exportTimesheetCsv,
  };
});
