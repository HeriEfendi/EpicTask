<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useAuthStore } from '@/stores/auth';
import { useAutomationStore } from '@/stores/automation';
import { useComponentStore } from '@/stores/component';
import { useVersionStore } from '@/stores/version';
import { useLabelStore } from '@/stores/label';
import JiraFilterModal from '@/components/filter/JiraFilterModal.vue';
import {
  Kanban,
  Layers,
  CalendarRange,
  ListTodo,
  BarChart3,
  Bot,
  Workflow,
  X,
  Users,
  ChevronDown,
  Check,
  Boxes,
  Rocket,
  Tag,
  Search,
  ListFilter
} from 'lucide-vue-next';

const emit = defineEmits(['open-workflow-modal']);

const projectStore = useProjectStore();
const authStore = useAuthStore();
const autoStore = useAutomationStore();
const compStore = useComponentStore();
const versionStore = useVersionStore();
const labelStore = useLabelStore();

// Assignee dropdown state
const isAssigneeDropdownOpen = ref(false);
const assigneeDropdownRef = ref(null);

// Close dropdown when clicking outside
function handleClickOutside(e) {
  if (assigneeDropdownRef.value && !assigneeDropdownRef.value.contains(e.target)) {
    isAssigneeDropdownOpen.value = false;
  }
}
onMounted(() => document.addEventListener('mousedown', handleClickOutside));
onBeforeUnmount(() => document.removeEventListener('mousedown', handleClickOutside));

// Toggle a user in/out of filterAssignees array
function toggleAssignee(userId) {
  const idx = projectStore.filterAssignees.indexOf(userId);
  if (idx === -1) {
    projectStore.filterAssignees.push(userId);
  } else {
    projectStore.filterAssignees.splice(idx, 1);
  }
}

function isAssigneeSelected(userId) {
  return projectStore.filterAssignees.includes(userId);
}

// Computed label for the Tiket Saya button
const assigneeLabel = computed(() => {
  const len = projectStore.filterAssignees.length;
  if (len === 0) return 'Semua Anggota';
  if (len === 1) {
    const u = authStore.allUsers.find((u) => u.id === projectStore.filterAssignees[0]);
    const name = u?.full_name?.split(' ')[0] || 'User';
    return name;
  }
  return `${len} Terpilih`;
});

const hasAssigneeFilter = computed(() => projectStore.filterAssignees.length > 0);

function clearAssigneeFilter() {
  projectStore.filterAssignees = [];
}

// Active filters detection
const hasActiveFilters = computed(() => {
  return (
    !!projectStore.searchQuery ||
    projectStore.activeFilterDimensionsCount > 0 ||
    projectStore.filterAssignees.length > 0 ||
    !!projectStore.filterAssignee ||
    !!projectStore.filterPriority ||
    !!projectStore.filterEpic ||
    !!projectStore.filterType ||
    !!projectStore.filterComponent ||
    !!projectStore.filterVersion ||
    !!projectStore.filterLabel ||
    !!projectStore.filterSprint ||
    (projectStore.quickFilters && projectStore.quickFilters.length > 0)
  );
});

function clearFilters() {
  projectStore.clearAllAdvancedFilters();
}

// View tabs definition
const views = [
  { id: 'kanban',   label: 'Kanban',   icon: Kanban },
  { id: 'backlog',  label: 'Backlog',  icon: Layers },
  { id: 'releases', label: 'Rilis',    icon: Rocket },
  { id: 'timeline', label: 'Timeline', icon: CalendarRange },
  { id: 'list',     label: 'List',     icon: ListTodo },
  { id: 'reports',  label: 'Laporan',  icon: BarChart3 },
];
</script>

