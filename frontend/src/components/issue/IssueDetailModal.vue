<script setup>
import { ref, reactive, computed, watch } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useAuthStore } from '@/stores/auth';
import {
  X,
  Clock,
  MessageSquare,
  Trash2,
  Plus,
  Sparkles,
  AlertCircle,
  Loader2,
  CheckCircle2
} from 'lucide-vue-next';
import RichTextEditor from '@/components/common/RichTextEditor.vue';
import IssueChecklist from './IssueChecklist.vue';
import IssueComments from './IssueComments.vue';
import IssueTimeLog from './IssueTimeLog.vue';
import IssueMetaSidebar from './IssueMetaSidebar.vue';
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
const createErrorMessage = ref('');
const isSubmitting = ref(false);

const createForm = reactive({
  type: 'TASK',
  statusId: null,
  priority: 'MEDIUM',
  assigneeId: null,
  epicId: null,
  sprintId: null,
  componentId: null,
  fixVersionId: null,
  labels: [],
  storyPoints: 0,
  startDate: '',
  dueDate: ''
});

// --- Detail / Activity State ---
const activeTab = ref('comments'); // 'comments' | 'timelog'

// Watch for create modal opening to reset form
watch(
  () => projectStore.isCreateModalOpen,
  (open) => {
    if (open) {
      createSummary.value = '';
      createDescription.value = '';
      createForm.type = 'TASK';
      createForm.statusId = projectStore.defaultCreateStatusId || (projectStore.statuses[0]?.id || null);
      createForm.priority = 'MEDIUM';
      createForm.assigneeId = null;
      createForm.epicId = null;
      createForm.sprintId = null;
      createForm.componentId = null;
      createForm.fixVersionId = null;
      createForm.labels = [];
      createForm.storyPoints = 0;
      createForm.startDate = '';
      createForm.dueDate = '';
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
      issue_type: createForm.type,
      status_id: createForm.statusId,
      priority: createForm.priority,
      assignee_id: createForm.assigneeId ? Number(createForm.assigneeId) : null,
      epic_id: createForm.epicId ? Number(createForm.epicId) : null,
      sprint_id: createForm.sprintId ? Number(createForm.sprintId) : null,
      component_id: createForm.componentId ? Number(createForm.componentId) : null,
      fix_version_id: createForm.fixVersionId ? Number(createForm.fixVersionId) : null,
      labels: createForm.labels && createForm.labels.length > 0 ? createForm.labels : null,
      story_points: Number(createForm.storyPoints) || 0,
      start_date: createForm.startDate || null,
      due_date: createForm.dueDate || null,
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

async function handleAddComment(text) {
  if (!text || !issue.value) return;
  try {
    await projectStore.addComment(issue.value.id, text);
    toast.success('Komentar berhasil ditambahkan');
  } catch (e) {
    toast.error(e.message || 'Gagal menambahkan komentar');
  }
}

async function handleAddSubtask(summary) {
  if (!summary || !issue.value) return;
  try {
    await projectStore.createIssue({
      summary,
      issue_type: 'SUBTASK',
      parent_id: issue.value.id,
      epic_id: issue.value.epic_id || (issue.value.issue_type === 'EPIC' ? issue.value.id : null),
      status_id: issue.value.status_id,
      priority: 'MEDIUM',
    });
    toast.success('Subtask berhasil ditambahkan');
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

async function handleLogWork({ hours, description }) {
  if (!issue.value) return;
  try {
    const seconds = Math.round(hours * 3600);
    await projectStore.logTime(issue.value.id, seconds, description);
    toast.success(`Berhasil mencatat ${hours} jam kerja`);
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
            <span
              class="text-xs font-semibold px-2.5 py-0.5 rounded uppercase tracking-wider"
              :class="`badge-type-${issue.issue_type}`"
            >
              {{ issue.issue_type }}
            </span>

            <span class="font-mono text-base font-bold text-blue-600">
              {{ issue.key }}
            </span>

            <span
              v-if="issue.epic_summary && issue.issue_type !== 'EPIC'"
              class="text-xs px-2.5 py-0.5 rounded-full bg-purple-50 text-purple-700 border border-purple-200 font-semibold"
            >
              🟣 {{ issue.epic_summary }}
            </span>

            <span v-if="issue.parent_key" class="text-sm text-slate-500">
              Subtask dari <strong class="text-slate-800">{{ issue.parent_key }}</strong>
            </span>
          </template>
        </div>

        <!-- Header Right: Actions & Close Button -->
        <div class="flex items-center space-x-2">
          <button
            v-if="isDetailMode && issue"
            @click="handleDelete"
            class="p-2 rounded-lg text-slate-400 hover:text-red-600 hover:bg-red-50 transition"
            title="Hapus tiket ini"
          >
            <Trash2 class="w-4 h-4" />
          </button>

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

            <RichTextEditor
              v-if="isCreateMode"
              v-model="createDescription"
              placeholder="Jelaskan kebutuhan tiket, kriteria penerimaan (acceptance criteria), tabel spesifikasi teknis, atau lampiran tangkapan layar..."
            />

            <RichTextEditor
              v-else-if="issue"
              :key="issue.id"
              :model-value="issue.description || ''"
              @update:model-value="(val) => handleUpdateField({ description: val })"
              @save="(val) => handleUpdateField({ description: val })"
            />
          </div>

          <!-- ③ SUBTASKS CHECKLIST (Detail Mode Only) -->
          <IssueChecklist
            v-if="isDetailMode && issue && issue.issue_type !== 'SUBTASK'"
            :subtasks="projectStore.activeIssueSubtasks"
            :subtask-stats="subtaskStats"
            @toggle-done="toggleSubtaskDone"
            @add-subtask="handleAddSubtask"
          />

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
            <IssueComments
              v-if="activeTab === 'comments'"
              :comments="projectStore.activeIssueComments"
              :current-user-avatar="authStore.user?.avatar_url || 'https://api.dicebear.com/7.x/avataaars/svg?seed=user'"
              @add-comment="handleAddComment"
            />

            <!-- Tab 2: Work Log -->
            <IssueTimeLog
              v-if="activeTab === 'timelog'"
              :time-logs="projectStore.activeIssueTimeLogs"
              :total-time-spent-seconds="projectStore.totalTimeSpentSeconds"
              @log-work="handleLogWork"
            />
          </div>
        </div>

        <!-- ════════ RIGHT COLUMN (Sidebar Attributes) ════════ -->
        <div class="lg:col-span-4 xl:col-span-3 h-full">
          <IssueMetaSidebar
            :is-create-mode="isCreateMode"
            :is-detail-mode="isDetailMode"
            :issue="issue"
            :create-form="createForm"
            :statuses="projectStore.statuses"
            :users="authStore.allUsers"
            :epics="projectStore.epics"
            @update-field="handleUpdateField"
            @status-change="handleStatusChange"
          />
        </div>
      </div>

      <!-- ── MODAL FOOTER BAR ── -->
      <div class="px-6 py-4 border-t border-slate-200 bg-slate-50 flex items-center justify-between select-none">
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

        <div class="flex items-center space-x-3 ml-auto">
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
