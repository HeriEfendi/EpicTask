<script setup>
import { ref, computed, watch, onMounted, onBeforeUnmount } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useAuthStore } from '@/stores/auth';
import { useComponentStore } from '@/stores/component';
import { useVersionStore } from '@/stores/version';
import { useLabelStore } from '@/stores/label';
import { useSprintStore } from '@/stores/sprint';
import {
  Search,
  X,
  Check,
  Plus,
  ChevronDown,
  Bookmark,
  SlidersHorizontal,
  FolderGit2,
  CheckCircle2,
  Sparkles
} from 'lucide-vue-next';

const emit = defineEmits(['close']);

const projectStore = useProjectStore();
const authStore = useAuthStore();
const compStore = useComponentStore();
const versionStore = useVersionStore();
const labelStore = useLabelStore();
const sprintStore = useSprintStore();

// Header active tab: 'basic' | 'advanced' | 'jql'
const activeTab = ref('basic');

// Active left sidebar dimension
const activeDimension = ref('status');

// Right panel live search filter within current dimension options
const dimensionSearch = ref('');

// Dimension definitions
const allDimensions = [
  { id: 'status', label: 'Status' },
  { id: 'category', label: 'Status category' },
  { id: 'assignee', label: 'Assignee' },
  { id: 'type', label: 'Work type' },
  { id: 'priority', label: 'Priority' },
  { id: 'epic', label: 'Parent / Epic' },
  { id: 'sprint', label: 'Sprint' },
  { id: 'component', label: 'Components' },
  { id: 'version', label: 'Fix Version' },
  { id: 'label', label: 'Labels' },
  { id: 'resolution', label: 'Resolution' },
];

// Visible dimensions (can be customized via "+ Add field")
const visibleDimensionIds = ref([
  'status',
  'category',
  'assignee',
  'type',
  'priority',
  'epic',
  'sprint',
  'component',
  'version',
  'label',
  'resolution',
]);

const isAddFieldOpen = ref(false);
const isSavedFiltersOpen = ref(false);

// Reset search when switching dimension
watch(activeDimension, () => {
  dimensionSearch.value = '';
});

// Compute count of selected items per dimension
function getDimensionCount(dimId) {
  switch (dimId) {
    case 'status': return projectStore.filterStatuses.length;
    case 'category': return projectStore.filterCategories.length;
    case 'assignee': return projectStore.filterAssignees.length;
    case 'type': return projectStore.filterTypes.length;
    case 'priority': return projectStore.filterPriorities.length;
    case 'epic': return projectStore.filterEpics.length;
    case 'sprint': return projectStore.filterSprints.length;
    case 'component': return projectStore.filterComponents.length;
    case 'version': return projectStore.filterVersions.length;
    case 'label': return projectStore.filterLabels.length;
    case 'resolution': return projectStore.filterResolutions.length;
    default: return 0;
  }
}

// Total dimensions that have at least 1 filter active
const totalActiveDimensions = computed(() => projectStore.activeFilterDimensionsCount);

