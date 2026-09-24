<script setup>
import { ref } from 'vue';
import { useProjectStore } from '@/stores/project';
import {
  Workflow,
  Plus,
  Trash2,
  ArrowRight,
  ShieldCheck,
  X
} from 'lucide-vue-next';

const props = defineProps({
  isOpen: Boolean,
});

const emit = defineEmits(['close']);

const projectStore = useProjectStore();

// New Status State
const newStatusName = ref('');
const newStatusCategory = ref('IN_PROGRESS');
const newStatusColor = ref('#3b82f6');

// New Transition State
const fromStatusId = ref(null);
const toStatusId = ref(null);

async function handleCreateStatus() {
  if (!newStatusName.value.trim()) return;
  try {
    await projectStore.createStatus(
      newStatusName.value.trim(),
      newStatusCategory.value,
      newStatusColor.value
    );
    newStatusName.value = '';
  } catch (e) {
    alert(e.message || 'Gagal membuat status');
  }
}

async function handleCreateTransition() {
  if (!fromStatusId.value || !toStatusId.value) return;
  if (fromStatusId.value === toStatusId.value) {
    alert('Status asal dan status tujuan tidak boleh sama');
    return;
  }
  try {
    await projectStore.createTransition(Number(fromStatusId.value), Number(toStatusId.value));
    fromStatusId.value = null;
    toStatusId.value = null;
  } catch (e) {
    alert(e.message || 'Gagal menyimpan aturan transisi workflow');
  }
}

