<script setup>
import { ref, watch } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useAuthStore } from '@/stores/auth';
import {
  Plus,
  X,
  Layers,
  Calendar,
  User,
  Flag,
  Hash,
  AlertCircle,
  Loader2,
  Sparkles
} from 'lucide-vue-next';
import RichTextEditor from '@/components/common/RichTextEditor.vue';

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
const isSubmitting = ref(false);

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
      isSubmitting.value = false;
    }
  }
);

async function handleCreate() {
  if (!summary.value.trim()) {
    errorMessage.value = 'Judul ringkasan tiket wajib diisi.';
    return;
  }

  isSubmitting.value = true;
  errorMessage.value = '';

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
    errorMessage.value = e.message || 'Gagal membuat tiket baru';
  } finally {
    isSubmitting.value = false;
  }
}
</script>

<template>
  <div
    v-if="projectStore.isCreateModalOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-3 sm:p-5 bg-slate-900/50 backdrop-blur-sm"
    @keydown.esc="projectStore.isCreateModalOpen = false"
  >
    <div
      class="glass-modal w-[96vw] max-w-[1440px] rounded-2xl border border-slate-200 bg-white h-[92vh] max-h-[95vh] flex flex-col shadow-2xl animate-slide-up overflow-hidden"
    >
      <!-- Modal Top Header -->
      <div class="px-6 py-4 border-b border-slate-200 bg-slate-50 flex items-center justify-between select-none">
        <div class="flex items-center space-x-3">
          <div class="w-8 h-8 rounded-lg bg-blue-600 text-white flex items-center justify-center shadow-xs">
            <Plus class="w-5 h-5" />
          </div>
          <div>
            <h2 class="text-base font-bold text-slate-900 flex items-center gap-2">
              <span>Buat Tiket Baru</span>
              <span
                v-if="projectStore.currentProject"
                class="text-xs px-2.5 py-0.5 rounded-full bg-blue-50 text-blue-700 border border-blue-200 font-mono font-semibold"
              >
                {{ projectStore.currentProject.key }} — {{ projectStore.currentProject.name }}
              </span>
            </h2>
          </div>
        </div>

        <button
          @click="projectStore.isCreateModalOpen = false"
          class="p-2 rounded-lg text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition"
          title="Tutup (Esc)"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Error Message Banner -->
      <div
        v-if="errorMessage"
        class="px-6 py-3 bg-red-50 border-b border-red-200 text-sm text-red-700 flex items-center gap-2"
      >
        <AlertCircle class="w-4 h-4 text-red-600 flex-shrink-0" />
        <span>{{ errorMessage }}</span>
      </div>

      <!-- Main Content Grid (2 Columns matching Detail Modal: Left 75%, Right 25%) -->
      <div class="flex-1 overflow-hidden grid grid-cols-1 lg:grid-cols-12 divide-y lg:divide-y-0 lg:divide-x divide-slate-200">
        
        <!-- LEFT COLUMN: Summary & Rich Text Description -->
        <div class="lg:col-span-8 xl:col-span-9 p-6 space-y-6 overflow-y-auto h-full">
          <!-- Summary (Title) Input -->
          <div class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider">
              Ringkasan Tiket <span class="text-red-500">*</span>
            </label>
            <input
              v-model="summary"
              class="w-full text-xl font-bold text-slate-900 bg-white border border-slate-300 focus:border-blue-600 focus:ring-2 focus:ring-blue-100 rounded-xl px-4 py-3 transition focus:outline-none placeholder-slate-400 shadow-2xs"
              placeholder="e.g. Implementasikan retry webhook atau fitur export laporan..."
              autofocus
              @keyup.enter="handleCreate"
            />
          </div>

          <!-- Description Section (Rich Text Editor with Jira/Word features) -->
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider">
                Deskripsi Lengkap
              </label>
              <span class="text-xs text-slate-500 hidden sm:inline flex items-center gap-1">
                <Sparkles class="w-3.5 h-3.5 text-blue-600" />
                Mendukung gaya Word, tabel spesifikasi, kode & lampiran gambar (Paste Ctrl+V)
              </span>
            </div>

            <!-- Rich WYSIWYG Editor -->
            <RichTextEditor
              v-model="description"
              placeholder="Jelaskan kebutuhan tiket, kriteria penerimaan (acceptance criteria), tabel spesifikasi teknis, atau lampiran tangkapan layar..."
            />
          </div>
        </div>

        <!-- RIGHT COLUMN: Sidebar Metadata -->
        <div class="lg:col-span-4 xl:col-span-3 p-6 space-y-5 bg-slate-50/70 overflow-y-auto h-full text-sm">
          <div class="font-bold text-xs text-slate-500 uppercase tracking-wider pb-2 border-b border-slate-200 flex items-center gap-1.5">
            <Layers class="w-3.5 h-3.5 text-slate-600" />
            <span>Atribut Tiket</span>
          </div>

          <!-- Issue Type -->
          <div class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider">Tipe Tiket</label>
            <select
              v-model="issueType"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs font-semibold"
            >
              <option value="TASK">📋 Task (Pekerjaan Standar)</option>
              <option value="STORY">📖 Story (Fitur / User Story)</option>
              <option value="EPIC">🟣 Epic (Milestone Besar)</option>
              <option value="BUG">🐞 Bug (Kendala / Masalah)</option>
            </select>
          </div>

          <!-- Initial Status -->
          <div class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider">Status Awal</label>
            <select
              v-model="statusId"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs font-medium"
            >
              <option v-for="s in projectStore.statuses" :key="s.id" :value="s.id">
                {{ s.name }} ({{ s.category }})
              </option>
            </select>
          </div>

          <!-- Priority -->
          <div class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1">
              <Flag class="w-3 h-3 text-slate-500" />
              <span>Prioritas</span>
            </label>
            <select
              v-model="priority"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs font-semibold"
            >
              <option value="HIGHEST">🔴 Tertinggi (Highest)</option>
              <option value="HIGH">🟠 Tinggi (High)</option>
              <option value="MEDIUM">🟡 Sedang (Medium)</option>
              <option value="LOW">🔵 Rendah (Low)</option>
            </select>
          </div>

          <!-- Assignee -->
          <div class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1">
              <User class="w-3 h-3 text-slate-500" />
              <span>Ditugaskan Kepada</span>
            </label>
            <select
              v-model="assigneeId"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs font-medium"
            >
              <option :value="null">Belum Ditugaskan</option>
              <option v-for="u in authStore.allUsers" :key="u.id" :value="u.id">
                {{ u.full_name }} ({{ u.email }})
              </option>
            </select>
          </div>

          <!-- Parent Epic (if not creating an epic) -->
          <div v-if="issueType !== 'EPIC'" class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider">Parent Epic</label>
            <select
              v-model="epicId"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs font-medium"
            >
              <option :value="null">Tanpa Epic</option>
              <option v-for="e in projectStore.epics" :key="e.id" :value="e.id">
                🟣 {{ e.summary }}
              </option>
            </select>
          </div>

          <!-- Story Points -->
          <div class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1">
              <Hash class="w-3 h-3 text-slate-500" />
              <span>Story Points (Estimasi)</span>
            </label>
            <input
              v-model.number="storyPoints"
              type="number"
              min="0"
              placeholder="0"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs font-semibold"
            />
          </div>

          <!-- Dates (Start & Due) -->
          <div class="space-y-3 pt-2 border-t border-slate-200">
            <div class="space-y-1.5">
              <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1">
                <Calendar class="w-3 h-3 text-slate-500" />
                <span>Tanggal Mulai</span>
              </label>
              <input
                v-model="startDate"
                type="date"
                class="w-full bg-white border border-slate-300 rounded-lg px-3 py-1.5 text-sm text-slate-800 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
              />
            </div>

            <div class="space-y-1.5">
              <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1">
                <Calendar class="w-3 h-3 text-slate-500" />
                <span>Tenggat Waktu (Due Date)</span>
              </label>
              <input
                v-model="dueDate"
                type="date"
                class="w-full bg-white border border-slate-300 rounded-lg px-3 py-1.5 text-sm text-slate-800 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
              />
            </div>
          </div>
        </div>
      </div>

      <!-- Modal Bottom Actions Bar -->
      <div class="px-6 py-4 border-t border-slate-200 bg-slate-50 flex items-center justify-between select-none">
        <span class="text-xs text-slate-400 hidden sm:inline">
          Tekan <kbd class="px-1.5 py-0.5 bg-slate-200 border border-slate-300 rounded text-[10px] font-mono text-slate-700">Enter</kbd> di judul untuk membuat cepat, atau <kbd class="px-1.5 py-0.5 bg-slate-200 border border-slate-300 rounded text-[10px] font-mono text-slate-700">Esc</kbd> untuk membatalkan.
        </span>

        <div class="flex items-center space-x-3 ml-auto">
          <button
            type="button"
            @click="projectStore.isCreateModalOpen = false"
            class="px-4 py-2 text-sm font-semibold text-slate-600 hover:text-slate-900 hover:bg-slate-200/60 rounded-xl transition"
            :disabled="isSubmitting"
          >
            Batal
          </button>
          <button
            type="button"
            @click="handleCreate"
            class="px-6 py-2 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-xl text-sm font-bold shadow-sm hover:shadow transition flex items-center gap-2 active:scale-98 disabled:opacity-50 disabled:pointer-events-none"
            :disabled="isSubmitting"
          >
            <Loader2 v-if="isSubmitting" class="w-4 h-4 animate-spin" />
            <Plus v-else class="w-4 h-4" />
            <span>{{ isSubmitting ? 'Menyimpan...' : 'Buat Tiket' }}</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