// Options for the currently active dimension
const rawOptions = computed(() => {
  switch (activeDimension.value) {
    case 'status':
      return projectStore.statuses.map((s) => ({
        id: s.id,
        label: s.name,
        color: s.color || '#64748b',
        category: s.category,
      }));

    case 'category':
      return [
        { id: 'TODO', label: 'To Do (TODO)', color: '#64748b', desc: 'Belum dimulai' },
        { id: 'IN_PROGRESS', label: 'In Progress', color: '#3b82f6', desc: 'Sedang dikerjakan' },
        { id: 'DONE', label: 'Done', color: '#10b981', desc: 'Telah selesai' },
      ];

    case 'assignee':
      return [
        { id: null, label: 'Unassigned (Belum Ditugaskan)', avatar: null, isUnassigned: true },
        ...authStore.allUsers.map((u) => ({
          id: u.id,
          label: u.full_name,
          email: u.email,
          avatar: u.avatar_url,
          isMe: u.id === authStore.user?.id,
        })),
      ];

    case 'type':
      return [
        { id: 'STORY', label: 'Story', icon: '📖', color: '#10b981', desc: 'Fitur pengguna' },
        { id: 'TASK', label: 'Task', icon: '📋', color: '#3b82f6', desc: 'Tugas teknis standar' },
        { id: 'BUG', label: 'Bug', icon: '🐞', color: '#ef4444', desc: 'Masalah / cacat sistem' },
        { id: 'EPIC', label: 'Epic', icon: '⚡', color: '#8b5cf6', desc: 'Inisiatif proyek besar' },
      ];

    case 'priority':
      return [
        { id: 'HIGHEST', label: 'Highest', icon: '🔴', color: '#dc2626', desc: 'Kritis / blocker' },
        { id: 'HIGH', label: 'High', icon: '🟠', color: '#ea580c', desc: 'Prioritas tinggi' },
        { id: 'MEDIUM', label: 'Medium', icon: '🟡', color: '#d97706', desc: 'Prioritas normal' },
        { id: 'LOW', label: 'Low', icon: '🔵', color: '#2563eb', desc: 'Dapat ditunda' },
        { id: 'LOWEST', label: 'Lowest', icon: '⚪', color: '#64748b', desc: 'Prioritas paling rendah' },
      ];

    case 'epic':
      return projectStore.epics.map((e) => ({
        id: e.id,
        label: e.summary,
        key: e.key,
        color: '#8b5cf6',
      }));

    case 'sprint':
      return (sprintStore.sprints || []).map((sp) => ({
        id: sp.id,
        label: sp.name,
        status: sp.status,
        goal: sp.goal,
      }));

    case 'component':
      return (compStore.components || []).map((c) => ({
        id: c.id,
        label: c.name,
        desc: c.description,
        leadName: c.lead_name,
      }));

    case 'version':
      return (versionStore.versions || []).map((v) => ({
        id: v.id,
        label: v.name,
        status: v.status,
        date: v.release_date,
      }));

    case 'label':
      return (labelStore.labels || []).map((l) => ({
        id: l.name,
        label: l.name,
        color: l.color || '#3b82f6',
      }));

    case 'resolution':
      return [
        { id: 'UNRESOLVED', label: 'Unresolved (Belum Terselesaikan)', desc: 'Tiket masih aktif' },
        { id: 'Fixed', label: 'Fixed', desc: 'Selesai dan diperbaiki' },
        { id: 'Done', label: 'Done', desc: 'Selesai dikerjakan' },
        { id: "Won't Fix", label: "Won't Fix", desc: 'Ditolak / tidak dikerjakan' },
        { id: 'Duplicate', label: 'Duplicate', desc: 'Duplikasi dari tiket lain' },
        { id: 'Cannot Reproduce', label: 'Cannot Reproduce', desc: 'Bug tidak dapat direproduksi' },
        { id: 'Incomplete', label: 'Incomplete', desc: 'Informasi tidak lengkap' },
      ];

    default:
      return [];
  }
});

// Filtered options based on user text search inside right panel
const filteredOptions = computed(() => {
  const q = dimensionSearch.value.trim().toLowerCase();
  if (!q) return rawOptions.value;
  return rawOptions.value.filter((opt) => {
    const textMatch = opt.label && opt.label.toLowerCase().includes(q);
    const descMatch = opt.desc && opt.desc.toLowerCase().includes(q);
    const emailMatch = opt.email && opt.email.toLowerCase().includes(q);
    const keyMatch = opt.key && opt.key.toLowerCase().includes(q);
    return textMatch || descMatch || emailMatch || keyMatch;
  });
});

// Check if an option is currently selected
function isSelected(optId) {
  return projectStore.isFilterItemSelected(activeDimension.value, optId);
}

// Toggle an option
function toggleOption(optId) {
  projectStore.toggleFilterItem(activeDimension.value, optId);
}

