<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useAuthStore } from '@/stores/auth';
import { useAutomationStore } from '@/stores/automation';
import {
  Kanban,
  CalendarRange,
  ListTodo,
  BarChart3,
  Bot,
  Workflow,
  X,
  Users,
  ChevronDown,
  Check,
} from 'lucide-vue-next';

const emit = defineEmits(['open-workflow-modal']);

const projectStore = useProjectStore();
const authStore = useAuthStore();
const autoStore = useAutomationStore();

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

// Other filters
const hasActiveFilters = computed(() => {
  return (
    !!projectStore.searchQuery ||
    projectStore.filterAssignees.length > 0 ||
    !!projectStore.filterAssignee ||
    !!projectStore.filterPriority ||
    !!projectStore.filterEpic ||
    !!projectStore.filterType
  );
});

function clearFilters() {
  projectStore.searchQuery = '';
  projectStore.filterAssignees = [];
  projectStore.filterAssignee = null;
  projectStore.filterPriority = null;
  projectStore.filterEpic = null;
  projectStore.filterType = null;
}

// View tabs definition
const views = [
  { id: 'kanban',   label: 'Kanban',   icon: Kanban },
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

    <!-- ── ROW 2: Filter Bar ── -->
    <!-- Hidden when on Reports view (reports has its own controls) -->
    <div
      v-if="projectStore.activeView !== 'reports'"
      class="px-4 py-2 flex items-center flex-wrap gap-2"
    >

      <!-- ① Assignee Multi-Checkbox Dropdown "Tiket Saya" -->
      <div class="relative" ref="assigneeDropdownRef">
        <button
          @click="isAssigneeDropdownOpen = !isAssigneeDropdownOpen"
          class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold border transition-all shadow-2xs"
          :class="hasAssigneeFilter
            ? 'bg-blue-600 border-blue-600 text-white hover:bg-blue-700'
            : 'bg-white border-slate-300 text-slate-700 hover:border-slate-400 hover:bg-slate-50'"
        >
          <Users class="w-3.5 h-3.5 shrink-0" :class="hasAssigneeFilter ? 'text-white' : 'text-slate-500'" />
          <span class="max-w-[120px] truncate">{{ assigneeLabel }}</span>
          <ChevronDown
            class="w-3 h-3 shrink-0 transition-transform"
            :class="[hasAssigneeFilter ? 'text-white' : 'text-slate-400', isAssigneeDropdownOpen ? 'rotate-180' : '']"
          />
        </button>

        <!-- Dropdown Panel -->
        <div
          v-if="isAssigneeDropdownOpen"
          class="absolute left-0 top-full mt-1.5 w-64 bg-white rounded-xl border border-slate-200 shadow-xl z-50 overflow-hidden animate-slide-up"
        >
          <!-- Header -->
          <div class="px-3 pt-3 pb-2 border-b border-slate-100">
            <div class="flex items-center justify-between">
              <span class="text-xs font-bold text-slate-700 uppercase tracking-wider">Filter Assignee</span>
              <button
                v-if="hasAssigneeFilter"
                @click="clearAssigneeFilter"
                class="text-[11px] text-blue-600 hover:text-blue-800 font-semibold"
              >
                Reset
              </button>
            </div>
            <p class="text-[11px] text-slate-400 mt-0.5">Tampilkan tiket dari anggota berikut</p>
          </div>

          <!-- User List -->
          <div class="py-1 max-h-52 overflow-y-auto">
            <label
              v-for="u in authStore.allUsers"
              :key="u.id"
              class="flex items-center gap-2.5 px-3 py-2 cursor-pointer hover:bg-slate-50 transition group"
            >
              <!-- Custom Checkbox -->
              <div
                class="w-4 h-4 rounded border-2 flex items-center justify-center shrink-0 transition-all"
                :class="isAssigneeSelected(u.id)
                  ? 'bg-blue-600 border-blue-600'
                  : 'border-slate-300 bg-white group-hover:border-blue-400'"
                @click="toggleAssignee(u.id)"
              >
                <Check v-if="isAssigneeSelected(u.id)" class="w-2.5 h-2.5 text-white" />
              </div>

              <!-- Avatar + Name (click anywhere toggles) -->
              <div class="flex items-center gap-2 flex-1 min-w-0" @click="toggleAssignee(u.id)">
                <img
                  :src="u.avatar_url || `https://api.dicebear.com/7.x/avataaars/svg?seed=${u.full_name}`"
                  class="w-6 h-6 rounded-full border border-slate-200 bg-slate-100 shrink-0"
                  :alt="u.full_name"
                />
                <div class="truncate">
                  <p class="text-xs font-semibold text-slate-800 truncate leading-tight">{{ u.full_name }}</p>
                  <p class="text-[11px] text-slate-400 truncate leading-tight">{{ u.email }}</p>
                </div>
              </div>

              <!-- "Me" badge -->
              <span
                v-if="u.id === authStore.user?.id"
                class="text-[10px] font-bold px-1.5 py-0.5 rounded bg-blue-100 text-blue-700 shrink-0"
              >
                Saya
              </span>
            </label>
          </div>

          <!-- Footer: active count -->
          <div v-if="hasAssigneeFilter" class="px-3 py-2 border-t border-slate-100 bg-slate-50">
            <p class="text-[11px] text-slate-500">
              Menampilkan tiket dari
              <strong class="text-slate-800">{{ projectStore.filterAssignees.length }}</strong>
              anggota dipilih
            </p>
          </div>
        </div>
      </div>

      <!-- ② Priority Filter -->
      <div class="relative">
        <select
          v-model="projectStore.filterPriority"
          class="bg-white border border-slate-300 hover:border-slate-400 rounded-lg pl-3 pr-7 py-1.5 text-xs text-slate-700
                 focus:outline-none focus:border-blue-500 appearance-none cursor-pointer shadow-2xs transition font-medium"
          :class="projectStore.filterPriority ? 'border-amber-400 bg-amber-50 text-amber-800' : ''"
        >
          <option :value="null">Semua Prioritas</option>
          <option value="HIGHEST">🔴 Tertinggi</option>
          <option value="HIGH">🟠 Tinggi</option>
          <option value="MEDIUM">🟡 Sedang</option>
          <option value="LOW">🔵 Rendah</option>
        </select>
        <ChevronDown class="w-3.5 h-3.5 text-slate-400 absolute right-2 top-1/2 -translate-y-1/2 pointer-events-none" />
      </div>

      <!-- ③ Epic Filter -->
      <div class="relative" v-if="projectStore.epics.length > 0">
        <select
          v-model="projectStore.filterEpic"
          class="bg-white border border-slate-300 hover:border-slate-400 rounded-lg pl-3 pr-7 py-1.5 text-xs text-slate-700
                 focus:outline-none focus:border-blue-500 appearance-none cursor-pointer shadow-2xs transition font-medium max-w-[180px]"
          :class="projectStore.filterEpic ? 'border-purple-400 bg-purple-50 text-purple-800' : ''"
        >
          <option :value="null">Semua Epic</option>
          <option v-for="epic in projectStore.epics" :key="epic.id" :value="epic.id">
            🟣 {{ epic.summary }}
          </option>
        </select>
        <ChevronDown class="w-3.5 h-3.5 text-slate-400 absolute right-2 top-1/2 -translate-y-1/2 pointer-events-none" />
      </div>

      <!-- ④ Active Filter Chips Summary -->
      <div v-if="hasAssigneeFilter" class="flex items-center gap-1">
        <span
          v-for="uid in projectStore.filterAssignees"
          :key="uid"
          class="flex items-center gap-1 pl-1.5 pr-2 py-0.5 rounded-full bg-blue-100 border border-blue-200 text-blue-700 text-[11px] font-semibold"
        >
          <img
            :src="authStore.allUsers.find(u => u.id === uid)?.avatar_url || `https://api.dicebear.com/7.x/avataaars/svg?seed=${uid}`"
            class="w-4 h-4 rounded-full border border-blue-200"
            alt=""
          />
          {{ authStore.allUsers.find(u => u.id === uid)?.full_name?.split(' ')[0] || 'User' }}
          <button @click="toggleAssignee(uid)" class="hover:text-blue-900 ml-0.5">
            <X class="w-2.5 h-2.5" />
          </button>
        </span>
      </div>

      <!-- ⑤ Reset All Filters -->
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
  </div>
</template>
