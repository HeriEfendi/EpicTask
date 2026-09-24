<script setup>
import { ref } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useAuthStore } from '@/stores/auth';
import {
  ListTodo,
  Plus,
  Trash2,
  ExternalLink,
  CheckCircle2,
  Calendar,
  AlertCircle
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
    alert(e.message || 'Failed to create issue');
  }
}

async function handleStatusChange(issue, newStatusId) {
  try {
    await projectStore.moveIssue(issue.id, Number(newStatusId));
  } catch (e) {
    alert(e.message || 'Status transition rejected by workflow rules');
  }
}

async function handlePriorityChange(issue, newPriority) {
  try {
    await projectStore.updateIssue(issue.id, { priority: newPriority });
  } catch (e) {
    alert(e.message || 'Failed to update priority');
  }
}

async function handleAssigneeChange(issue, newAssigneeId) {
  try {
    await projectStore.updateIssue(issue.id, {
      assignee_id: newAssigneeId ? Number(newAssigneeId) : null,
    });
  } catch (e) {
    alert(e.message || 'Failed to update assignee');
  }
}

async function handleDueDateChange(issue, newDate) {
  try {
    await projectStore.updateIssue(issue.id, { due_date: newDate || null });
  } catch (e) {
    alert(e.message || 'Failed to update due date');
  }
}

async function handlePointsChange(issue, newPoints) {
  try {
    await projectStore.updateIssue(issue.id, { story_points: Number(newPoints) || 0 });
  } catch (e) {
    alert(e.message || 'Failed to update points');
  }
}

async function handleDelete(issueId) {
  if (confirm('Are you sure you want to delete this issue?')) {
    await projectStore.deleteIssue(issueId);
  }
}
</script>