// Clear current dimension filter
function clearCurrentDimension() {
  projectStore.clearDimensionFilter(activeDimension.value);
}

// Clear all filters across all dimensions
function clearAll() {
  projectStore.clearAllAdvancedFilters();
}

// Toggle field visibility in left sidebar
function toggleFieldVisibility(dimId) {
  const idx = visibleDimensionIds.value.indexOf(dimId);
  if (idx === -1) {
    visibleDimensionIds.value.push(dimId);
  } else {
    // Don't allow removing all fields
    if (visibleDimensionIds.value.length > 1) {
      visibleDimensionIds.value.splice(idx, 1);
      if (activeDimension.value === dimId) {
        activeDimension.value = visibleDimensionIds.value[0];
      }
    }
  }
}

// Preset saved filters (Jira standard shortcuts)
function applyPreset(presetKey) {
  clearAll();
  isSavedFiltersOpen.value = false;
  switch (presetKey) {
    case 'my_open':
      if (authStore.user?.id) {
        projectStore.toggleFilterItem('assignee', authStore.user.id);
        projectStore.toggleFilterItem('resolution', 'UNRESOLVED');
      }
      break;
    case 'bugs':
      projectStore.toggleFilterItem('type', 'BUG');
      projectStore.toggleFilterItem('resolution', 'UNRESOLVED');
      break;
    case 'high_priority':
      projectStore.toggleFilterItem('priority', 'HIGHEST');
      projectStore.toggleFilterItem('priority', 'HIGH');
      break;
    case 'done_recently':
      projectStore.toggleFilterItem('category', 'DONE');
      break;
  }
}

// JQL simple query input state
const jqlQuery = ref('');
function applyJql() {
  if (!jqlQuery.value.trim()) return;
  projectStore.searchQuery = jqlQuery.value.trim();
  emit('close');
}

// Close on Escape key
function handleKeyDown(e) {
  if (e.key === 'Escape') {
    emit('close');
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown);
});

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleKeyDown);
});
</script>

