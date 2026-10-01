<script setup>
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useAuthStore } from '@/stores/auth';
import {
  ListTodo,
  Plus,
  Trash2,
  ExternalLink,
  ChevronDown,
  Loader2
} from 'lucide-vue-next';
import { toast } from '@/utils/toast';

const PAGE_SIZE = 20;

const projectStore = useProjectStore();
const authStore = useAuthStore();

// --- Infinite Scroll State ---
const visibleCount = ref(PAGE_SIZE);
const isLoadingMore = ref(false);
const tableScrollRef = ref(null); // ref to the scrollable div

// Reset visible count whenever the filtered dataset changes (filter/search)
watch(
  () => projectStore.filteredIssues.length,
  () => {
    visibleCount.value = PAGE_SIZE;
  }
);

// Sliced rows shown in the table
const visibleIssues = computed(() =>
  projectStore.filteredIssues.slice(0, visibleCount.value)
);

const totalCount = computed(() => projectStore.filteredIssues.length);
const hasMore = computed(() => visibleCount.value < totalCount.value);

function loadMore() {
  if (isLoadingMore.value || !hasMore.value) return;
  isLoadingMore.value = true;
  // Simulate a brief async pause so the loader is visible
  setTimeout(() => {
    visibleCount.value = Math.min(visibleCount.value + PAGE_SIZE, totalCount.value);
    isLoadingMore.value = false;
  }, 150);
}

// Scroll listener — trigger loadMore when near bottom
function handleScroll(e) {
  const el = e.target;
  const threshold = 100; // px from bottom
  if (el.scrollTop + el.clientHeight >= el.scrollHeight - threshold) {
    loadMore();
  }
}

onMounted(() => {
  if (tableScrollRef.value) {
    tableScrollRef.value.addEventListener('scroll', handleScroll);
  }
});

onUnmounted(() => {
  if (tableScrollRef.value) {
    tableScrollRef.value.removeEventListener('scroll', handleScroll);
  }
});

async function handleStatusChange(issue, newStatusId) {
  try {
    await projectStore.moveIssue(issue.id, Number(newStatusId));
    toast.success(`Status ${issue.key} diperbarui`);
  } catch (e) {
    toast.error(e.message || 'Transisi status ditolak oleh aturan workflow');
  }
}

async function handlePriorityChange(issue, newPriority) {
  try {
    await projectStore.updateIssue(issue.id, { priority: newPriority });
    toast.success(`Prioritas ${issue.key} diubah ke ${newPriority}`);
  } catch (e) {
    toast.error(e.message || 'Gagal mengubah prioritas');
  }
}

async function handleAssigneeChange(issue, newAssigneeId) {
  try {
    await projectStore.updateIssue(issue.id, {
      assignee_id: newAssigneeId ? Number(newAssigneeId) : null,
    });
    toast.success(`Penugasan ${issue.key} diperbarui`);
  } catch (e) {
    toast.error(e.message || 'Gagal mengubah penugasan');
  }
}

async function handleDueDateChange(issue, newDate) {
  try {
    await projectStore.updateIssue(issue.id, { due_date: newDate || null });
    toast.success(`Tenggat waktu ${issue.key} diperbarui`);
  } catch (e) {
    toast.error(e.message || 'Gagal mengubah tenggat waktu');
  }
}

async function handlePointsChange(issue, newPoints) {
  try {
    await projectStore.updateIssue(issue.id, { story_points: Number(newPoints) || 0 });
    toast.success(`Story point ${issue.key} diperbarui`);
  } catch (e) {
    toast.error(e.message || 'Gagal mengubah story points');
  }
}

async function handleDelete(issueId) {
  if (confirm('Yakin ingin menghapus tiket ini?')) {
    try {
      await projectStore.deleteIssue(issueId);
      toast.info('Tiket berhasil dihapus');
    } catch (e) {
      toast.error(e.message || 'Gagal menghapus tiket');
    }
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
        {{ totalCount }} item tiket
      </span>
    </div>

    <!-- Table Container with scroll listener -->
    <div
      ref="tableScrollRef"
      class="flex-1 overflow-auto mt-4 border border-slate-200 rounded-xl bg-white shadow-xs"
    >
      <table class="w-full text-left text-sm divide-y divide-slate-200 select-none">
        <!-- Table Header -->
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


          <!-- Issues Rows (sliced to visibleIssues) -->
          <tr
            v-for="issue in visibleIssues"
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

            <!-- Status Dropdown -->
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

            <!-- Priority Dropdown -->
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

            <!-- Assignee Dropdown -->
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

            <!-- Story Points -->
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

            <!-- Due Date -->
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

          <!-- Load More Spinner Row (shown while loading next batch) -->
          <tr v-if="isLoadingMore">
            <td colspan="9" class="py-4 text-center">
              <div class="flex items-center justify-center space-x-2 text-slate-400">
                <Loader2 class="w-4 h-4 animate-spin" />
                <span class="text-xs font-medium">Memuat lebih banyak data...</span>
              </div>
            </td>
          </tr>

          <!-- Empty state -->
          <tr v-if="totalCount === 0">
            <td colspan="9" class="py-16 text-center text-slate-400">
              <ListTodo class="w-10 h-10 mx-auto mb-3 text-slate-300" />
              <p class="text-sm font-medium">Belum ada tiket. Buat tiket pertama di baris atas!</p>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Footer: row count + Load More button -->
    <div class="mt-3 flex items-center justify-between px-1">
      <!-- Row counter: "Menampilkan X dari Y tiket" -->
      <div class="flex items-center space-x-2">
        <div class="h-1.5 bg-slate-200 rounded-full w-32 overflow-hidden">
          <div
            class="h-full bg-blue-500 rounded-full transition-all duration-300"
            :style="{ width: totalCount > 0 ? `${(visibleCount / totalCount) * 100}%` : '0%' }"
          ></div>
        </div>
        <span class="text-xs text-slate-500 font-medium">
          Menampilkan
          <span class="font-bold text-slate-800">{{ Math.min(visibleCount, totalCount) }}</span>
          dari
          <span class="font-bold text-slate-800">{{ totalCount }}</span>
          tiket
        </span>
      </div>

      <!-- Load More button (visible only if there's more data) -->
      <button
        v-if="hasMore"
        @click="loadMore"
        :disabled="isLoadingMore"
        class="flex items-center space-x-1.5 px-3.5 py-1.5 rounded-lg border border-slate-200 bg-white hover:bg-slate-50 text-xs font-semibold text-slate-700 shadow-2xs transition disabled:opacity-60"
      >
        <Loader2 v-if="isLoadingMore" class="w-3.5 h-3.5 animate-spin text-blue-500" />
        <ChevronDown v-else class="w-3.5 h-3.5 text-slate-500" />
        <span>{{ isLoadingMore ? 'Memuat...' : `Muat 20 lagi` }}</span>
      </button>

      <!-- All loaded indicator -->
      <span v-else-if="totalCount > 0" class="text-xs text-slate-400 font-medium italic">
        ✓ Semua tiket ditampilkan
      </span>
    </div>
  </div>
</template>
