<script setup>
import { ref, computed, watch } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useAuthStore } from '@/stores/auth';
import {
  X,
  Clock,
  CheckSquare,
  MessageSquare,
  Trash2,
  Plus,
  Send,
  Sparkles,
  Layers,
  Flag,
  User,
  Calendar,
  Hash,
  AlertCircle,
  Loader2,
  CheckCircle2
} from 'lucide-vue-next';
import RichTextEditor from '@/components/common/RichTextEditor.vue';
import { toast } from '@/utils/toast';

const projectStore = useProjectStore();
const authStore = useAuthStore();

// Mode detection
const isCreateMode = computed(() => projectStore.isCreateModalOpen);
const isDetailMode = computed(() => projectStore.isDetailModalOpen && !!projectStore.activeIssue);
const isOpen = computed(() => isCreateMode.value || isDetailMode.value);

const issue = computed(() => projectStore.activeIssue);

// --- Create Form State ---
const createSummary = ref('');
const createDescription = ref('');
const createType = ref('TASK');
const createStatusId = ref(null);
const createPriority = ref('MEDIUM');
const createAssigneeId = ref(null);
const createEpicId = ref(null);
const createStoryPoints = ref(0);
const createStartDate = ref('');
const createDueDate = ref('');
const createErrorMessage = ref('');
const isSubmitting = ref(false);

// --- Detail / Activity State ---
const activeTab = ref('comments'); // 'comments' | 'timelog'
const newCommentText = ref('');
const newSubtaskSummary = ref('');

// Log work state
const isLoggingWork = ref(false);
const logHours = ref(1);
const logDescription = ref('');

// Watch for create modal opening to reset form
watch(
  () => projectStore.isCreateModalOpen,
  (open) => {
    if (open) {
      createSummary.value = '';
      createDescription.value = '';
      createType.value = 'TASK';
      createStatusId.value = projectStore.defaultCreateStatusId || (projectStore.statuses[0]?.id || null);
      createPriority.value = 'MEDIUM';
      createAssigneeId.value = null;
      createEpicId.value = null;
      createStoryPoints.value = 0;
      createStartDate.value = '';
      createDueDate.value = '';
      createErrorMessage.value = '';
      isSubmitting.value = false;
    }
  }
);

// Subtasks calculation
const subtaskStats = computed(() => {
  const list = projectStore.activeIssueSubtasks || [];
  if (list.length === 0) return null;
  const done = list.filter((s) => s.status_category === 'DONE').length;
  const percent = Math.round((done / list.length) * 100);
  return { total: list.length, done, percent };
});

function handleClose() {
  if (isCreateMode.value) {
    projectStore.isCreateModalOpen = false;
  }
  if (isDetailMode.value || projectStore.isDetailModalOpen) {
    projectStore.closeDetailModal();
  }
}

// --- Create Issue Handler ---
async function handleCreateIssue() {
  if (!createSummary.value.trim()) {
    createErrorMessage.value = 'Judul ringkasan tiket wajib diisi.';
    toast.warning('Judul ringkasan tiket wajib diisi.');
    return;
  }

  isSubmitting.value = true;
  createErrorMessage.value = '';

  try {
    const newIssue = await projectStore.createIssue({
      summary: createSummary.value.trim(),
      description: createDescription.value.trim() || null,
      issue_type: createType.value,
      status_id: createStatusId.value,
      priority: createPriority.value,
      assignee_id: createAssigneeId.value ? Number(createAssigneeId.value) : null,
      epic_id: createEpicId.value ? Number(createEpicId.value) : null,
      story_points: Number(createStoryPoints.value) || 0,
      start_date: createStartDate.value || null,
      due_date: createDueDate.value || null,
    });
    toast.success(`Tiket ${newIssue?.key || ''} berhasil dibuat!`);
    projectStore.isCreateModalOpen = false;
  } catch (e) {
    createErrorMessage.value = e.message || 'Gagal membuat tiket baru';
    toast.error(e.message || 'Gagal membuat tiket baru');
  } finally {
    isSubmitting.value = false;
  }
}

// --- Update Field in Detail Mode ---
async function handleUpdateField(fields) {
  if (!issue.value) return;
  try {
    await projectStore.updateIssue(issue.value.id, fields);
  } catch (e) {
    toast.error(e.message || 'Gagal memperbarui tiket');
  }
}