<template>
  <!-- Backdrop Overlay -->
  <div
    class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-4 bg-slate-950/60 backdrop-blur-xs transition-opacity animate-fade-in"
    @click.self="emit('close')"
  >
    <!-- Modal Card (Jira Dark Theme Styled as per reference Image 2) -->
    <div
      class="w-[740px] max-w-[95vw] h-[540px] max-h-[85vh] bg-slate-900 text-slate-100 rounded-2xl border border-slate-700/80 shadow-2xl flex flex-col overflow-hidden animate-slide-up"
    >
      <!-- 1. Top Header -->
      <div class="px-5 py-3.5 border-b border-slate-800 flex items-center justify-between shrink-0 bg-slate-900/90">
        <!-- Mode Tabs -->
        <div class="flex items-center gap-1 bg-slate-800/80 p-1 rounded-xl border border-slate-700/60">
          <button
            @click="activeTab = 'basic'"
            class="px-3 py-1 rounded-lg text-xs font-semibold transition"
            :class="activeTab === 'basic' ? 'bg-blue-600 text-white shadow-xs' : 'text-slate-300 hover:text-white'"
          >
            Basic
          </button>
          <button
            @click="activeTab = 'advanced'"
            class="px-3 py-1 rounded-lg text-xs font-semibold transition"
            :class="activeTab === 'advanced' ? 'bg-blue-600 text-white shadow-xs' : 'text-slate-300 hover:text-white'"
          >
            Advanced
          </button>
          <button
            @click="activeTab = 'jql'"
            class="px-3 py-1 rounded-lg text-xs font-semibold transition flex items-center gap-1"
            :class="activeTab === 'jql' ? 'bg-blue-600 text-white shadow-xs' : 'text-slate-300 hover:text-white'"
          >
            <span>JQL</span>
            <span class="text-[9px] px-1 py-0.2 rounded bg-blue-500/30 text-blue-300 font-mono">beta</span>
          </button>
        </div>

        <!-- Right Actions: Saved Filters + Close -->
        <div class="flex items-center gap-2">
          <!-- Saved Filters Dropdown -->
          <div class="relative">
            <button
              @click="isSavedFiltersOpen = !isSavedFiltersOpen"
              class="flex items-center gap-1.5 px-2.5 py-1 text-xs text-slate-300 hover:text-white rounded-lg hover:bg-slate-800 border border-slate-700/70 transition"
            >
              <Bookmark class="w-3.5 h-3.5 text-blue-400" />
              <span>Saved filters</span>
              <ChevronDown class="w-3 h-3 text-slate-400" />
            </button>

            <!-- Dropdown Menu -->
            <div
              v-if="isSavedFiltersOpen"
              class="absolute right-0 top-full mt-1.5 w-56 bg-slate-850 rounded-xl border border-slate-700 shadow-xl z-50 p-1 text-xs text-slate-200"
            >
              <div class="px-2.5 py-1 text-[10px] font-bold text-slate-400 uppercase tracking-wider">
                Preset Standar Jira
              </div>
              <button
                @click="applyPreset('my_open')"
                class="w-full text-left px-2.5 py-1.5 rounded-lg hover:bg-slate-750 flex items-center gap-2"
              >
                <span>👤</span>
                <span>Tiket Saya Belum Selesai</span>
              </button>
              <button
                @click="applyPreset('bugs')"
                class="w-full text-left px-2.5 py-1.5 rounded-lg hover:bg-slate-750 flex items-center gap-2"
              >
                <span>🐞</span>
                <span>Semua Bug Aktif</span>
              </button>
              <button
                @click="applyPreset('high_priority')"
                class="w-full text-left px-2.5 py-1.5 rounded-lg hover:bg-slate-750 flex items-center gap-2"
              >
                <span>🔥</span>
                <span>Prioritas Kritis & Tinggi</span>
              </button>
              <button
                @click="applyPreset('done_recently')"
                class="w-full text-left px-2.5 py-1.5 rounded-lg hover:bg-slate-750 flex items-center gap-2"
              >
                <span>✅</span>
                <span>Isu Selesai (Done)</span>
              </button>
            </div>
          </div>

          <!-- Close Modal -->
          <button
            @click="emit('close')"
            class="p-1 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 transition"
            title="Tutup (Esc)"
          >
            <X class="w-4 h-4" />
          </button>
        </div>
      </div>

      <!-- 2. Body View: BASIC (2-Column Layout matching Image 2) -->
      <div v-if="activeTab === 'basic'" class="flex-1 flex overflow-hidden">
        <!-- LEFT SIDEBAR: Dimensions list -->
        <div class="w-56 border-r border-slate-800 flex flex-col justify-between shrink-0 bg-slate-900/60">
          <!-- Scrollable Dimension Buttons -->
          <div class="py-2 overflow-y-auto flex-1 space-y-0.5">
            <template v-for="dim in allDimensions" :key="dim.id">
              <button
                v-if="visibleDimensionIds.includes(dim.id)"
                @click="activeDimension = dim.id"
                class="w-full flex items-center justify-between px-3.5 py-2 text-xs transition text-left group"
                :class="activeDimension === dim.id
                  ? 'border-l-2 border-blue-500 bg-slate-800/80 text-white font-medium'
                  : 'text-slate-300 hover:bg-slate-800/40 hover:text-slate-100'"
              >
                <span class="truncate">{{ dim.label }}</span>
                <!-- Count Badge (matching Image 2) -->
                <span
                  v-if="getDimensionCount(dim.id) > 0"
                  class="px-1.5 py-0.2 rounded-full text-[11px] font-bold bg-blue-600 text-white shrink-0 shadow-2xs"
                >
                  {{ getDimensionCount(dim.id) }}
                </span>
              </button>
            </template>

            <!-- + Add Field Button -->
            <div class="relative px-3 pt-2">
              <button
                @click="isAddFieldOpen = !isAddFieldOpen"
                class="flex items-center gap-1.5 text-xs text-blue-400 hover:text-blue-300 transition py-1 font-medium"
              >
                <Plus class="w-3.5 h-3.5" />
                <span>Add field</span>
              </button>

              <!-- Add Field Popover Menu -->
              <div
                v-if="isAddFieldOpen"
                class="absolute left-3 top-full mt-1 w-48 bg-slate-850 rounded-xl border border-slate-700 shadow-xl z-50 p-1 text-xs text-slate-200 max-h-56 overflow-y-auto"
              >
                <div class="px-2 py-1 text-[10px] font-bold text-slate-400 uppercase tracking-wider">
                  Tampilkan Field
                </div>
                <label
                  v-for="d in allDimensions"
                  :key="d.id"
                  class="flex items-center gap-2 px-2 py-1 rounded hover:bg-slate-750 cursor-pointer"
                >
                  <input
                    type="checkbox"
                    :checked="visibleDimensionIds.includes(d.id)"
                    @change="toggleFieldVisibility(d.id)"
                    class="rounded border-slate-600 text-blue-600 focus:ring-0"
                  />
                  <span>{{ d.label }}</span>
                </label>
              </div>
            </div>
          </div>

          <!-- Bottom Left: Clear All Button -->
          <div class="p-3 border-t border-slate-800/80 bg-slate-900/80">
            <button
              @click="clearAll"
              :disabled="totalActiveDimensions === 0"
              class="text-xs transition font-semibold"
              :class="totalActiveDimensions > 0
                ? 'text-red-400 hover:text-red-300'
                : 'text-slate-500 cursor-not-allowed'"
            >
              Clear all
            </button>
          </div>
        </div>

        <!-- RIGHT PANEL: Checkbox List for Current Dimension -->
        <div class="flex-1 flex flex-col justify-between overflow-hidden bg-slate-900">
          <!-- Search input at top (matching Image 2) -->
          <div class="p-3.5 border-b border-slate-800">
            <div class="relative">
              <Search class="w-4 h-4 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2 pointer-events-none" />
              <input
                v-model="dimensionSearch"
                type="text"
                :placeholder="`Search ${allDimensions.find(d => d.id === activeDimension)?.label.toLowerCase()}...`"
                class="w-full bg-slate-800/80 border border-slate-700 rounded-lg pl-9 pr-8 py-1.5 text-xs text-slate-100
                       placeholder-slate-400 focus:outline-none focus:border-blue-500 transition"
              />
              <button
                v-if="dimensionSearch"
                @click="dimensionSearch = ''"
                class="absolute right-2.5 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-200"
              >
                <X class="w-3.5 h-3.5" />
              </button>
            </div>
          </div>

          <!-- Options List (Scrollable Checkboxes) -->
          <div class="flex-1 overflow-y-auto p-2 space-y-1">
            <div
              v-if="filteredOptions.length === 0"
              class="py-12 text-center text-xs text-slate-500"
            >
              Tidak ada opsi yang cocok dengan pencarian "{{ dimensionSearch }}"
            </div>

            <label
              v-for="opt in filteredOptions"
              :key="opt.id ?? 'unassigned'"
              class="flex items-center gap-3 px-3 py-2 rounded-lg cursor-pointer hover:bg-slate-800/70 transition group select-none"
              :class="isSelected(opt.id) ? 'bg-slate-800/50' : ''"
              @click.prevent="toggleOption(opt.id)"
            >
              <!-- Checkbox (matching Image 2) -->
              <div
                class="w-4 h-4 rounded border flex items-center justify-center shrink-0 transition-all"
                :class="isSelected(opt.id)
                  ? 'bg-blue-600 border-blue-600 text-white'
                  : 'border-slate-600 bg-slate-800 group-hover:border-slate-400'"
              >
                <Check v-if="isSelected(opt.id)" class="w-3 h-3 stroke-[3]" />
              </div>

              <!-- Option Content according to Dimension Type -->

              <!-- 1. Status Dimension: pill badge matching Image 2 -->
              <div v-if="activeDimension === 'status'" class="flex items-center gap-2 flex-1 min-w-0">
                <span
                  class="px-2 py-0.5 rounded text-[11px] font-semibold text-white tracking-wide shrink-0 shadow-2xs"
                  :style="{ backgroundColor: opt.color }"
                >
                  {{ opt.label }}
                </span>
                <span class="text-[11px] text-slate-400 truncate">({{ opt.category }})</span>
              </div>

              <!-- 2. Status Category Dimension -->
              <div v-else-if="activeDimension === 'category'" class="flex items-center gap-2 flex-1 min-w-0">
                <span
                  class="px-2 py-0.5 rounded text-[11px] font-semibold text-white shrink-0"
                  :style="{ backgroundColor: opt.color }"
                >
                  {{ opt.id }}
                </span>
                <span class="text-xs text-slate-300">{{ opt.desc }}</span>
              </div>

              <!-- 3. Assignee Dimension -->
              <div v-else-if="activeDimension === 'assignee'" class="flex items-center gap-2.5 flex-1 min-w-0">
                <template v-if="opt.isUnassigned">
                  <div class="w-6 h-6 rounded-full bg-slate-700 flex items-center justify-center text-[10px] text-slate-400 shrink-0">
                    👤
                  </div>
                  <span class="text-xs font-medium text-slate-300">Belum Ditugaskan (Unassigned)</span>
                </template>
                <template v-else>
                  <img
                    :src="opt.avatar || `https://api.dicebear.com/7.x/avataaars/svg?seed=${opt.label}`"
                    class="w-6 h-6 rounded-full border border-slate-700 bg-slate-800 shrink-0"
                    alt=""
                  />
                  <div class="flex items-center gap-2 truncate">
                    <span class="text-xs font-medium text-slate-200 truncate">{{ opt.label }}</span>
                    <span v-if="opt.isMe" class="px-1.5 py-0.2 rounded bg-blue-500/20 text-blue-400 text-[10px] font-bold">
                      Saya
                    </span>
                    <span class="text-[11px] text-slate-500 truncate">{{ opt.email }}</span>
                  </div>
                </template>
              </div>

              <!-- 4. Work Type Dimension -->
              <div v-else-if="activeDimension === 'type'" class="flex items-center gap-2.5 flex-1 min-w-0">
                <span class="text-sm shrink-0">{{ opt.icon }}</span>
                <span
                  class="px-1.5 py-0.5 rounded text-[11px] font-bold tracking-wider shrink-0"
                  :style="{ backgroundColor: opt.color + '22', color: opt.color }"
                >
                  {{ opt.id }}
                </span>
                <span class="text-xs text-slate-400 truncate">{{ opt.desc }}</span>
              </div>

              <!-- 5. Priority Dimension -->
              <div v-else-if="activeDimension === 'priority'" class="flex items-center gap-2.5 flex-1 min-w-0">
                <span class="text-sm shrink-0">{{ opt.icon }}</span>
                <span class="text-xs font-semibold text-slate-200">{{ opt.label }}</span>
                <span class="text-[11px] text-slate-400 truncate">({{ opt.desc }})</span>
              </div>

              <!-- 6. Epic Dimension -->
              <div v-else-if="activeDimension === 'epic'" class="flex items-center gap-2 flex-1 min-w-0">
                <span class="px-1.5 py-0.5 rounded bg-purple-500/20 text-purple-300 font-mono text-[10px] shrink-0">
                  {{ opt.key }}
                </span>
                <span class="text-xs font-medium text-slate-200 truncate">{{ opt.label }}</span>
              </div>

              <!-- 7. Sprint Dimension -->
              <div v-else-if="activeDimension === 'sprint'" class="flex items-center gap-2 flex-1 min-w-0">
                <span class="text-xs font-medium text-slate-200 truncate">{{ opt.label }}</span>
                <span
                  class="px-1.5 py-0.2 rounded text-[10px] font-bold uppercase tracking-wider shrink-0"
                  :class="{
                    'bg-emerald-500/20 text-emerald-300': opt.status === 'ACTIVE',
                    'bg-amber-500/20 text-amber-300': opt.status === 'PLANNED',
                    'bg-slate-700 text-slate-300': opt.status === 'COMPLETED'
                  }"
                >
                  {{ opt.status }}
                </span>
              </div>

              <!-- 8. Component Dimension -->
              <div v-else-if="activeDimension === 'component'" class="flex items-center gap-2 flex-1 min-w-0">
                <span class="text-xs font-medium text-slate-200 truncate">🧩 {{ opt.label }}</span>
                <span v-if="opt.leadName" class="text-[11px] text-slate-400 truncate">
                  (Lead: {{ opt.leadName }})
                </span>
              </div>

              <!-- 9. Version Dimension -->
              <div v-else-if="activeDimension === 'version'" class="flex items-center gap-2 flex-1 min-w-0">
                <span class="text-xs font-medium text-slate-200 truncate">🚀 {{ opt.label }}</span>
                <span
                  class="px-1.5 py-0.2 rounded text-[10px] font-bold uppercase tracking-wider shrink-0"
                  :class="{
                    'bg-blue-500/20 text-blue-300': opt.status === 'UNRELEASED',
                    'bg-emerald-500/20 text-emerald-300': opt.status === 'RELEASED',
                    'bg-slate-700 text-slate-300': opt.status === 'ARCHIVED'
                  }"
                >
                  {{ opt.status }}
                </span>
              </div>

              <!-- 10. Label Dimension -->
              <div v-else-if="activeDimension === 'label'" class="flex items-center gap-2 flex-1 min-w-0">
                <span
                  class="px-2 py-0.5 rounded-full text-[11px] font-semibold text-white tracking-wide shrink-0 shadow-2xs"
                  :style="{ backgroundColor: opt.color }"
                >
                  🏷️ {{ opt.label }}
                </span>
              </div>

              <!-- 11. Resolution Dimension -->
              <div v-else-if="activeDimension === 'resolution'" class="flex items-center gap-2 flex-1 min-w-0">
                <span class="text-xs font-semibold text-slate-200">✓ {{ opt.label }}</span>
                <span class="text-[11px] text-slate-400 truncate">({{ opt.desc }})</span>
              </div>
            </label>
          </div>

          <!-- Bottom Right Footer (matching Image 2) -->
          <div class="p-3 border-t border-slate-800 flex items-center justify-between shrink-0 bg-slate-900/80">
            <button
              @click="clearCurrentDimension"
              :disabled="getDimensionCount(activeDimension) === 0"
              class="text-xs transition font-semibold"
              :class="getDimensionCount(activeDimension) > 0
                ? 'text-blue-400 hover:text-blue-300'
                : 'text-slate-500 cursor-not-allowed'"
            >
              Clear
            </button>
            <span class="text-xs text-slate-400">
              {{ filteredOptions.length }} of {{ rawOptions.length }}
            </span>
          </div>
        </div>
      </div>

      <!-- 3. Body View: ADVANCED (Quick Filters Matrix) -->
      <div v-else-if="activeTab === 'advanced'" class="flex-1 p-5 overflow-y-auto space-y-4">
        <div class="flex items-center justify-between border-b border-slate-800 pb-3">
          <div>
            <h3 class="text-sm font-bold text-slate-100">Jira Quick Filter Dimensions</h3>
            <p class="text-xs text-slate-400 mt-0.5">Toggle filter instan satu-klik lintas papan</p>
          </div>
          <button
            @click="projectStore.clearQuickFilters()"
            class="text-xs text-red-400 hover:text-red-300 font-semibold"
          >
            Reset Quick Filters
          </button>
        </div>

        <div class="grid grid-cols-2 gap-3 pt-2">
          <button
            v-for="qf in [
              { key: 'my_issues', label: 'Tiket Saya', icon: '👤', desc: 'Hanya tiket yang ditugaskan ke saya' },
              { key: 'only_bugs', label: 'Hanya Bug', icon: '🐞', desc: 'Isu dengan tipe BUG' },
              { key: 'highest_priority', label: 'Prioritas Tinggi', icon: '🔥', desc: 'Prioritas HIGHEST dan HIGH' },
              { key: 'unassigned', label: 'Belum Ditugaskan', icon: '❓', desc: 'Tiket tanpa penanggung jawab' },
              { key: 'due_this_week', label: 'Due Minggu Ini', icon: '📅', desc: 'Jatuh tempo minggu ini' },
              { key: 'unresolved', label: 'Belum Selesai', icon: '⏳', desc: 'Belum berstatus DONE / unresolved' },
            ]"
            :key="qf.key"
            @click="projectStore.toggleQuickFilter(qf.key)"
            class="flex items-start gap-3 p-3 rounded-xl border text-left transition"
            :class="projectStore.isQuickFilterActive(qf.key)
              ? 'bg-blue-600/20 border-blue-500 text-white shadow-xs'
              : 'bg-slate-800/50 border-slate-700/80 text-slate-300 hover:bg-slate-800 hover:text-white'"
          >
            <span class="text-xl shrink-0">{{ qf.icon }}</span>
            <div class="min-w-0">
              <div class="flex items-center gap-1.5 font-semibold text-xs">
                <span>{{ qf.label }}</span>
                <Check v-if="projectStore.isQuickFilterActive(qf.key)" class="w-3.5 h-3.5 text-blue-400 shrink-0" />
              </div>
              <p class="text-[11px] text-slate-400 mt-0.5 leading-snug">{{ qf.desc }}</p>
            </div>
          </button>
        </div>
      </div>

      <!-- 4. Body View: JQL (Query Bar) -->
      <div v-else-if="activeTab === 'jql'" class="flex-1 p-5 overflow-y-auto space-y-4">
        <div>
          <h3 class="text-sm font-bold text-slate-100">Jira Query Language (JQL) Search</h3>
          <p class="text-xs text-slate-400 mt-0.5">Ketik pencarian kata kunci atau query terstruktur</p>
        </div>

        <div class="space-y-2">
          <div class="relative">
            <input
              v-model="jqlQuery"
              @keydown.enter="applyJql"
              type="text"
              placeholder="e.g. project = EPIC AND status = Done AND priority = High"
              class="w-full bg-slate-950 font-mono border border-slate-700 rounded-xl px-4 py-3 text-xs text-blue-300 placeholder-slate-500 focus:outline-none focus:border-blue-500 shadow-inner"
            />
          </div>
          <div class="flex items-center justify-between text-[11px] text-slate-400">
            <span>Tekan <strong class="text-slate-200">Enter</strong> untuk mengeksekusi pencarian</span>
            <button
              @click="applyJql"
              class="px-3 py-1 rounded-lg bg-blue-600 hover:bg-blue-500 text-white font-semibold text-xs transition"
            >
              Cari JQL
            </button>
          </div>
        </div>

        <!-- Syntax hints -->
        <div class="bg-slate-800/40 rounded-xl p-3 border border-slate-800 text-[11px] text-slate-400 space-y-1.5">
          <p class="font-bold text-slate-300 text-xs">💡 Contoh Sintaks JQL Populer:</p>
          <p>• <code>status IN ('To Do', 'In Progress')</code></p>
          <p>• <code>priority = 'HIGH' AND assignee = currentUser()</code></p>
          <p>• <code>type = 'BUG' AND resolution IS EMPTY</code></p>
        </div>
      </div>
    </div>
  </div>
</template>
