<script setup>
import { ref, watch } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useAuthStore } from '@/stores/auth';
import { Plus, X, Layers, AlertCircle } from 'lucide-vue-next';

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
    errorMessage.value = 'Summary (title) is required';
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
    errorMessage.value = e.message || 'Failed to create issue';
  }
}
</script>

<template>
  <div
    v-if="projectStore.isCreateModalOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/50 backdrop-blur-sm"
  >
    <div class="glass-modal w-full max-w-xl rounded-2xl border border-slate-200 bg-white p-6 flex flex-col shadow-2xl animate-slide-up max-h-[90vh] overflow-y-auto">
      <div class="flex items-center justify-between pb-3 border-b border-slate-200 mb-4">
        <h2 class="text-base font-bold text-slate-800 flex items-center gap-2">
          <Plus class="w-4 h-4 text-blue-600" />
          <span>Create New Issue ({{ projectStore.currentProject?.key }})</span>
        </h2>
        <button
          @click="projectStore.isCreateModalOpen = false"
          class="p-1 rounded-lg text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <div v-if="errorMessage" class="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg text-xs text-red-600">
        {{ errorMessage }}
      </div>

      <div class="space-y-4 text-xs">
        <!-- Issue Type & Status -->
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="block text-slate-500 font-bold uppercase tracking-wider text-[10px] mb-1">Issue Type</label>
            <select
              v-model="issueType"
              class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-2 text-xs text-slate-800 focus:outline-none focus:border-blue-500 focus:bg-white"
            >
              <option value="TASK">Task (Standard Issue)</option>
              <option value="STORY">Story (Feature / User Story)</option>
              <option value="EPIC">Epic (Large Milestone)</option>
              <option value="BUG">Bug (Problem or Defect)</option>
            </select>
          </div>

          <div>
            <label class="block text-slate-500 font-bold uppercase tracking-wider text-[10px] mb-1">Initial Status</label>
            <select
              v-model="statusId"
              class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-2 text-xs text-slate-800 focus:outline-none focus:border-blue-500 focus:bg-white"
            >
              <option v-for="s in projectStore.statuses" :key="s.id" :value="s.id">
                {{ s.name }} ({{ s.category }})
              </option>
            </select>
          </div>
        </div>

        <!-- Summary -->
        <div>
          <label class="block text-slate-500 font-bold uppercase tracking-wider text-[10px] mb-1">Summary *</label>
          <input
            v-model="summary"
            placeholder="e.g. Implement Webhook retry policies"
            class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-800 focus:outline-none focus:border-blue-500 focus:bg-white placeholder-slate-400 font-medium"
            autofocus
            @keyup.enter="handleCreate"
          />
        </div>

        <!-- Description -->
        <div>
          <label class="block text-slate-500 font-bold uppercase tracking-wider text-[10px] mb-1">Description</label>
          <textarea
            v-model="description"
            rows="3"
            placeholder="Detailed description, requirements, or steps to reproduce..."
            class="w-full bg-slate-50 border border-slate-300 rounded-lg p-3 text-xs text-slate-800 focus:outline-none focus:border-blue-500 focus:bg-white placeholder-slate-400"
          ></textarea>
        </div>

        <!-- Assignee & Priority -->
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="block text-slate-500 font-bold uppercase tracking-wider text-[10px] mb-1">Assignee</label>
            <select
              v-model="assigneeId"
              class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-2 text-xs text-slate-800 focus:outline-none focus:bg-white"
            >
              <option :value="null">Unassigned</option>
              <option v-for="u in authStore.allUsers" :key="u.id" :value="u.id">
                {{ u.full_name }}
              </option>
            </select>
          </div>

          <div>
            <label class="block text-slate-500 font-bold uppercase tracking-wider text-[10px] mb-1">Priority</label>
            <select
              v-model="priority"
              class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-2 text-xs text-slate-800 focus:outline-none focus:bg-white"
            >
              <option value="HIGHEST">🔴 Highest</option>
              <option value="HIGH">🟠 High</option>
              <option value="MEDIUM">🟡 Medium</option>
              <option value="LOW">🔵 Low</option>
            </select>
          </div>
        </div>

        <!-- Epic & Story Points -->
        <div class="grid grid-cols-2 gap-3">
          <div v-if="issueType !== 'EPIC'">
            <label class="block text-slate-500 font-bold uppercase tracking-wider text-[10px] mb-1">Parent Epic</label>
            <select
              v-model="epicId"
              class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-2 text-xs text-slate-800 focus:outline-none focus:bg-white"
            >
              <option :value="null">No Epic</option>
              <option v-for="e in projectStore.epics" :key="e.id" :value="e.id">
                {{ e.summary }}
              </option>
            </select>
          </div>

          <div>
            <label class="block text-slate-500 font-bold uppercase tracking-wider text-[10px] mb-1">Story Points</label>
            <input
              v-model.number="storyPoints"
              type="number"
              min="0"
              class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-2 text-xs text-slate-800 focus:outline-none focus:bg-white"
            />
          </div>
        </div>

        <!-- Start Date & Due Date -->
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="block text-slate-500 font-bold uppercase tracking-wider text-[10px] mb-1">Start Date</label>
            <input
              v-model="startDate"
              type="date"
              class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-1.5 text-xs text-slate-800 focus:outline-none focus:bg-white"
            />
          </div>

          <div>
            <label class="block text-slate-500 font-bold uppercase tracking-wider text-[10px] mb-1">Due Date</label>
            <input
              v-model="dueDate"
              type="date"
              class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-1.5 text-xs text-slate-800 focus:outline-none focus:bg-white"
            />
          </div>
        </div>

        <!-- Actions -->
        <div class="flex justify-end space-x-2 pt-3 border-t border-slate-200">
          <button
            @click="projectStore.isCreateModalOpen = false"
            class="px-4 py-2 text-xs text-slate-500 hover:text-slate-800"
          >
            Cancel
          </button>
          <button
            @click="handleCreate"
            class="px-5 py-2 bg-blue-600 hover:bg-blue-500 text-white rounded-lg text-xs font-bold shadow-md shadow-blue-600/30 active:scale-95 transition"
          >
            Create Issue
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