async function handleStatusChange(e) {
  const newStatusId = Number(e.target.value);
  if (!issue.value || newStatusId === issue.value.status_id) return;
  try {
    await projectStore.moveIssue(issue.value.id, newStatusId);
    toast.success('Status tiket berhasil diperbarui');
  } catch (e) {
    toast.error(e.message || 'Pelanggaran aturan transisi alur kerja');
    e.target.value = issue.value.status_id;
  }
}

async function handleAddComment() {
  if (!newCommentText.value.trim() || !issue.value) return;
  try {
    await projectStore.addComment(issue.value.id, newCommentText.value.trim());
    toast.success('Komentar berhasil ditambahkan');
    newCommentText.value = '';
  } catch (e) {
    toast.error(e.message || 'Gagal menambahkan komentar');
  }
}

async function handleAddSubtask() {
  if (!newSubtaskSummary.value.trim() || !issue.value) return;
  try {
    await projectStore.createIssue({
      summary: newSubtaskSummary.value.trim(),
      issue_type: 'SUBTASK',
      parent_id: issue.value.id,
      epic_id: issue.value.epic_id || (issue.value.issue_type === 'EPIC' ? issue.value.id : null),
      status_id: issue.value.status_id,
      priority: 'MEDIUM',
    });
    toast.success('Subtask berhasil ditambahkan');
    newSubtaskSummary.value = '';
    await projectStore.openIssueDetail(issue.value.id);
  } catch (e) {
    toast.error(e.message || 'Gagal menambahkan subtask');
  }
}

async function toggleSubtaskDone(subtask) {
  const isDone = subtask.status_category === 'DONE';
  const targetCategory = isDone ? 'TODO' : 'DONE';
  const targetStatus = projectStore.statuses.find((s) => s.category === targetCategory) || projectStore.statuses[0];

  try {
    await projectStore.moveIssue(subtask.id, targetStatus.id);
    await projectStore.openIssueDetail(issue.value.id);
  } catch (e) {
    toast.error(e.message || 'Gagal memperbarui status subtask');
  }
}

async function handleLogWork() {
  if (logHours.value <= 0 || !issue.value) return;
  try {
    const seconds = Math.round(logHours.value * 3600);
    await projectStore.logTime(issue.value.id, seconds, logDescription.value.trim());
    toast.success(`Berhasil mencatat ${logHours.value} jam kerja`);
    isLoggingWork.value = false;
    logHours.value = 1;
    logDescription.value = '';
  } catch (e) {
    toast.error(e.message || 'Gagal mencatat waktu pengerjaan');
  }
}

