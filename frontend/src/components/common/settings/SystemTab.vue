<script setup>
import { ref } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { useProjectStore } from '@/stores/project';
import { api } from '@/services/api';
import { toast } from '@/utils/toast';
import { RefreshCw, CheckCircle2 } from 'lucide-vue-next';

const authStore = useAuthStore();
const projectStore = useProjectStore();

const isResetting = ref(false);
const resetSuccess = ref(false);

async function handleResetDemoData() {
  if (!confirm('Apakah Anda yakin ingin mereset data demo ke default? Semua tiket dan perubahan akan dikembalikan ke data awal.')) {
    return;
  }
  isResetting.value = true;
  resetSuccess.value = false;
  try {
    await api.post('/seed');
    resetSuccess.value = true;
    toast.success('Data demo berhasil direset ke kondisi awal');
    await authStore.fetchMe();
    if (authStore.currentWorkspace) {
      await projectStore.fetchProjects(authStore.currentWorkspace.id);
    }
  } catch (e) {
    toast.error('Gagal mereset data: ' + (e.message || 'Error'));
  } finally {
    isResetting.value = false;
  }
}
</script>

<template>
  <div class="space-y-6">
    <div class="p-4 bg-slate-50 rounded-xl border border-slate-200 space-y-2">
      <h4 class="text-sm font-bold text-slate-900">Tentang Aplikasi</h4>
      <div class="grid grid-cols-2 gap-3 text-xs">
        <div>
          <span class="text-slate-400">Versi:</span>
          <span class="font-semibold text-slate-700 ml-1.5">EpicTask v0.1.3</span>
        </div>
        <div>
          <span class="text-slate-400">Backend:</span>
          <span class="font-semibold text-slate-700 ml-1.5">Rust (Axum + SQLx)</span>
        </div>
        <div>
          <span class="text-slate-400">Database:</span>
          <span class="font-semibold text-slate-700 ml-1.5">MariaDB / MySQL</span>
        </div>
        <div>
          <span class="text-slate-400">Frontend:</span>
          <span class="font-semibold text-slate-700 ml-1.5">Vue 3 + Vite + Tailwind</span>
        </div>
      </div>
    </div>

    <!-- Reset Seed Database -->
    <div class="p-4 bg-red-50/50 rounded-xl border border-red-200 space-y-3">
      <div>
        <h4 class="text-sm font-bold text-red-900">Reset Demo Data</h4>
        <p class="text-xs text-red-700 mt-0.5">
          Mengembalikan status projek, tiket contoh, dan rule otomatisasi ke kondisi awal bawaan database.
        </p>
      </div>

      <div class="flex items-center space-x-3">
        <button
          @click="handleResetDemoData"
          :disabled="isResetting"
          class="px-4 py-2 bg-red-600 hover:bg-red-700 disabled:opacity-50 text-white rounded-lg text-sm font-semibold transition flex items-center space-x-1.5"
        >
          <RefreshCw class="w-4 h-4" :class="isResetting ? 'animate-spin' : ''" />
          <span>{{ isResetting ? 'Mereset Database...' : 'Reset ke Data Demo' }}</span>
        </button>

        <span
          v-if="resetSuccess"
          class="text-xs font-semibold text-green-600 flex items-center space-x-1"
        >
          <CheckCircle2 class="w-4 h-4" />
          <span>Berhasil direset!</span>
        </span>
      </div>
    </div>
  </div>
</template>

