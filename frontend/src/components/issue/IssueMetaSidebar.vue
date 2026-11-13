<script setup>
import { computed } from 'vue';
import { Layers, Flag, User, Hash, Calendar, Milestone, Boxes, Rocket, Tag } from 'lucide-vue-next';
import { useSprintStore } from '@/stores/sprint';
import { useComponentStore } from '@/stores/component';
import { useVersionStore } from '@/stores/version';
import IssueLabelsEditor from './IssueLabelsEditor.vue';

const sprintStore = useSprintStore();
const compStore = useComponentStore();
const versionStore = useVersionStore();

const props = defineProps({
  isCreateMode: {
    type: Boolean,
    default: false
  },
  isDetailMode: {
    type: Boolean,
    default: false
  },
  issue: {
    type: Object,
    default: null
  },
  createForm: {
    type: Object,
    default: () => ({})
  },
  statuses: {
    type: Array,
    default: () => []
  },
  users: {
    type: Array,
    default: () => []
  },
  epics: {
    type: Array,
    default: () => []
  }
});

const emit = defineEmits(['update-field', 'status-change']);

function handleFieldUpdate(fields) {
  emit('update-field', fields);
}

function handleStatusSelect(e) {
  emit('status-change', e);
}

const currentIssueLabels = computed(() => {
  if (!props.issue?.labels) return [];
  return props.issue.labels
    .split(',')
    .map((l) => l.trim())
    .filter(Boolean);
});
</script>

