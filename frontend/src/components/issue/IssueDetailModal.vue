<script setup>
import { ref, computed } from 'vue';
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
  Sparkles
} from 'lucide-vue-next';
import RichTextEditor from '@/components/common/RichTextEditor.vue';
import { toast } from '@/utils/toast';

const projectStore = useProjectStore();
const authStore = useAuthStore();

const activeTab = ref('comments'); // 'comments' | 'timelog'
const newCommentText = ref('');
const newSubtaskSummary = ref('');

// Log work modal / inputs
const isLoggingWork = ref(false);
const logHours = ref(1);
const logDescription = ref('');

const issue = computed(() => projectStore.activeIssue);

// Subtasks calculation
const subtaskStats = computed(() => {
  const list = projectStore.activeIssueSubtasks || [];
  if (list.length === 0) return null;
  const done = list.filter((s) => s.status_category === 'DONE').length;
  const percent = Math.round((done / list.length) * 100);
  return { total: list.length, done, percent };
});

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
    // Refresh detail to get updated subtasks list
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
    v-if="projectStore.isDetailModalOpen && issue"
    class="fixed inset-0 z-50 flex items-center justify-center p-3 sm:p-5 bg-slate-900/50 backdrop-blur-sm"
  >
    <div class="glass-modal w-[96vw] max-w-[1440px] rounded-2xl border border-slate-200 bg-white h-[94vh] max-h-[96vh] flex flex-col shadow-2xl animate-slide-up overflow-hidden">
      <!-- Modal Top Header -->
      <div class="px-6 py-4 border-b border-slate-200 bg-slate-50 flex items-center justify-between select-none">
        <div class="flex items-center space-x-3">
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
          <span v-if="issue.epic_summary && issue.issue_type !== 'EPIC'" class="text-xs px-2.5 py-0.5 rounded-full bg-purple-50 text-purple-700 border border-purple-200 font-semibold">
            🟣 {{ issue.epic_summary }}
          </span>

          <!-- Parent link if present -->
          <span v-if="issue.parent_key" class="text-sm text-slate-500">
            Subtask dari <strong class="text-slate-800">{{ issue.parent_key }}</strong>
          </span>
        </div>

        <div class="flex items-center space-x-2">
          <button
            @click="handleDelete"
            class="p-2 rounded-lg text-slate-400 hover:text-red-600 hover:bg-red-50 transition"
            title="Hapus tiket ini"
          >
            <Trash2 class="w-4 h-4" />
          </button>

          <button
            @click="projectStore.closeDetailModal"
            class="p-2 rounded-lg text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition"
          >
            <X class="w-5 h-5" />
          </button>
        </div>
      </div>

      <!-- Main Content Grid: 2 Columns (Main left 75%, Metadata right 25%) -->
      <div class="flex-1 overflow-hidden grid grid-cols-1 lg:grid-cols-12 divide-y lg:divide-y-0 lg:divide-x divide-slate-200">
        <!-- LEFT COLUMNS: Summary, Description, Subtasks, Activity -->
        <div class="lg:col-span-8 xl:col-span-9 p-6 space-y-6 overflow-y-auto h-full">
          <!-- Summary (Title) Input -->
          <div>
            <input
              :value="issue.summary"
              @change="(e) => handleUpdateField({ summary: e.target.value })"
              class="w-full text-2xl font-bold text-slate-900 bg-transparent border-b border-transparent hover:border-slate-300 focus:border-blue-600 focus:bg-slate-50 rounded px-2 py-1.5 transition focus:outline-none"
              placeholder="Judul tiket..."
            />
          </div>

          <!-- Description Section (Rich Text Editor) -->
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider">
                Deskripsi Tiket
              </label>
              <span class="text-xs text-slate-400 hidden sm:inline">
                Mendukung gaya Word, tabel spesifikasi, kode & lampiran gambar (Paste Ctrl+V)
              </span>
            </div>

            <!-- Rich WYSIWYG Editor -->
            <RichTextEditor
              :key="issue.id"
              :model-value="issue.description || ''"
              @update:model-value="(val) => handleUpdateField({ description: val })"
              @save="(val) => handleUpdateField({ description: val })"
            />
          </div>

          <!-- Subtasks Checklist Section (FR-4.4) -->
          <div v-if="issue.issue_type !== 'SUBTASK'" class="space-y-3.5 bg-slate-50 border border-slate-200 rounded-xl p-4">
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

          <!-- Activity Tabs: Comments & Work Log -->
          <div class="space-y-4 pt-2">
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
              <!-- Add Comment Input Box -->
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

              <!-- Log Work Inline Form -->
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

        <!-- RIGHT COLUMN: Status, Priority, Assignee, Dates, Points -->
        <div class="lg:col-span-4 xl:col-span-3 p-6 space-y-5 bg-slate-50/70 text-sm overflow-y-auto h-full">
          <!-- Status Dropdown (with Workflow Transition rules) -->
          <div>
            <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1.5">
              Status Alur Kerja
            </label>
            <select
              :value="issue.status_id"
              @change="handleStatusChange"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm font-semibold text-slate-800 focus:outline-none focus:border-blue-600 cursor-pointer shadow-2xs"
            >
              <option
                v-for="s in projectStore.statuses"
                :key="s.id"
                :value="s.id"
              >
                {{ s.name }} ({{ s.category }})
              </option>
            </select>
          </div>

          <!-- Priority -->
          <div>
            <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1.5">
              Prioritas
            </label>
            <select
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

          <!-- Assignee -->
          <div>
            <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1.5">
              Ditugaskan Kepada
            </label>
            <select
              :value="issue.assignee_id || ''"
              @change="(e) => handleUpdateField({ assignee_id: e.target.value ? Number(e.target.value) : null })"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-800 focus:outline-none cursor-pointer shadow-2xs font-medium"
            >
              <option value="">Belum Ditugaskan</option>
              <option
                v-for="u in authStore.allUsers"
                :key="u.id"
                :value="u.id"
              >
                {{ u.full_name }}
              </option>
            </select>
          </div>

          <!-- Story Points (Estimation) -->
          <div>
            <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1.5">
              Story Points (Estimasi)
            </label>
            <div class="flex items-center space-x-1.5">
              <button
                v-for="p in [1, 2, 3, 5, 8, 13]"
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
          <div class="space-y-3 pt-1 border-t border-slate-200">
            <div>
              <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1">
                Tanggal Mulai
              </label>
              <input
                type="date"
                :value="issue.start_date || ''"
                @change="(e) => handleUpdateField({ start_date: e.target.value || null })"
                class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-800 focus:outline-none shadow-2xs font-medium"
              />
            </div>

            <div>
              <label class="block text-slate-600 font-bold uppercase tracking-wider text-xs mb-1">
                Tenggat Waktu
              </label>
              <input
                type="date"
                :value="issue.due_date || ''"
                @change="(e) => handleUpdateField({ due_date: e.target.value || null })"
                class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-800 focus:outline-none shadow-2xs font-medium"
              />
            </div>
          </div>

          <!-- Reporter & Meta -->
          <div class="pt-3 border-t border-slate-200 text-xs text-slate-500 space-y-1">
            <p>Pembuat: <strong class="text-slate-800">{{ issue.reporter_name || 'System' }}</strong></p>
            <p>Dibuat: <span class="text-slate-700">{{ new Date(issue.created_at).toLocaleDateString() }}</span></p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