<template>
  <div class="flex-1 p-6 flex flex-col overflow-hidden bg-slate-50">
    <div class="flex items-center justify-between pb-4 border-b border-slate-200">
      <h2 class="text-sm font-bold text-slate-800 flex items-center gap-2">
        <ListTodo class="w-4 h-4 text-blue-600" />
        <span>Spreadsheet List View (Instant Inline Editing)</span>
      </h2>
      <span class="text-xs text-slate-500 font-medium">
        {{ projectStore.filteredIssues.length }} items
      </span>
    </div>

    <!-- Table Container -->
    <div class="flex-1 overflow-auto mt-4 border border-slate-200 rounded-xl bg-white shadow-xs">
      <table class="w-full text-left text-xs divide-y divide-slate-200 select-none">
        <!-- Table Header -->
        <thead class="bg-slate-100 text-slate-600 font-semibold sticky top-0 z-10 border-b border-slate-200">
          <tr>
            <th class="py-3 px-3 w-28">Key</th>
            <th class="py-3 px-3 w-28">Type</th>
            <th class="py-3 px-3 min-w-[280px]">Summary</th>
            <th class="py-3 px-3 w-36">Status</th>
            <th class="py-3 px-3 w-32">Priority</th>
            <th class="py-3 px-3 w-36">Assignee</th>
            <th class="py-3 px-3 w-24">Points</th>
            <th class="py-3 px-3 w-36">Due Date</th>
            <th class="py-3 px-3 w-20 text-center">Actions</th>
          </tr>
        </thead>

        <tbody class="divide-y divide-slate-100 text-slate-700">
          <!-- Quick Inline Add Row -->
          <tr class="bg-blue-50/50 hover:bg-blue-50 transition border-b border-blue-100">
            <td class="py-2.5 px-3 font-mono text-[11px] text-blue-600 font-bold">+ New</td>
            <td class="py-2.5 px-3">
              <select
                v-model="quickType"
                class="bg-white border border-slate-300 rounded px-2 py-1 text-xs text-slate-700 focus:outline-none focus:border-blue-500"
              >
                <option value="TASK">TASK</option>
                <option value="STORY">STORY</option>
                <option value="BUG">BUG</option>
                <option value="EPIC">EPIC</option>
              </select>
            </td>
            <td class="py-2.5 px-3">
              <input
                v-model="quickSummary"
                placeholder="What needs to be done? Press Enter to add..."
                class="w-full bg-white border border-slate-300 rounded px-2.5 py-1 text-xs text-slate-800 placeholder-slate-400 focus:outline-none focus:border-blue-500"
                @keyup.enter="handleQuickCreate"
              />
            </td>
            <td class="py-2.5 px-3 text-slate-400 italic text-[11px]">To Do</td>
            <td class="py-2.5 px-3">
              <select
                v-model="quickPriority"
                class="bg-white border border-slate-300 rounded px-2 py-1 text-xs text-slate-700 focus:outline-none"
              >
                <option value="HIGHEST">HIGHEST</option>
                <option value="HIGH">HIGH</option>
                <option value="MEDIUM">MEDIUM</option>
                <option value="LOW">LOW</option>
              </select>
            </td>
            <td class="py-2.5 px-3 text-slate-400 italic text-[11px]">Unassigned</td>
            <td class="py-2.5 px-3 text-slate-400 text-center">-</td>
            <td class="py-2.5 px-3 text-slate-400 italic text-[11px]">None</td>
            <td class="py-2.5 px-3 text-center">
              <button
                @click="handleQuickCreate"
                class="px-2.5 py-1 bg-blue-600 hover:bg-blue-500 text-white rounded font-medium text-xs shadow-xs"
              >
                Add
              </button>
            </td>
          </tr>

          <!-- Issues Rows -->
          <tr
            v-for="issue in projectStore.filteredIssues"
            :key="issue.id"
            class="hover:bg-slate-50 transition group"
          >
            <!-- Key -->
            <td
              @click="projectStore.openIssueDetail(issue.id)"
              class="py-2.5 px-3 font-mono font-semibold text-blue-600 cursor-pointer hover:underline"
            >
              {{ issue.key }}
            </td>

            <!-- Type -->
            <td class="py-2.5 px-3">
              <span
                class="text-[10px] font-bold px-1.5 py-0.5 rounded uppercase"
                :class="`badge-type-${issue.issue_type}`"
              >
                {{ issue.issue_type }}
              </span>
            </td>

            <!-- Summary -->
            <td
              @click="projectStore.openIssueDetail(issue.id)"
              class="py-2.5 px-3 font-medium text-slate-800 cursor-pointer group-hover:text-blue-600"
            >
              <div class="flex items-center space-x-2">
                <span class="truncate max-w-md">{{ issue.summary }}</span>
                <span
                  v-if="issue.epic_summary && issue.issue_type !== 'EPIC'"
                  class="text-[9px] px-1.5 py-0.2 rounded-full bg-purple-50 text-purple-700 border border-purple-200"
                >
                  {{ issue.epic_summary }}
                </span>
              </div>
            </td>

            <!-- Status Dropdown (Inline Edit with transition rule check) -->
            <td class="py-2.5 px-3">
              <select
                :value="issue.status_id"
                @change="(e) => handleStatusChange(issue, e.target.value)"
                class="bg-white border border-slate-200 rounded px-2 py-1 text-xs text-slate-800 focus:outline-none focus:border-blue-500 cursor-pointer hover:bg-slate-50"
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
            <td class="py-2.5 px-3">
              <select
                :value="issue.priority"
                @change="(e) => handlePriorityChange(issue, e.target.value)"
                class="border border-slate-200 rounded px-2 py-1 text-xs focus:outline-none cursor-pointer"
                :class="`badge-priority-${issue.priority}`"
              >
                <option value="HIGHEST">🔴 Highest</option>
                <option value="HIGH">🟠 High</option>
                <option value="MEDIUM">🟡 Medium</option>
                <option value="LOW">🔵 Low</option>
              </select>
            </td>

            <!-- Assignee Dropdown (Inline Edit) -->
            <td class="py-2.5 px-3">
              <select
                :value="issue.assignee_id || ''"
                @change="(e) => handleAssigneeChange(issue, e.target.value)"
                class="bg-white border border-slate-200 rounded px-2 py-1 text-xs text-slate-800 focus:outline-none cursor-pointer hover:bg-slate-50 max-w-[130px] truncate"
              >
                <option value="">Unassigned</option>
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
            <td class="py-2.5 px-3">
              <input
                type="number"
                min="0"
                max="99"
                :value="issue.story_points || 0"
                @change="(e) => handlePointsChange(issue, e.target.value)"
                class="w-14 bg-white border border-slate-200 rounded px-2 py-1 text-xs text-center text-slate-800 focus:outline-none focus:border-blue-500"
              />
            </td>

            <!-- Due Date (Inline Edit) -->
            <td class="py-2.5 px-3">
              <input
                type="date"
                :value="issue.due_date || ''"
                @change="(e) => handleDueDateChange(issue, e.target.value)"
                class="bg-white border border-slate-200 rounded px-2 py-1 text-xs text-slate-700 focus:outline-none focus:border-blue-500 cursor-pointer"
              />
            </td>

            <!-- Actions -->
            <td class="py-2.5 px-3 text-center">
              <div class="flex items-center justify-center space-x-1">
                <button
                  @click="projectStore.openIssueDetail(issue.id)"
                  class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition"
                  title="Open Issue Details"
                >
                  <ExternalLink class="w-3.5 h-3.5" />
                </button>
                <button
                  @click="handleDelete(issue.id)"
                  class="p-1 rounded text-slate-400 hover:text-red-600 hover:bg-red-50 transition"
                  title="Delete Issue"
                >
                  <Trash2 class="w-3.5 h-3.5" />
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