<template>
  <div class="border-b border-slate-200 bg-white shadow-2xs">
    <!-- ── ROW 1: View Tabs (left)  +  Tool Buttons (right) ── -->
    <div class="px-4 pt-2.5 pb-0 flex items-center justify-between gap-2">

      <!-- Left: View Switcher Tabs -->
      <div class="flex items-center">
        <div class="inline-flex items-center bg-slate-100/80 p-1 rounded-xl border border-slate-200/80 gap-0.5">
          <button
            v-for="v in views"
            :key="v.id"
            @click="projectStore.activeView = v.id"
            class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-sm font-medium transition-all duration-150"
            :class="projectStore.activeView === v.id
              ? 'bg-white text-blue-700 shadow font-semibold border border-blue-100'
              : 'text-slate-500 hover:text-slate-800 hover:bg-white/60'"
          >
            <component
              :is="v.icon"
              class="w-4 h-4 shrink-0"
              :class="projectStore.activeView === v.id ? 'text-blue-600' : 'text-slate-400'"
            />
            <span>{{ v.label }}</span>
          </button>
        </div>
      </div>

      <!-- Right: Otomasi + Aturan Workflow (always right-aligned) -->
      <div class="flex items-center gap-2 ml-auto pb-1">
        <!-- Automation Engine Button -->
        <button
          @click="autoStore.isRuleModalOpen = true"
          title="Konfigurasi Aturan Otomasi"
          class="group flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold
                 bg-purple-50 hover:bg-purple-100 border border-purple-200 hover:border-purple-300
                 text-purple-700 transition-all duration-150 shadow-2xs"
        >
          <Bot class="w-3.5 h-3.5 text-purple-600 group-hover:scale-110 transition-transform" />
          <span>Otomasi</span>
        </button>

        <!-- Workflow Rules Button -->
        <button
          @click="emit('open-workflow-modal')"
          title="Aturan Alur Kerja & Guard Transisi"
          class="group flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold
                 bg-blue-50 hover:bg-blue-100 border border-blue-200 hover:border-blue-300
                 text-blue-700 transition-all duration-150 shadow-2xs"
        >
          <Workflow class="w-3.5 h-3.5 text-blue-600 group-hover:rotate-12 transition-transform" />
          <span>Aturan Workflow</span>
        </button>
      </div>
    </div>

    <!-- ── ROW 2: Unified Jira Enterprise Filter Bar (Matching Reference Image 2) ── -->
    <!-- Hidden when on Reports view (reports has its own controls) -->
    <div
      v-if="projectStore.activeView !== 'reports'"
      class="px-4 py-2 flex items-center flex-wrap gap-2 text-xs"
    >
      <!-- ① Search Input -->
      <div class="relative w-48 sm:w-56">
        <Search class="w-3.5 h-3.5 text-slate-400 absolute left-2.5 top-1/2 -translate-y-1/2 pointer-events-none" />
        <input
          v-model="projectStore.searchQuery"
          type="text"
          placeholder="Cari tiket atau key..."
          class="w-full bg-white border border-slate-300 hover:border-slate-400 rounded-lg pl-8 pr-7 py-1.5 text-xs text-slate-800
                 placeholder-slate-400 focus:outline-none focus:border-blue-500 shadow-2xs transition"
        />
        <button
          v-if="projectStore.searchQuery"
          @click="projectStore.searchQuery = ''"
          class="absolute right-2 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600"
        >
          <X class="w-3 h-3" />
        </button>
      </div>

      <!-- ② Main Jira Filter Button [ ≡ Filter (N) ] (Matches Reference Image 2) -->
      <button
        @click="projectStore.isFilterModalOpen = true"
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg font-semibold border transition-all duration-150 shadow-2xs group"
        :class="projectStore.activeFilterDimensionsCount > 0
          ? 'bg-blue-600 border-blue-600 text-white hover:bg-blue-700'
          : 'bg-white border-slate-300 text-slate-700 hover:border-slate-400 hover:bg-slate-50'"
        title="Buka Filter Multi-Dimensi Standar Jira"
      >
        <ListFilter
          class="w-3.5 h-3.5 shrink-0"
          :class="projectStore.activeFilterDimensionsCount > 0 ? 'text-white' : 'text-slate-500 group-hover:text-slate-700'"
        />
        <span>Filter</span>
        <!-- Active Filter Count Badge (matching Image 2) -->
        <span
          v-if="projectStore.activeFilterDimensionsCount > 0"
          class="ml-0.5 px-1.5 py-0.2 rounded-full text-[10px] font-bold bg-white text-blue-700 shadow-2xs"
        >
          {{ projectStore.activeFilterDimensionsCount }}
        </span>
      </button>

      <!-- ③ Active Dimension Filter Chips with 1-Click Clear -->
      <div v-if="projectStore.activeFilterDimensionsCount > 0" class="flex items-center flex-wrap gap-1.5">
        <!-- Status Chip -->
        <span
          v-if="projectStore.filterStatuses.length > 0"
          class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-blue-50 border border-blue-200 text-blue-700 text-[11px] font-medium"
        >
          <span>Status: <strong>{{ projectStore.filterStatuses.length }}</strong></span>
          <button
            @click="projectStore.clearDimensionFilter('status')"
            class="hover:text-blue-900 ml-0.5"
            title="Hapus filter status"
          >
            <X class="w-2.5 h-2.5" />
          </button>
        </span>

        <!-- Status Category Chip -->
        <span
          v-if="projectStore.filterCategories.length > 0"
          class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-slate-100 border border-slate-200 text-slate-700 text-[11px] font-medium"
        >
          <span>Category: <strong>{{ projectStore.filterCategories.length }}</strong></span>
          <button
            @click="projectStore.clearDimensionFilter('category')"
            class="hover:text-slate-900 ml-0.5"
            title="Hapus filter category"
          >
            <X class="w-2.5 h-2.5" />
          </button>
        </span>

        <!-- Assignee Chip -->
        <span
          v-if="projectStore.filterAssignees.length > 0"
          class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-indigo-50 border border-indigo-200 text-indigo-700 text-[11px] font-medium"
        >
          <span>Assignee: <strong>{{ projectStore.filterAssignees.length }}</strong></span>
          <button
            @click="projectStore.clearDimensionFilter('assignee')"
            class="hover:text-indigo-900 ml-0.5"
            title="Hapus filter assignee"
          >
            <X class="w-2.5 h-2.5" />
          </button>
        </span>

        <!-- Work Type Chip -->
        <span
          v-if="projectStore.filterTypes.length > 0"
          class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-emerald-50 border border-emerald-200 text-emerald-700 text-[11px] font-medium"
        >
          <span>Type: <strong>{{ projectStore.filterTypes.length }}</strong></span>
          <button
            @click="projectStore.clearDimensionFilter('type')"
            class="hover:text-emerald-900 ml-0.5"
            title="Hapus filter work type"
          >
            <X class="w-2.5 h-2.5" />
          </button>
        </span>

        <!-- Priority Chip -->
        <span
          v-if="projectStore.filterPriorities.length > 0"
          class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-amber-50 border border-amber-200 text-amber-700 text-[11px] font-medium"
        >
          <span>Priority: <strong>{{ projectStore.filterPriorities.length }}</strong></span>
          <button
            @click="projectStore.clearDimensionFilter('priority')"
            class="hover:text-amber-900 ml-0.5"
            title="Hapus filter priority"
          >
            <X class="w-2.5 h-2.5" />
          </button>
        </span>

        <!-- Epic Chip -->
        <span
          v-if="projectStore.filterEpics.length > 0"
          class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-purple-50 border border-purple-200 text-purple-700 text-[11px] font-medium"
        >
          <span>Epic: <strong>{{ projectStore.filterEpics.length }}</strong></span>
          <button
            @click="projectStore.clearDimensionFilter('epic')"
            class="hover:text-purple-900 ml-0.5"
            title="Hapus filter epic"
          >
            <X class="w-2.5 h-2.5" />
          </button>
        </span>

        <!-- Sprint Chip -->
        <span
          v-if="projectStore.filterSprints.length > 0"
          class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-sky-50 border border-sky-200 text-sky-700 text-[11px] font-medium"
        >
          <span>Sprint: <strong>{{ projectStore.filterSprints.length }}</strong></span>
          <button
            @click="projectStore.clearDimensionFilter('sprint')"
            class="hover:text-sky-900 ml-0.5"
            title="Hapus filter sprint"
          >
            <X class="w-2.5 h-2.5" />
          </button>
        </span>

        <!-- Component Chip -->
        <span
          v-if="projectStore.filterComponents.length > 0"
          class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-cyan-50 border border-cyan-200 text-cyan-700 text-[11px] font-medium"
        >
          <span>Component: <strong>{{ projectStore.filterComponents.length }}</strong></span>
          <button
            @click="projectStore.clearDimensionFilter('component')"
            class="hover:text-cyan-900 ml-0.5"
            title="Hapus filter component"
          >
            <X class="w-2.5 h-2.5" />
          </button>
        </span>

        <!-- Version Chip -->
        <span
          v-if="projectStore.filterVersions.length > 0"
          class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-teal-50 border border-teal-200 text-teal-700 text-[11px] font-medium"
        >
          <span>Version: <strong>{{ projectStore.filterVersions.length }}</strong></span>
          <button
            @click="projectStore.clearDimensionFilter('version')"
            class="hover:text-teal-900 ml-0.5"
            title="Hapus filter version"
          >
            <X class="w-2.5 h-2.5" />
          </button>
        </span>

        <!-- Label Chip -->
        <span
          v-if="projectStore.filterLabels.length > 0"
          class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-violet-50 border border-violet-200 text-violet-700 text-[11px] font-medium"
        >
          <span>Label: <strong>{{ projectStore.filterLabels.length }}</strong></span>
          <button
            @click="projectStore.clearDimensionFilter('label')"
            class="hover:text-violet-900 ml-0.5"
            title="Hapus filter label"
          >
            <X class="w-2.5 h-2.5" />
          </button>
        </span>

        <!-- Resolution Chip -->
        <span
          v-if="projectStore.filterResolutions.length > 0"
          class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-rose-50 border border-rose-200 text-rose-700 text-[11px] font-medium"
        >
          <span>Resolution: <strong>{{ projectStore.filterResolutions.length }}</strong></span>
          <button
            @click="projectStore.clearDimensionFilter('resolution')"
            class="hover:text-rose-900 ml-0.5"
            title="Hapus filter resolution"
          >
            <X class="w-2.5 h-2.5" />
          </button>
        </span>
      </div>

      <!-- ④ Reset All Filters -->
      <button
        v-if="hasActiveFilters"
        @click="clearFilters"
        class="flex items-center gap-1.5 px-3 py-1.5 text-xs text-red-600 hover:text-red-700
               hover:bg-red-50 rounded-lg border border-red-200 hover:border-red-300 transition font-semibold ml-auto shadow-2xs"
        title="Reset semua filter"
      >
        <X class="w-3 h-3" />
        <span>Reset Filter</span>
      </button>
    </div>

    <!-- ── Jira Enterprise Filter Dialog Modal ── -->
    <JiraFilterModal
      v-if="projectStore.isFilterModalOpen"
      @close="projectStore.isFilterModalOpen = false"
    />
  </div>
</template>
