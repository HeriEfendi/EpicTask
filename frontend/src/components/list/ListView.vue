<script setup>
import { ref } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useAuthStore } from '@/stores/auth';
import {
  ListTodo,
  Plus,
  Trash2,
  ExternalLink
} from 'lucide-vue-next';

const projectStore = useProjectStore();
const authStore = useAuthStore();

// Inline quick create row state
const quickSummary = ref('');
const quickType = ref('TASK');
const quickPriority = ref('MEDIUM');

async function handleQuickCreate() {
  if (!quickSummary.value.trim()) return;
  try {
    await projectStore.createIssue({
      summary: quickSummary.value.trim(),
      issue_type: quickType.value,
      priority: quickPriority.value,
    });
    quickSummary.value = '';
  } catch (e) {
    alert(e.message || 'Gagal membuat tiket');
  }
}

async function handleStatusChange(issue, newStatusId) {
  try {
    await projectStore.moveIssue(issue.id, Number(newStatusId));
  } catch (e) {
    alert(e.message || 'Transisi status ditolak oleh aturan workflow');
  }
}

async function handlePriorityChange(issue, newPriority) {
  try {
    await projectStore.updateIssue(issue.id, { priority: newPriority });
  } catch (e) {
    alert(e.message || 'Gagal mengubah prioritas');
  }
}

async function handleAssigneeChange(issue, newAssigneeId) {
  try {
    await projectStore.updateIssue(issue.id, {
      assignee_id: newAssigneeId ? Number(newAssigneeId) : null,
    });
  } catch (e) {
    alert(e.message || 'Gagal mengubah penugasan');
  }
}

async function handleDueDateChange(issue, newDate) {
  try {
    await projectStore.updateIssue(issue.id, { due_date: newDate || null });
  } catch (e) {
    alert(e.message || 'Gagal mengubah tenggat waktu');
  }
}

async function handlePointsChange(issue, newPoints) {
  try {
    await projectStore.updateIssue(issue.id, { story_points: Number(newPoints) || 0 });
  } catch (e) {
    alert(e.message || 'Gagal mengubah story points');
  }
}

async function handleDelete(issueId) {
  if (confirm('Yakin ingin menghapus tiket ini?')) {
    await projectStore.deleteIssue(issueId);
  }
}
</script>