async function handleDeleteTransition(tid) {
  try {
    await projectStore.deleteTransition(tid);
  } catch (e) {
    alert(e.message || 'Gagal menghapus aturan transisi');
  }
}
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/40 backdrop-blur-sm"
  >
    <div class="glass-modal w-full max-w-2xl rounded-2xl p-6 border border-slate-200 bg-white max-h-[90vh] flex flex-col shadow-2xl animate-slide-up">
      <!-- Header -->
      <div class="flex items-center justify-between pb-4 border-b border-slate-200">
        <div class="flex items-center space-x-3">
          <div class="w-9 h-9 rounded-xl bg-blue-50 border border-blue-200 flex items-center justify-center text-blue-600">
            <Workflow class="w-5 h-5" />
          </div>
          <div>
            <h2 class="text-lg font-bold text-slate-900">Workflow Kustom & Guard Transisi</h2>
            <p class="text-sm text-slate-500">Kelola status kolom board dan tegakkan aturan perpindahan tiket bertahap</p>
          </div>
        </div>

        <button
          @click="emit('close')"
          class="p-1.5 rounded-lg text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto py-4 space-y-6 text-sm">
        <!-- Info Alert -->
        <div class="bg-blue-50 border border-blue-200 rounded-xl p-3.5 flex items-start space-x-3 text-sm text-blue-900">
          <ShieldCheck class="w-5 h-5 text-blue-600 shrink-0 mt-0.5" />
          <div>
            <p class="font-bold text-blue-900">Mesin Aturan Transisi Aktif</p>
            <p class="text-xs text-blue-700 mt-0.5">
              Ketika ada aturan transisi yang didaftarkan untuk suatu status, tiket TIDAK DAPAT ditarik ke status lain di luar daftar yang diizinkan (misal: "To Do" hanya boleh ke "In Progress").
            </p>
          </div>
        </div>

        <!-- Section 1: Dynamic Status Columns (FR-2.3) -->
        <div class="space-y-3">
          <h3 class="text-xs font-bold text-slate-600 uppercase tracking-wider">
            Status Projek (Kolom Board)
          </h3>

          <div class="grid grid-cols-2 sm:grid-cols-3 gap-2.5">
            <div
              v-for="status in projectStore.statuses"
              :key="status.id"
              class="p-3 rounded-xl border border-slate-200 bg-slate-50 flex items-center justify-between shadow-2xs"
            >
              <div class="flex items-center space-x-2.5 truncate">
                <span class="w-3.5 h-3.5 rounded-full shrink-0" :style="{ backgroundColor: status.color || '#3b82f6' }"></span>
                <span class="text-sm font-semibold text-slate-800 truncate">{{ status.name }}</span>
              </div>
              <span class="text-xs uppercase px-2 py-0.5 rounded font-mono font-semibold bg-white border border-slate-200 text-slate-600">
                {{ status.category }}
              </span>
            </div>
          </div>

          <!-- Add New Status Form -->
          <div class="p-3.5 bg-slate-50 border border-slate-200 rounded-xl flex flex-wrap items-center gap-2.5">
            <input
              v-model="newStatusName"
              placeholder="Nama status baru (e.g. QA Testing)..."
              class="flex-1 min-w-[170px] bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
            />
            <select
              v-model="newStatusCategory"
              class="bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-700 focus:outline-none shadow-2xs font-medium"
            >
              <option value="TODO">Kategori: TODO</option>
              <option value="IN_PROGRESS">Kategori: IN PROGRESS</option>
              <option value="DONE">Kategori: DONE</option>
            </select>
            <input
              v-model="newStatusColor"
              type="color"
              class="w-9 h-9 rounded-lg border border-slate-300 bg-white cursor-pointer p-0.5 shadow-2xs"
              title="Pilih warna kolom"
            />
            <button
              @click="handleCreateStatus"
              class="px-4 py-2 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-lg text-sm font-semibold flex items-center space-x-1.5 shadow-xs transition"
            >
              <Plus class="w-4 h-4" />
              <span>Tambah Status</span>
            </button>
          </div>
        </div>

        <!-- Section 2: Workflow Transition Guards (FR-2.3) -->
        <div class="space-y-3">
          <h3 class="text-xs font-bold text-slate-600 uppercase tracking-wider">
            Daftar Aturan Transisi yang Diizinkan
          </h3>

          <div v-if="projectStore.transitions.length === 0" class="p-3.5 text-sm text-slate-500 italic bg-slate-50 rounded-xl border border-slate-200">
            Belum ada aturan pembatasan. Tiket dapat dipindahkan bebas ke semua kolom. Tambahkan aturan di bawah untuk mengunci transisi!
          </div>

          <div v-else class="space-y-2 max-h-48 overflow-y-auto">
            <div
              v-for="t in projectStore.transitions"
              :key="t.id"
              class="p-3 rounded-xl border border-slate-200 bg-white flex items-center justify-between text-sm shadow-2xs"
            >
              <div class="flex items-center space-x-3">
                <span class="font-bold text-blue-700 px-2.5 py-0.5 rounded-lg bg-blue-50 border border-blue-200 text-xs">
                  {{ t.from_status_name }}
                </span>
                <ArrowRight class="w-4 h-4 text-slate-400" />
                <span class="font-bold text-emerald-700 px-2.5 py-0.5 rounded-lg bg-emerald-50 border border-emerald-200 text-xs">
                  {{ t.to_status_name }}
                </span>
              </div>

              <button
                @click="handleDeleteTransition(t.id)"
                class="p-1.5 text-slate-400 hover:text-red-600 hover:bg-red-50 rounded-lg transition"
                title="Hapus aturan"
              >
                <Trash2 class="w-4 h-4" />
              </button>
            </div>
          </div>

          <!-- Add Transition Form -->
          <div class="p-3.5 bg-slate-50 border border-slate-200 rounded-xl flex flex-wrap items-center gap-2.5">
            <span class="text-sm text-slate-600 font-medium">Boleh pindah dari</span>
            <select
              v-model="fromStatusId"
              class="bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-700 focus:outline-none shadow-2xs font-medium"
            >
              <option :value="null">Pilih status asal...</option>
              <option v-for="s in projectStore.statuses" :key="s.id" :value="s.id">
                {{ s.name }}
              </option>
            </select>

            <span class="text-sm text-slate-600 font-medium">ke</span>
            <select
              v-model="toStatusId"
              class="bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-700 focus:outline-none shadow-2xs font-medium"
            >
              <option :value="null">Pilih status tujuan...</option>
              <option v-for="s in projectStore.statuses" :key="s.id" :value="s.id">
                {{ s.name }}
              </option>
            </select>

            <button
              @click="handleCreateTransition"
              class="px-4 py-2 bg-emerald-600 hover:bg-emerald-700 active:bg-emerald-800 text-white rounded-lg text-sm font-semibold flex items-center space-x-1.5 shadow-xs transition"
            >
              <Plus class="w-4 h-4" />
              <span>Izinkan Transisi</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
