<script setup>
import { ref, watch } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useAuthStore } from '@/stores/auth';
import { Plus, X } from 'lucide-vue-next';

const projectStore = useProjectStore();
const authStore = useAuthStore();

const summary = ref('');
const description = ref('');
const issueType = ref('TASK');
const statusId = ref(null);
const priority = ref('MEDIUM');
const assigneeId = ref(null);
const epicId = ref(null);
const storyPoints = ref(0);
const startDate = ref('');
const dueDate = ref('');
const errorMessage = ref('');

watch(
  () => projectStore.isCreateModalOpen,
  (open) => {
    if (open) {
      summary.value = '';
      description.value = '';
      issueType.value = 'TASK';
      statusId.value = projectStore.defaultCreateStatusId || (projectStore.statuses[0]?.id || null);
      priority.value = 'MEDIUM';
      assigneeId.value = null;
      epicId.value = null;
      storyPoints.value = 0;
      startDate.value = '';
      dueDate.value = '';
      errorMessage.value = '';
    }
  }
);

async function handleCreate() {
  if (!summary.value.trim()) {
    errorMessage.value = 'Judul tiket (summary) wajib diisi';
    return;
  }

  try {
    await projectStore.createIssue({
      summary: summary.value.trim(),
      description: description.value.trim() || null,
      issue_type: issueType.value,
      status_id: statusId.value,
      priority: priority.value,
      assignee_id: assigneeId.value ? Number(assigneeId.value) : null,
      epic_id: epicId.value ? Number(epicId.value) : null,
      story_points: Number(storyPoints.value) || 0,
      start_date: startDate.value || null,
      due_date: dueDate.value || null,
    });
    projectStore.isCreateModalOpen = false;
  } catch (e) {
    errorMessage.value = e.message || 'Gagal membuat tiket';
  }
}
</script>

<template>
  <div
    v-if="projectStore.isCreateModalOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/40 backdrop-blur-sm"
  >
    <div class="glass-modal w-full max-w-2xl rounded-2xl border border-slate-200 bg-white p-6 flex flex-col shadow-2xl animate-slide-up max-h-[90vh] overflow-y-auto">
      <div class="flex items-center justify-between pb-3.5 border-b border-slate-200 mb-5">
        <h2 class="text-lg font-bold text-slate-900 flex items-center gap-2">
          <Plus class="w-5 h-5 text-blue-600" />
          <span>Buat Tiket Baru ({{ projectStore.currentProject?.key }})</span>
        </h2>
        <button
          @click="projectStore.isCreateModalOpen = false"
          class="p-1.5 rounded-lg text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <div v-if="errorMessage" class="mb-4 p-3.5 bg-red-50 border border-red-200 rounded-xl text-sm text-red-600">
        {{ errorMessage }}
      </div>

      <div class="space-y-4 text-sm">
        <!-- Issue Type & Status -->
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1.5">Tipe Tiket</label>
            <select
              v-model="issueType"
              class="w-full bg-white border border-slate-300 rounded-lg px-3.5 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs font-medium"
            >
              <option value="TASK">Task (Pekerjaan Standar)</option>
              <option value="STORY">Story (Fitur / User Story)</option>
              <option value="EPIC">Epic (Milestone Besar)</option>
              <option value="BUG">Bug (Kendala / Masalah)</option>
            </select>
          </div>

          <div>
            <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1.5">Status Awal</label>
            <select
              v-model="statusId"
              class="w-full bg-white border border-slate-300 rounded-lg px-3.5 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs font-medium"
            >
              <option v-for="s in projectStore.statuses" :key="s.id" :value="s.id">
                {{ s.name }} ({{ s.category }})
              </option>
            </select>
          </div>
        </div>

        <!-- Summary -->
        <div>
          <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1.5">Judul Ringkasan *</label>
          <input
            v-model="summary"
            placeholder="e.g. Implementasikan retry webhook atau fitur notifikasi..."
            class="w-full bg-white border border-slate-300 rounded-lg px-3.5 py-2.5 text-base text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 placeholder-slate-400 font-medium shadow-2xs"
            autofocus
            @keyup.enter="handleCreate"
          />
        </div>

        <!-- Description -->
        <div>
          <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1.5">Deskripsi Lengkap</label>
          <textarea
            v-model="description"
            rows="3"
            placeholder="Kebutuhan tiket, acceptance criteria, atau langkah pengerjaan..."
            class="w-full bg-white border border-slate-300 rounded-lg p-3.5 text-sm text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 placeholder-slate-400 shadow-2xs leading-relaxed"
          ></textarea>
        </div>

        <!-- Assignee & Priority -->
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1.5">Ditugaskan Kepada</label>
            <select
              v-model="assigneeId"
              class="w-full bg-white border border-slate-300 rounded-lg px-3.5 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-medium"
            >
              <option :value="null">Belum Ditugaskan</option>
              <option v-for="u in authStore.allUsers" :key="u.id" :value="u.id">
                {{ u.full_name }}
              </option>
            </select>
          </div>

          <div>
            <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1.5">Prioritas</label>
            <select
              v-model="priority"
              class="w-full bg-white border border-slate-300 rounded-lg px-3.5 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-semibold"
            >
              <option value="HIGHEST">🔴 Tertinggi (Highest)</option>
              <option value="HIGH">🟠 Tinggi (High)</option>
              <option value="MEDIUM">🟡 Sedang (Medium)</option>
              <option value="LOW">🔵 Rendah (Low)</option>
            </select>
          </div>
        </div>

        <!-- Epic & Story Points -->
        <div class="grid grid-cols-2 gap-4">
          <div v-if="issueType !== 'EPIC'">
            <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1.5">Parent Epic</label>
            <select
              v-model="epicId"
              class="w-full bg-white border border-slate-300 rounded-lg px-3.5 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-medium"
            >
              <option :value="null">Tanpa Epic</option>
              <option v-for="e in projectStore.epics" :key="e.id" :value="e.id">
                {{ e.summary }}
              </option>
            </select>
          </div>

          <div>
            <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1.5">Story Points (Estimasi)</label>
            <input
              v-model.number="storyPoints"
              type="number"
              min="0"
              class="w-full bg-white border border-slate-300 rounded-lg px-3.5 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-semibold"
            />
          </div>
        </div>

        <!-- Start Date & Due Date -->
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1.5">Tanggal Mulai</label>
            <input
              v-model="startDate"
              type="date"
              class="w-full bg-white border border-slate-300 rounded-lg px-3.5 py-2 text-sm text-slate-800 focus:outline-none shadow-2xs font-medium"
            />
          </div>

          <div>
            <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1.5">Tenggat Waktu (Due)</label>
            <input
              v-model="dueDate"
              type="date"
              class="w-full bg-white border border-slate-300 rounded-lg px-3.5 py-2 text-sm text-slate-800 focus:outline-none shadow-2xs font-medium"
            />
          </div>
        </div>

        <!-- Actions -->
        <div class="flex justify-end space-x-3 pt-4 border-t border-slate-200">
          <button
            @click="projectStore.isCreateModalOpen = false"
            class="px-4 py-2 text-sm font-medium text-slate-600 hover:text-slate-800 hover:bg-slate-100 rounded-lg transition"
          >
            Batal
          </button>
          <button
            @click="handleCreate"
            class="px-6 py-2 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-lg text-sm font-semibold shadow-xs transition"
          >
            Buat Tiket
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