<template>
  <div class="flex-1 p-6 flex flex-col overflow-hidden bg-slate-50">
    <div class="flex items-center justify-between pb-4 border-b border-slate-200">
      <h2 class="text-base font-bold text-slate-800 flex items-center gap-2">
        <ListTodo class="w-5 h-5 text-blue-600" />
        <span>Spreadsheet List View (Edit Langsung di Baris)</span>
      </h2>
      <span class="text-sm text-slate-600 font-medium">
        {{ projectStore.filteredIssues.length }} item tiket
      </span>
    </div>

    <!-- Table Container -->
    <div class="flex-1 overflow-auto mt-4 border border-slate-200 rounded-xl bg-white shadow-xs">
      <table class="w-full text-left text-sm divide-y divide-slate-200 select-none">
        <!-- Table Header (Enlarged to text-sm font-bold) -->
        <thead class="bg-slate-100 text-slate-700 font-bold sticky top-0 z-10 border-b border-slate-200">
          <tr>
            <th class="py-3 px-3.5 w-28">Key</th>
            <th class="py-3 px-3.5 w-28">Tipe</th>
            <th class="py-3 px-3.5 min-w-[300px]">Ringkasan Tiket</th>
            <th class="py-3 px-3.5 w-40">Status</th>
            <th class="py-3 px-3.5 w-36">Prioritas</th>
            <th class="py-3 px-3.5 w-44">Ditugaskan Ke</th>
            <th class="py-3 px-3.5 w-24 text-center">Poin</th>
            <th class="py-3 px-3.5 w-40">Tenggat</th>
            <th class="py-3 px-3.5 w-24 text-center">Aksi</th>
          </tr>
        </thead>

        <tbody class="divide-y divide-slate-100 text-slate-800">
          <!-- Quick Inline Add Row -->
          <tr class="bg-blue-50/50 hover:bg-blue-50 transition border-b border-blue-100">
            <td class="py-3 px-3.5 font-mono text-xs text-blue-600 font-bold">+ Baru</td>
            <td class="py-3 px-3.5">
              <select
                v-model="quickType"
                class="bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-800 focus:outline-none focus:border-blue-600 shadow-2xs"
              >
                <option value="TASK">TASK</option>
                <option value="STORY">STORY</option>
                <option value="BUG">BUG</option>
                <option value="EPIC">EPIC</option>
              </select>
            </td>
            <td class="py-3 px-3.5">
              <input
                v-model="quickSummary"
                placeholder="Apa yang perlu dikerjakan? Tekan Enter..."
                class="w-full bg-white border border-slate-300 rounded-lg px-3 py-1.5 text-sm text-slate-900 placeholder-slate-400 focus:outline-none focus:border-blue-600 shadow-2xs"
                @keyup.enter="handleQuickCreate"
              />
            </td>
            <td class="py-3 px-3.5 text-slate-400 italic text-xs">To Do</td>
            <td class="py-3 px-3.5">
              <select
                v-model="quickPriority"
                class="bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-800 focus:outline-none shadow-2xs"
              >
                <option value="HIGHEST">HIGHEST</option>
                <option value="HIGH">HIGH</option>
                <option value="MEDIUM">MEDIUM</option>
                <option value="LOW">LOW</option>
              </select>
            </td>
            <td class="py-3 px-3.5 text-slate-400 italic text-xs">Belum ditugaskan</td>
            <td class="py-3 px-3.5 text-slate-400 text-center text-xs">-</td>
            <td class="py-3 px-3.5 text-slate-400 italic text-xs">Tidak ada</td>
            <td class="py-3 px-3.5 text-center">
              <button
                @click="handleQuickCreate"
                class="px-3 py-1.5 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-lg font-semibold text-xs shadow-xs transition"
              >
                Tambah
              </button>
            </td>
          </tr>

          <!-- Issues Rows -->
          <tr
            v-for="issue in projectStore.filteredIssues"
            :key="issue.id"
            class="hover:bg-slate-50 transition group text-sm"
          >
            <!-- Key -->
            <td
              @click="projectStore.openIssueDetail(issue.id)"
              class="py-3 px-3.5 font-mono font-semibold text-blue-600 cursor-pointer hover:underline text-xs"
            >
              {{ issue.key }}
            </td>

            <!-- Type -->
            <td class="py-3 px-3.5">
              <span
                class="text-xs font-semibold px-2 py-0.5 rounded uppercase"
                :class="`badge-type-${issue.issue_type}`"
              >
                {{ issue.issue_type }}
              </span>
            </td>

            <!-- Summary -->
            <td
              @click="projectStore.openIssueDetail(issue.id)"
              class="py-3 px-3.5 font-medium text-slate-900 cursor-pointer group-hover:text-blue-600"
            >
              <div class="flex items-center space-x-2">
                <span class="truncate max-w-lg">{{ issue.summary }}</span>
                <span
                  v-if="issue.epic_summary && issue.issue_type !== 'EPIC'"
                  class="text-xs px-2 py-0.5 rounded-full bg-purple-50 text-purple-700 border border-purple-200 shrink-0 font-medium"
                >
                  {{ issue.epic_summary }}
                </span>
              </div>
            </td>

            <!-- Status Dropdown (Inline Edit) -->
            <td class="py-3 px-3.5">
              <select
                :value="issue.status_id"
                @change="(e) => handleStatusChange(issue, e.target.value)"
                class="bg-white border border-slate-300 rounded-lg px-2.5 py-1 text-xs text-slate-800 focus:outline-none focus:border-blue-600 cursor-pointer hover:bg-slate-50 shadow-2xs font-medium"
              >
                <option
                  v-for="status in projectStore.statuses"
                  :key="status.id"
                  :value="status.id"
                >
                  {{ status.name }}
                </option>
              </select>
            </td>

            <!-- Priority Dropdown (Inline Edit) -->
            <td class="py-3 px-3.5">
              <select
                :value="issue.priority"
                @change="(e) => handlePriorityChange(issue, e.target.value)"
                class="border border-slate-200 rounded-lg px-2 py-1 text-xs font-semibold focus:outline-none cursor-pointer shadow-2xs"
                :class="`badge-priority-${issue.priority}`"
              >
                <option value="HIGHEST">🔴 Highest</option>
                <option value="HIGH">🟠 High</option>
                <option value="MEDIUM">🟡 Medium</option>
                <option value="LOW">🔵 Low</option>
              </select>
            </td>

            <!-- Assignee Dropdown (Inline Edit) -->
            <td class="py-3 px-3.5">
              <select
                :value="issue.assignee_id || ''"
                @change="(e) => handleAssigneeChange(issue, e.target.value)"
                class="bg-white border border-slate-300 rounded-lg px-2.5 py-1 text-xs text-slate-800 focus:outline-none cursor-pointer hover:bg-slate-50 max-w-[150px] truncate shadow-2xs font-medium"
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
            </td>

            <!-- Story Points (Inline Edit) -->
            <td class="py-3 px-3.5 text-center">
              <input
                type="number"
                min="0"
                max="99"
                :value="issue.story_points || 0"
                @change="(e) => handlePointsChange(issue, e.target.value)"
                class="w-14 bg-white border border-slate-300 rounded-lg px-2 py-1 text-xs font-semibold text-center text-slate-800 focus:outline-none focus:border-blue-600 shadow-2xs"
              />
            </td>

            <!-- Due Date (Inline Edit) -->
            <td class="py-3 px-3.5">
              <input
                type="date"
                :value="issue.due_date || ''"
                @change="(e) => handleDueDateChange(issue, e.target.value)"
                class="bg-white border border-slate-300 rounded-lg px-2.5 py-1 text-xs text-slate-700 focus:outline-none focus:border-blue-600 cursor-pointer shadow-2xs font-medium"
              />
            </td>

            <!-- Actions -->
            <td class="py-3 px-3.5 text-center">
              <div class="flex items-center justify-center space-x-1.5">
                <button
                  @click="projectStore.openIssueDetail(issue.id)"
                  class="p-1.5 rounded-lg text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition"
                  title="Buka Detail Tiket"
                >
                  <ExternalLink class="w-4 h-4" />
                </button>
                <button
                  @click="handleDelete(issue.id)"
                  class="p-1.5 rounded-lg text-slate-400 hover:text-red-600 hover:bg-red-50 transition"
                  title="Hapus Tiket"
                >
                  <Trash2 class="w-4 h-4" />
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