async function handleDelete() {
  if (!issue.value) return;
  if (confirm(`Yakin ingin menghapus tiket ${issue.value.key}?`)) {
    try {
      await projectStore.deleteIssue(issue.value.id);
      toast.info(`Tiket ${issue.value.key} berhasil dihapus`);
    } catch (e) {
      toast.error(e.message || 'Gagal menghapus tiket');
    }
  }
}
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-3 sm:p-5 bg-slate-900/50 backdrop-blur-sm"
    @keydown.esc="handleClose"
  >
    <div
      class="glass-modal w-[96vw] max-w-[1440px] rounded-2xl border border-slate-200 bg-white h-[94vh] max-h-[96vh] flex flex-col shadow-2xl animate-slide-up overflow-hidden"
    >
      <!-- ── MODAL HEADER ── -->
      <div class="px-6 py-4 border-b border-slate-200 bg-slate-50 flex items-center justify-between select-none">
        
        <!-- Header Left: Create vs Edit Mode -->
        <div class="flex items-center space-x-3">
          <!-- CREATE MODE Header -->
          <template v-if="isCreateMode">
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
          </template>

          <!-- EDIT / DETAIL MODE Header -->
          <template v-else-if="issue">
            <!-- Type Badge -->
            <span
              class="text-xs font-semibold px-2.5 py-0.5 rounded uppercase tracking-wider"
              :class="`badge-type-${issue.issue_type}`"
            >
              {{ issue.issue_type }}
            </span>

            <!-- Key with link -->
            <span class="font-mono text-base font-bold text-blue-600">
              {{ issue.key }}
            </span>

            <!-- Epic link if present -->
            <span
              v-if="issue.epic_summary && issue.issue_type !== 'EPIC'"
              class="text-xs px-2.5 py-0.5 rounded-full bg-purple-50 text-purple-700 border border-purple-200 font-semibold"
            >
              🟣 {{ issue.epic_summary }}
            </span>

            <!-- Parent link if present -->
            <span v-if="issue.parent_key" class="text-sm text-slate-500">
              Subtask dari <strong class="text-slate-800">{{ issue.parent_key }}</strong>
            </span>
          </template>
        </div>

        <!-- Header Right: Actions & Close Button -->
        <div class="flex items-center space-x-2">
          <!-- Delete button (Detail mode only) -->
          <button
            v-if="isDetailMode && issue"
            @click="handleDelete"
            class="p-2 rounded-lg text-slate-400 hover:text-red-600 hover:bg-red-50 transition"
            title="Hapus tiket ini"
          >
            <Trash2 class="w-4 h-4" />
          </button>

          <!-- Close Modal -->
          <button
            @click="handleClose"
            class="p-2 rounded-lg text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition"
            title="Tutup (Esc)"
          >
            <X class="w-5 h-5" />
          </button>
        </div>
      </div>

      <!-- Error Message Banner (Create Mode) -->
      <div
        v-if="isCreateMode && createErrorMessage"
        class="px-6 py-3 bg-red-50 border-b border-red-200 text-sm text-red-700 flex items-center gap-2"
      >
        <AlertCircle class="w-4 h-4 text-red-600 flex-shrink-0" />
        <span>{{ createErrorMessage }}</span>
      </div>

      <!-- ── MAIN CONTENT GRID: 2 COLUMNS (Left 75%, Right 25%) ── -->
      <div class="flex-1 overflow-hidden grid grid-cols-1 lg:grid-cols-12 divide-y lg:divide-y-0 lg:divide-x divide-slate-200">
        
        <!-- ════════ LEFT COLUMN (Summary, Rich Description, Subtasks, Activity) ════════ -->
        <div class="lg:col-span-8 xl:col-span-9 p-6 space-y-6 overflow-y-auto h-full">
          
          <!-- ① SUMMARY (TITLE) -->
          <div>
            <!-- Create Mode Summary -->
            <template v-if="isCreateMode">
              <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider mb-1.5">
                Ringkasan Tiket <span class="text-red-500">*</span>
              </label>
              <input
                v-model="createSummary"
                class="w-full text-xl font-bold text-slate-900 bg-white border border-slate-300 focus:border-blue-600 focus:ring-2 focus:ring-blue-100 rounded-xl px-4 py-3 transition focus:outline-none placeholder-slate-400 shadow-2xs"
                placeholder="e.g. Implementasikan retry webhook atau fitur export laporan..."
                autofocus
                @keyup.enter="handleCreateIssue"
              />
            </template>

            <!-- Detail Mode Summary -->
            <template v-else-if="issue">
              <label class="block text-xs font-bold text-slate-500 uppercase tracking-wider mb-1">
                Ringkasan Tiket
              </label>
              <input
                :value="issue.summary"
                @change="(e) => handleUpdateField({ summary: e.target.value })"
                class="w-full text-2xl font-bold text-slate-900 bg-transparent border-b border-transparent hover:border-slate-300 focus:border-blue-600 focus:bg-slate-50 rounded px-2 py-1.5 transition focus:outline-none"
                placeholder="Judul tiket..."
              />
            </template>
          </div>

          <!-- ② DESCRIPTION (RICH TEXT WYSIWYG EDITOR) -->
          <div class="space-y-2">
            <div class="flex items-center justify-between gap-3 select-none">
              <label class="text-xs font-bold text-slate-700 uppercase tracking-wider shrink-0">
                Deskripsi Lengkap
              </label>
              <span class="text-xs text-slate-500 hidden sm:inline-flex items-center gap-1.5 shrink-0">
                <Sparkles class="w-3.5 h-3.5 text-blue-600 shrink-0" />
                <span>Mendukung gaya Word, tabel spesifikasi, kode & lampiran gambar (Paste Ctrl+V)</span>
              </span>
            </div>

            <!-- Create Mode Rich Editor -->
            <RichTextEditor
              v-if="isCreateMode"
              v-model="createDescription"
              placeholder="Jelaskan kebutuhan tiket, kriteria penerimaan (acceptance criteria), tabel spesifikasi teknis, atau lampiran tangkapan layar..."
            />

            <!-- Detail Mode Rich Editor -->
            <RichTextEditor
              v-else-if="issue"
              :key="issue.id"
              :model-value="issue.description || ''"
              @update:model-value="(val) => handleUpdateField({ description: val })"
              @save="(val) => handleUpdateField({ description: val })"
            />
          </div>

          <!-- ③ SUBTASKS CHECKLIST (Detail Mode Only) -->
          <div
            v-if="isDetailMode && issue && issue.issue_type !== 'SUBTASK'"
            class="space-y-3.5 bg-slate-50 border border-slate-200 rounded-xl p-4"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center space-x-2">
                <CheckSquare class="w-4 h-4 text-emerald-600" />
                <h3 class="text-sm font-bold text-slate-800 uppercase tracking-wider">Checklist Subtask</h3>
              </div>
              <span v-if="subtaskStats" class="text-xs font-mono text-slate-600 font-semibold">
                {{ subtaskStats.done }} dari {{ subtaskStats.total }} selesai ({{ subtaskStats.percent }}%)
              </span>
            </div>

            <!-- Progress Bar -->
            <div v-if="subtaskStats" class="w-full bg-slate-200 h-2.5 rounded-full overflow-hidden">
              <div
                class="h-full transition-all duration-300 rounded-full"
                :class="subtaskStats.percent === 100 ? 'bg-emerald-500' : 'bg-blue-600'"
                :style="{ width: `${subtaskStats.percent}%` }"
              ></div>
            </div>

            <!-- Subtask Items List -->
            <div class="space-y-2 max-h-48 overflow-y-auto">
              <div
                v-for="sub in projectStore.activeIssueSubtasks"
                :key="sub.id"
                class="flex items-center justify-between p-2.5 rounded-lg bg-white border border-slate-200 hover:border-slate-300 transition shadow-2xs"
              >
                <div class="flex items-center space-x-3 truncate">
                  <input
                    type="checkbox"
                    :checked="sub.status_category === 'DONE'"
                    @change="toggleSubtaskDone(sub)"
                    class="rounded bg-white border-slate-300 text-blue-600 focus:ring-0 cursor-pointer w-4 h-4"
                  />
                  <span class="font-mono text-xs text-slate-500 font-semibold">{{ sub.key }}</span>
                  <span
                    class="text-sm text-slate-800 truncate"
                    :class="{ 'line-through text-slate-400': sub.status_category === 'DONE' }"
                  >
                    {{ sub.summary }}
                  </span>
                </div>

                <span
                  class="text-xs uppercase px-2 py-0.5 rounded font-mono font-semibold"
                  :class="sub.status_category === 'DONE' ? 'bg-emerald-50 text-emerald-700 border border-emerald-200' : 'bg-slate-100 text-slate-600'"
                >
                  {{ sub.status_name }}
                </span>
              </div>
            </div>

            <!-- Add Subtask Input -->
            <div class="flex items-center space-x-2 pt-1">
              <input
                v-model="newSubtaskSummary"
                placeholder="Tambah item subtask baru..."
                class="flex-1 bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-800 placeholder-slate-400 focus:outline-none focus:border-blue-600 shadow-2xs"
                @keyup.enter="handleAddSubtask"
              />
              <button
                @click="handleAddSubtask"
                class="px-4 py-2 bg-white hover:bg-slate-50 text-slate-700 border border-slate-300 rounded-lg text-sm font-semibold flex items-center space-x-1 shadow-2xs transition"
              >
                <Plus class="w-4 h-4" />
                <span>Tambah</span>
              </button>
            </div>
          </div>

          <!-- ④ ACTIVITY TABS: Comments & Work Log (Detail Mode Only) -->
          <div v-if="isDetailMode && issue" class="space-y-4 pt-2">
            <div class="flex items-center space-x-5 border-b border-slate-200 text-sm font-bold select-none">
              <button
                @click="activeTab = 'comments'"
                class="pb-2.5 flex items-center space-x-2 transition"
                :class="activeTab === 'comments' ? 'text-blue-600 border-b-2 border-blue-600' : 'text-slate-500 hover:text-slate-800'"
              >
                <MessageSquare class="w-4 h-4" />
                <span>Komentar ({{ projectStore.activeIssueComments.length }})</span>
              </button>

              <button
                @click="activeTab = 'timelog'"
                class="pb-2.5 flex items-center space-x-2 transition"
                :class="activeTab === 'timelog' ? 'text-blue-600 border-b-2 border-blue-600' : 'text-slate-500 hover:text-slate-800'"
              >
                <Clock class="w-4 h-4" />
                <span>Catatan Waktu (Work Log)</span>
              </button>
            </div>

            <!-- Tab 1: Comments -->
            <div v-if="activeTab === 'comments'" class="space-y-3.5">
              <!-- Add Comment Box -->
              <div class="flex items-start space-x-3">
                <img
                  :src="authStore.user?.avatar_url || 'https://api.dicebear.com/7.x/avataaars/svg?seed=user'"
                  class="w-8 h-8 rounded-full bg-slate-200 ring-1 ring-slate-300 shrink-0 mt-1"
                />
                <div class="flex-1 space-y-2">
                  <textarea
                    v-model="newCommentText"
                    rows="2"
                    placeholder="Tulis komentar... (Ketik @nama untuk me-mention anggota tim)"
                    class="w-full bg-white border border-slate-300 rounded-xl p-3 text-sm text-slate-800 placeholder-slate-400 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs"
                  ></textarea>
                  <div class="flex justify-between items-center">
                    <span class="text-xs text-slate-500">Tip: @mention mengirimkan notifikasi instan</span>
                    <button
                      @click="handleAddComment"
                      class="px-4 py-1.5 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-lg text-sm font-semibold flex items-center space-x-1.5 shadow-xs transition"
                    >
                      <Send class="w-3.5 h-3.5" />
                      <span>Kirim</span>
                    </button>
                  </div>
                </div>
              </div>

              <!-- Comments List -->
              <div class="space-y-2.5 max-h-56 overflow-y-auto pt-2">
                <div
                  v-for="comment in projectStore.activeIssueComments"
                  :key="comment.id"
                  class="p-3.5 bg-slate-50 border border-slate-200 rounded-xl flex items-start space-x-3"
                >
                  <img
                    :src="comment.user_avatar || 'https://api.dicebear.com/7.x/avataaars/svg?seed=' + comment.id"
                    class="w-7 h-7 rounded-full bg-slate-200 shrink-0 mt-0.5"
                  />
                  <div class="flex-1">
                    <div class="flex items-center justify-between mb-1">
                      <span class="text-sm font-bold text-slate-800">{{ comment.user_name || 'Anggota' }}</span>
                      <span class="text-xs text-slate-500">{{ new Date(comment.created_at).toLocaleString() }}</span>
                    </div>
                    <p class="text-sm text-slate-700 whitespace-pre-wrap leading-relaxed">{{ comment.body }}</p>
                  </div>
                </div>
              </div>
            </div>

            <!-- Tab 2: Work Log -->
            <div v-if="activeTab === 'timelog'" class="space-y-3.5">
              <div class="flex items-center justify-between p-3.5 bg-slate-50 rounded-xl border border-slate-200">
                <div>
                  <span class="text-sm text-slate-600">Total Waktu Dikerjakan: </span>
                  <span class="text-base font-bold font-mono text-emerald-600">
                    {{ (projectStore.totalTimeSpentSeconds / 3600).toFixed(1) }} jam
                  </span>
                </div>
                <button
                  @click="isLoggingWork = !isLoggingWork"
                  class="px-3 py-1.5 bg-white hover:bg-emerald-50 border border-emerald-500 text-emerald-700 rounded-lg text-sm font-semibold shadow-2xs transition"
                >
                  + Catat Waktu
                </button>
              </div>

              <!-- Log Work Form -->
              <div v-if="isLoggingWork" class="p-3.5 bg-white border border-emerald-400 rounded-xl space-y-3 shadow-xs">
                <div class="flex items-center space-x-3">
                  <div class="w-36">
                    <label class="block text-xs font-semibold text-slate-600 mb-1">Durasi (Jam)</label>
                    <input
                      v-model.number="logHours"
                      type="number"
                      step="0.25"
                      min="0.1"
                      class="w-full bg-slate-50 border border-slate-300 rounded-lg px-2.5 py-1.5 text-sm text-slate-900 shadow-2xs font-semibold"
                    />
                  </div>
                  <div class="flex-1">
                    <label class="block text-xs font-semibold text-slate-600 mb-1">Deskripsi Pengerjaan</label>
                    <input
                      v-model="logDescription"
                      placeholder="Pekerjaan apa yang sudah diselesaikan?..."
                      class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-1.5 text-sm text-slate-900 shadow-2xs"
                    />
                  </div>
                </div>
                <div class="flex justify-end space-x-2">
                  <button @click="isLoggingWork = false" class="px-3 py-1.5 text-sm font-medium text-slate-600 hover:text-slate-900">Batal</button>
                  <button @click="handleLogWork" class="px-4 py-1.5 bg-emerald-600 hover:bg-emerald-700 active:bg-emerald-800 text-white rounded-lg text-sm font-semibold shadow-xs">Simpan Catatan</button>
                </div>
              </div>

              <!-- Logs History -->
              <div class="space-y-2 max-h-48 overflow-y-auto">
                <div
                  v-for="tl in projectStore.activeIssueTimeLogs"
                  :key="tl.id"
                  class="p-3 bg-white border border-slate-200 rounded-lg flex items-center justify-between text-sm shadow-2xs"
                >
                  <div>
                    <span class="font-bold text-slate-800">{{ tl.user_name || 'Anggota' }}: </span>
                    <span class="text-slate-600">{{ tl.description || 'Mengerjakan tiket' }}</span>
                  </div>
                  <span class="font-mono font-bold text-emerald-600">
                    {{ (tl.time_spent_seconds / 3600).toFixed(1) }}h
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- ════════ RIGHT COLUMN (Sidebar Attributes) ════════ -->
        <div class="lg:col-span-4 xl:col-span-3 p-6 space-y-5 bg-slate-50/70 text-sm overflow-y-auto h-full">
          <div class="font-bold text-xs text-slate-500 uppercase tracking-wider pb-2 border-b border-slate-200 flex items-center gap-1.5">
            <Layers class="w-3.5 h-3.5 text-slate-600" />
            <span>Atribut Tiket</span>
          </div>

          <!-- Tipe Tiket -->
          <div class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider">Tipe Tiket</label>
            
            <!-- Create Mode Select -->
            <select
              v-if="isCreateMode"
              v-model="createType"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs font-semibold"
            >
              <option value="TASK">📋 Task (Pekerjaan Standar)</option>
              <option value="STORY">📖 Story (Fitur / User Story)</option>
              <option value="EPIC">🟣 Epic (Milestone Besar)</option>
              <option value="BUG">🐞 Bug (Kendala / Masalah)</option>
            </select>

            <!-- Detail Mode Select -->
            <select
              v-else-if="issue"
              :value="issue.issue_type"
              @change="(e) => handleUpdateField({ issue_type: e.target.value })"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs font-semibold cursor-pointer"
            >
              <option value="TASK">📋 Task (Pekerjaan Standar)</option>
              <option value="STORY">📖 Story (Fitur / User Story)</option>
              <option value="EPIC">🟣 Epic (Milestone Besar)</option>
              <option value="BUG">🐞 Bug (Kendala / Masalah)</option>
              <option v-if="issue.issue_type === 'SUBTASK'" value="SUBTASK">🔹 Subtask</option>
            </select>
          </div>

          <!-- Status Alur Kerja -->
          <div class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider">Status Alur Kerja</label>
            
            <!-- Create Mode Status -->
            <select
              v-if="isCreateMode"
              v-model="createStatusId"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
            >
              <option v-for="s in projectStore.statuses" :key="s.id" :value="s.id">
                {{ s.name }} ({{ s.category }})
              </option>
            </select>

            <!-- Detail Mode Status -->
            <select
              v-else-if="issue"
              :value="issue.status_id"
              @change="handleStatusChange"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm font-semibold text-slate-800 focus:outline-none focus:border-blue-600 cursor-pointer shadow-2xs"
            >
              <option v-for="s in projectStore.statuses" :key="s.id" :value="s.id">
                {{ s.name }} ({{ s.category }})
              </option>
            </select>
          </div>

          <!-- Prioritas -->
          <div class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1">
              <Flag class="w-3 h-3 text-slate-500" />
              <span>Prioritas</span>
            </label>

            <!-- Create Mode Priority -->
            <select
              v-if="isCreateMode"
              v-model="createPriority"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs font-semibold"
            >
              <option value="HIGHEST">🔴 Tertinggi (Highest)</option>
              <option value="HIGH">🟠 Tinggi (High)</option>
              <option value="MEDIUM">🟡 Sedang (Medium)</option>
              <option value="LOW">🔵 Rendah (Low)</option>
            </select>

            <!-- Detail Mode Priority -->
            <select
              v-else-if="issue"
              :value="issue.priority"
              @change="(e) => handleUpdateField({ priority: e.target.value })"
              class="w-full border border-slate-300 rounded-lg px-3 py-2 text-sm font-semibold focus:outline-none cursor-pointer shadow-2xs"
              :class="`badge-priority-${issue.priority}`"
            >
              <option value="HIGHEST">🔴 Tertinggi (Highest)</option>
              <option value="HIGH">🟠 Tinggi (High)</option>
              <option value="MEDIUM">🟡 Sedang (Medium)</option>
              <option value="LOW">🔵 Rendah (Low)</option>
            </select>
          </div>

          <!-- Ditugaskan Kepada (Assignee) -->
          <div class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1">
              <User class="w-3 h-3 text-slate-500" />
              <span>Ditugaskan Kepada</span>
            </label>

            <!-- Create Mode Assignee -->
            <select
              v-if="isCreateMode"
              v-model="createAssigneeId"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-medium"
            >
              <option :value="null">Belum Ditugaskan</option>
              <option v-for="u in authStore.allUsers" :key="u.id" :value="u.id">
                {{ u.full_name }} ({{ u.email }})
              </option>
            </select>

            <!-- Detail Mode Assignee -->
            <select
              v-else-if="issue"
              :value="issue.assignee_id || ''"
              @change="(e) => handleUpdateField({ assignee_id: e.target.value ? Number(e.target.value) : null })"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-800 focus:outline-none cursor-pointer shadow-2xs font-medium"
            >
              <option value="">Belum Ditugaskan</option>
              <option v-for="u in authStore.allUsers" :key="u.id" :value="u.id">
                {{ u.full_name }}
              </option>
            </select>
          </div>

          <!-- Parent Epic (if not EPIC) -->
          <div v-if="(isCreateMode && createType !== 'EPIC') || (isDetailMode && issue && issue.issue_type !== 'EPIC')" class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider">Parent Epic</label>

            <!-- Create Mode Epic -->
            <select
              v-if="isCreateMode"
              v-model="createEpicId"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-medium"
            >
              <option :value="null">Tanpa Epic</option>
              <option v-for="e in projectStore.epics" :key="e.id" :value="e.id">
                🟣 {{ e.summary }}
              </option>
            </select>

            <!-- Detail Mode Epic -->
            <select
              v-else-if="issue"
              :value="issue.epic_id || ''"
              @change="(e) => handleUpdateField({ epic_id: e.target.value ? Number(e.target.value) : null })"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-medium cursor-pointer"
            >
              <option value="">Tanpa Epic</option>
              <option v-for="e in projectStore.epics" :key="e.id" :value="e.id">
                🟣 {{ e.summary }}
              </option>
            </select>
          </div>

          <!-- Story Points (Estimation) -->
          <div class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1">
              <Hash class="w-3 h-3 text-slate-500" />
              <span>Story Points (Estimasi)</span>
            </label>

            <!-- Create Mode Story Points -->
            <input
              v-if="isCreateMode"
              v-model.number="createStoryPoints"
              type="number"
              min="0"
              placeholder="0"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs font-semibold"
            />

            <!-- Detail Mode Story Points Buttons -->
            <div v-else-if="issue" class="flex items-center space-x-1.5 flex-wrap gap-y-1">
              <button
                v-for="p in [0, 1, 2, 3, 5, 8, 13]"
                :key="p"
                @click="handleUpdateField({ story_points: p })"
                class="w-8 h-8 rounded-lg text-sm font-mono font-bold transition flex items-center justify-center border shadow-2xs"
                :class="issue.story_points === p ? 'bg-blue-600 text-white border-blue-600 ring-2 ring-blue-200' : 'bg-white border-slate-300 text-slate-700 hover:text-slate-900 hover:bg-slate-50'"
              >
                {{ p }}
              </button>
            </div>
          </div>

          <!-- Dates: Start Date & Due Date -->
          <div class="space-y-3 pt-2 border-t border-slate-200">
            <!-- Start Date -->
            <div class="space-y-1">
              <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1">
                <Calendar class="w-3 h-3 text-slate-500" />
                <span>Tanggal Mulai</span>
              </label>

              <input
                v-if="isCreateMode"
                v-model="createStartDate"
                type="date"
                class="w-full bg-white border border-slate-300 rounded-lg px-3 py-1.5 text-sm text-slate-800 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
              />
              <input
                v-else-if="issue"
                type="date"
                :value="issue.start_date || ''"
                @change="(e) => handleUpdateField({ start_date: e.target.value || null })"
                class="w-full bg-white border border-slate-300 rounded-lg px-3 py-1.5 text-sm text-slate-800 focus:outline-none shadow-2xs font-medium"
              />
            </div>

            <!-- Due Date -->
            <div class="space-y-1">
              <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1">
                <Calendar class="w-3 h-3 text-slate-500" />
                <span>Tenggat Waktu</span>
              </label>

              <input
                v-if="isCreateMode"
                v-model="createDueDate"
                type="date"
                class="w-full bg-white border border-slate-300 rounded-lg px-3 py-1.5 text-sm text-slate-800 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
              />
              <input
                v-else-if="issue"
                type="date"
                :value="issue.due_date || ''"
                @change="(e) => handleUpdateField({ due_date: e.target.value || null })"
                class="w-full bg-white border border-slate-300 rounded-lg px-3 py-1.5 text-sm text-slate-800 focus:outline-none shadow-2xs font-medium"
              />
            </div>
          </div>

          <!-- Reporter & Meta (Detail Mode Only) -->
          <div v-if="isDetailMode && issue" class="pt-3 border-t border-slate-200 text-xs text-slate-500 space-y-2">
            <div class="flex justify-between">
              <span>Pelapor:</span>
              <span class="font-medium text-slate-800">{{ issue.reporter_name || 'System' }}</span>
            </div>
            <div class="flex justify-between">
              <span>Dibuat:</span>
              <span>{{ new Date(issue.created_at).toLocaleDateString('id-ID', { day: 'numeric', month: 'short', year: 'numeric' }) }}</span>
            </div>
            <div class="flex justify-between">
              <span>Diperbarui:</span>
              <span>{{ new Date(issue.updated_at).toLocaleDateString('id-ID', { day: 'numeric', month: 'short', year: 'numeric' }) }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- ── MODAL FOOTER BAR ── -->
      <div class="px-6 py-4 border-t border-slate-200 bg-slate-50 flex items-center justify-between select-none">
        <!-- Footer Left Hint -->
        <span class="text-xs text-slate-400 hidden sm:inline">
          <template v-if="isCreateMode">
            Tekan <kbd class="px-1.5 py-0.5 bg-slate-200 border border-slate-300 rounded text-[10px] font-mono text-slate-700">Enter</kbd> di judul untuk membuat cepat, atau <kbd class="px-1.5 py-0.5 bg-slate-200 border border-slate-300 rounded text-[10px] font-mono text-slate-700">Esc</kbd> untuk membatalkan.
          </template>
          <template v-else>
            <span class="flex items-center gap-1 text-emerald-600 font-medium">
              <CheckCircle2 class="w-3.5 h-3.5" />
              Semua perubahan tersimpan secara otomatis
            </span>
          </template>
        </span>

        <!-- Footer Right Actions -->
        <div class="flex items-center space-x-3 ml-auto">
          <!-- CREATE MODE BUTTONS -->
          <template v-if="isCreateMode">
            <button
              type="button"
              @click="handleClose"
              class="px-4 py-2 text-sm font-semibold text-slate-600 hover:text-slate-900 hover:bg-slate-200/60 rounded-xl transition"
              :disabled="isSubmitting"
            >
              Batal
            </button>
            <button
              type="button"
              @click="handleCreateIssue"
              class="px-6 py-2 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-xl text-sm font-bold shadow-sm hover:shadow transition flex items-center gap-2 active:scale-98 disabled:opacity-50 disabled:pointer-events-none"
              :disabled="isSubmitting"
            >
              <Loader2 v-if="isSubmitting" class="w-4 h-4 animate-spin" />
              <Plus v-else class="w-4 h-4" />
              <span>{{ isSubmitting ? 'Menyimpan...' : 'Buat Tiket' }}</span>
            </button>
          </template>

          <!-- DETAIL MODE BUTTON -->
          <template v-else>
            <button
              type="button"
              @click="handleClose"
              class="px-5 py-2 bg-slate-800 hover:bg-slate-900 active:bg-black text-white rounded-xl text-sm font-semibold shadow-xs transition"
            >
              Tutup
            </button>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>