<template>
  <div class="p-6 space-y-5 bg-slate-50/70 text-sm overflow-y-auto h-full">
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
        v-model="createForm.type"
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
        @change="(e) => handleFieldUpdate({ issue_type: e.target.value })"
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
        v-model="createForm.statusId"
        class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
      >
        <option v-for="s in statuses" :key="s.id" :value="s.id">
          {{ s.name }} ({{ s.category }})
        </option>
      </select>

      <!-- Detail Mode Status -->
      <select
        v-else-if="issue"
        :value="issue.status_id"
        @change="handleStatusSelect"
        class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm font-semibold text-slate-800 focus:outline-none focus:border-blue-600 cursor-pointer shadow-2xs"
      >
        <option v-for="s in statuses" :key="s.id" :value="s.id">
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
        v-model="createForm.priority"
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
        @change="(e) => handleFieldUpdate({ priority: e.target.value })"
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
        v-model="createForm.assigneeId"
        class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-medium"
      >
        <option :value="null">Belum Ditugaskan</option>
        <option v-for="u in users" :key="u.id" :value="u.id">
          {{ u.full_name }} ({{ u.email }})
        </option>
      </select>

      <!-- Detail Mode Assignee -->
      <select
        v-else-if="issue"
        :value="issue.assignee_id || ''"
        @change="(e) => handleFieldUpdate({ assignee_id: e.target.value ? Number(e.target.value) : null })"
        class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-800 focus:outline-none cursor-pointer shadow-2xs font-medium"
      >
        <option value="">Belum Ditugaskan</option>
        <option v-for="u in users" :key="u.id" :value="u.id">
          {{ u.full_name }}
        </option>
      </select>
    </div>

    <!-- Parent Epic (if not EPIC) -->
    <div v-if="(isCreateMode && createForm.type !== 'EPIC') || (isDetailMode && issue && issue.issue_type !== 'EPIC')" class="space-y-1.5">
      <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider">Parent Epic</label>

      <!-- Create Mode Epic -->
      <select
        v-if="isCreateMode"
        v-model="createForm.epicId"
        class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-medium"
      >
        <option :value="null">Tanpa Epic</option>
        <option v-for="e in epics" :key="e.id" :value="e.id">
          🟣 {{ e.summary }}
        </option>
      </select>

      <!-- Detail Mode Epic -->
      <select
        v-else-if="issue"
        :value="issue.epic_id || ''"
        @change="(e) => handleFieldUpdate({ epic_id: e.target.value ? Number(e.target.value) : null })"
        class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-medium cursor-pointer"
      >
        <option value="">Tanpa Epic</option>
        <option v-for="e in epics" :key="e.id" :value="e.id">
          🟣 {{ e.summary }}
        </option>
      </select>
    </div>

    <!-- Sprint (Scrum Framework) -->
    <div class="space-y-1.5">
      <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1">
        <Milestone class="w-3 h-3 text-slate-500" />
        <span>Sprint</span>
      </label>

      <!-- Create Mode Sprint -->
      <select
        v-if="isCreateMode"
        v-model="createForm.sprintId"
        class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-medium"
      >
        <option :value="null">📦 Product Backlog</option>
        <option v-for="s in sprintStore.sprints" :key="s.id" :value="s.id">
          {{ s.status === 'ACTIVE' ? '🟢' : '⚪' }} {{ s.name }} ({{ s.status }})
        </option>
      </select>

      <!-- Detail Mode Sprint -->
      <select
        v-else-if="issue"
        :value="issue.sprint_id || ''"
        @change="(e) => handleFieldUpdate({ sprint_id: e.target.value ? Number(e.target.value) : null })"
        class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-medium cursor-pointer"
      >
        <option value="">📦 Product Backlog</option>
        <option v-for="s in sprintStore.sprints" :key="s.id" :value="s.id">
          {{ s.status === 'ACTIVE' ? '🟢' : '⚪' }} {{ s.name }} ({{ s.status }})
        </option>
      </select>
    </div>

    <!-- Komponen (Sub-Sistem Proyek) -->
    <div class="space-y-1.5">
      <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1">
        <Boxes class="w-3 h-3 text-slate-500" />
        <span>Komponen</span>
      </label>

      <!-- Create Mode Component -->
      <select
        v-if="isCreateMode"
        v-model="createForm.componentId"
        class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-medium"
      >
        <option :value="null">Tanpa Komponen</option>
        <option v-for="c in compStore.components" :key="c.id" :value="c.id">
          🧩 {{ c.name }}
        </option>
      </select>

      <!-- Detail Mode Component -->
      <select
        v-else-if="issue"
        :value="issue.component_id || ''"
        @change="(e) => handleFieldUpdate({ component_id: e.target.value ? Number(e.target.value) : null })"
        class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-medium cursor-pointer"
      >
        <option value="">Tanpa Komponen</option>
        <option v-for="c in compStore.components" :key="c.id" :value="c.id">
          🧩 {{ c.name }}
        </option>
      </select>
    </div>

    <!-- Fix Version (Versi Rilis) -->
    <div class="space-y-1.5">
      <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1">
        <Rocket class="w-3 h-3 text-slate-500" />
        <span>Fix Version (Rilis)</span>
      </label>

      <!-- Create Mode Fix Version -->
      <select
        v-if="isCreateMode"
        v-model="createForm.fixVersionId"
        class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-medium"
      >
        <option :value="null">Tanpa Versi Rilis</option>
        <option v-for="v in versionStore.versions" :key="v.id" :value="v.id">
          🚀 {{ v.name }} ({{ v.status }})
        </option>
      </select>

      <!-- Detail Mode Fix Version -->
      <select
        v-else-if="issue"
        :value="issue.fix_version_id || ''"
        @change="(e) => handleFieldUpdate({ fix_version_id: e.target.value ? Number(e.target.value) : null })"
        class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none shadow-2xs font-medium cursor-pointer"
      >
        <option value="">Tanpa Versi Rilis</option>
        <option v-for="v in versionStore.versions" :key="v.id" :value="v.id">
          🚀 {{ v.name }} ({{ v.status }})
        </option>
      </select>
    </div>

    <!-- Labels (Tagging Fleksibel) -->
    <div class="space-y-1.5">
      <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1">
        <Tag class="w-3 h-3 text-slate-500" />
        <span>Label / Tags</span>
      </label>

      <!-- Create Mode Labels -->
      <IssueLabelsEditor
        v-if="isCreateMode"
        v-model="createForm.labels"
      />

      <!-- Detail Mode Labels -->
      <IssueLabelsEditor
        v-else-if="issue"
        :model-value="currentIssueLabels"
        @change="(newLabels) => handleFieldUpdate({ labels: newLabels })"
      />
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
        v-model.number="createForm.storyPoints"
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
          @click="handleFieldUpdate({ story_points: p })"
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
          v-model="createForm.startDate"
          type="date"
          class="w-full bg-white border border-slate-300 rounded-lg px-3 py-1.5 text-sm text-slate-800 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
        />
        <input
          v-else-if="issue"
          type="date"
          :value="issue.start_date || ''"
          @change="(e) => handleFieldUpdate({ start_date: e.target.value || null })"
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
          v-model="createForm.dueDate"
          type="date"
          class="w-full bg-white border border-slate-300 rounded-lg px-3 py-1.5 text-sm text-slate-800 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
        />
        <input
          v-else-if="issue"
          type="date"
          :value="issue.due_date || ''"
          @change="(e) => handleFieldUpdate({ due_date: e.target.value || null })"
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
</template>

