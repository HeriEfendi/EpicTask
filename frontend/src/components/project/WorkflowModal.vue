<script setup>
import { ref } from 'vue';
import { useProjectStore } from '@/stores/project';
import {
  Workflow,
  Plus,
  Trash2,
  ArrowRight,
  ShieldCheck,
  X,
  Palette
} from 'lucide-vue-next';

const props = defineProps({
  isOpen: Boolean,
});

const emit = defineEmits(['close']);

const projectStore = useProjectStore();

// New Status State
const newStatusName = ref('');
const newStatusCategory = ref('IN_PROGRESS');
const newStatusColor = ref('#3b82f6');

// New Transition State
const fromStatusId = ref(null);
const toStatusId = ref(null);

async function handleCreateStatus() {
  if (!newStatusName.value.trim()) return;
  try {
    await projectStore.createStatus(
      newStatusName.value.trim(),
      newStatusCategory.value,
      newStatusColor.value
    );
    newStatusName.value = '';
  } catch (e) {
    alert(e.message || 'Failed to create status');
  }
}

async function handleCreateTransition() {
  if (!fromStatusId.value || !toStatusId.value) return;
  if (fromStatusId.value === toStatusId.value) {
    alert('Source and destination status cannot be the same');
    return;
  }
  try {
    await projectStore.createTransition(Number(fromStatusId.value), Number(toStatusId.value));
    fromStatusId.value = null;
    toStatusId.value = null;
  } catch (e) {
    alert(e.message || 'Failed to save workflow transition');
  }
}

async function handleDeleteTransition(tid) {
  try {
    await projectStore.deleteTransition(tid);
  } catch (e) {
    alert(e.message || 'Failed to delete transition');
  }
}
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/40 backdrop-blur-sm"
  >
    <div class="glass-modal w-full max-w-2xl rounded-2xl p-6 border border-slate-200 bg-white max-h-[90vh] flex flex-col shadow-2xl animate-slide-up">
      <!-- Header -->
      <div class="flex items-center justify-between pb-4 border-b border-slate-200">
        <div class="flex items-center space-x-2.5">
          <div class="w-8 h-8 rounded-lg bg-blue-50 border border-blue-200 flex items-center justify-center text-blue-600">
            <Workflow class="w-4 h-4" />
          </div>
          <div>
            <h2 class="text-base font-bold text-slate-800">Custom Workflow & Transition Guards</h2>
            <p class="text-xs text-slate-500">Control board columns and enforce step-by-step transition rules</p>
          </div>
        </div>

        <button
          @click="emit('close')"
          class="p-1 rounded-lg text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto py-4 space-y-6">
        <!-- Info Alert -->
        <div class="bg-blue-50 border border-blue-200 rounded-xl p-3 flex items-start space-x-2.5 text-xs text-blue-900">
          <ShieldCheck class="w-4 h-4 text-blue-600 shrink-0 mt-0.5" />
          <div>
            <p class="font-bold text-blue-900">Workflow Rules Engine Active</p>
            <p class="text-[11px] text-blue-700">
              When transition rules exist for a status, cards CANNOT be dragged to any unlisted status. For example, restricting "To Do" so it can only move to "In Progress".
            </p>
          </div>
        </div>

        <!-- Section 1: Dynamic Status Columns (FR-2.3) -->
        <div class="space-y-3">
          <h3 class="text-xs font-bold text-slate-700 uppercase tracking-wider">
            Project Statuses (Columns)
          </h3>

          <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
            <div
              v-for="status in projectStore.statuses"
              :key="status.id"
              class="p-2.5 rounded-lg border border-slate-200 bg-slate-50 flex items-center justify-between shadow-2xs"
            >
              <div class="flex items-center space-x-2 truncate">
                <span class="w-3 h-3 rounded-full shrink-0" :style="{ backgroundColor: status.color || '#3b82f6' }"></span>
                <span class="text-xs font-semibold text-slate-800 truncate">{{ status.name }}</span>
              </div>
              <span class="text-[9px] uppercase px-1.5 py-0.5 rounded font-mono bg-white border border-slate-200 text-slate-600">
                {{ status.category }}
              </span>
            </div>
          </div>

          <!-- Add New Status Form -->
          <div class="p-3 bg-slate-50 border border-slate-200 rounded-xl flex flex-wrap items-center gap-2">
            <input
              v-model="newStatusName"
              placeholder="New status name (e.g. QA Testing)..."
              class="flex-1 min-w-[160px] bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs"
            />
            <select
              v-model="newStatusCategory"
              class="bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-700 focus:outline-none shadow-2xs"
            >
              <option value="TODO">Category: TODO</option>
              <option value="IN_PROGRESS">Category: IN PROGRESS</option>
              <option value="DONE">Category: DONE</option>
            </select>
            <input
              v-model="newStatusColor"
              type="color"
              class="w-8 h-8 rounded border border-slate-300 bg-white cursor-pointer p-0.5 shadow-2xs"
              title="Pick column color"
            />
            <button
              @click="handleCreateStatus"
              class="px-3.5 py-1.5 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-lg text-xs font-semibold flex items-center space-x-1 shadow-xs transition"
            >
              <Plus class="w-3.5 h-3.5" />
              <span>Add Status</span>
            </button>
          </div>
        </div>

        <!-- Section 2: Workflow Transition Guards (FR-2.3) -->
        <div class="space-y-3">
          <h3 class="text-xs font-bold text-slate-700 uppercase tracking-wider">
            Allowed Transition Rules
          </h3>

          <div v-if="projectStore.transitions.length === 0" class="p-3 text-xs text-slate-500 italic bg-slate-50 rounded-lg border border-slate-200">
            No restrictive rules defined. Cards can currently move freely between all columns. Add a rule below to lock down transitions!
          </div>

          <div v-else class="space-y-1.5 max-h-48 overflow-y-auto">
            <div
              v-for="t in projectStore.transitions"
              :key="t.id"
              class="p-2.5 rounded-lg border border-slate-200 bg-white flex items-center justify-between text-xs shadow-2xs"
            >
              <div class="flex items-center space-x-3">
                <span class="font-bold text-blue-700 px-2 py-0.5 rounded bg-blue-50 border border-blue-200">
                  {{ t.from_status_name }}
                </span>
                <ArrowRight class="w-3.5 h-3.5 text-slate-400" />
                <span class="font-bold text-emerald-700 px-2 py-0.5 rounded bg-emerald-50 border border-emerald-200">
                  {{ t.to_status_name }}
                </span>
              </div>

              <button
                @click="handleDeleteTransition(t.id)"
                class="p-1 text-slate-400 hover:text-red-600 hover:bg-red-50 rounded transition"
                title="Remove rule"
              >
                <Trash2 class="w-3.5 h-3.5" />
              </button>
            </div>
          </div>

          <!-- Add Transition Form -->
          <div class="p-3 bg-slate-50 border border-slate-200 rounded-xl flex flex-wrap items-center gap-2">
            <span class="text-xs text-slate-600 font-medium">Allow moving from</span>
            <select
              v-model="fromStatusId"
              class="bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-700 focus:outline-none shadow-2xs"
            >
              <option :value="null">Select origin...</option>
              <option v-for="s in projectStore.statuses" :key="s.id" :value="s.id">
                {{ s.name }}
              </option>
            </select>

            <span class="text-xs text-slate-600 font-medium">to</span>
            <select
              v-model="toStatusId"
              class="bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-700 focus:outline-none shadow-2xs"
            >
              <option :value="null">Select target...</option>
              <option v-for="s in projectStore.statuses" :key="s.id" :value="s.id">
                {{ s.name }}
              </option>
            </select>

            <button
              @click="handleCreateTransition"
              class="px-3.5 py-1.5 bg-emerald-600 hover:bg-emerald-700 active:bg-emerald-800 text-white rounded-lg text-xs font-semibold flex items-center space-x-1 shadow-xs transition"
            >
              <Plus class="w-3.5 h-3.5" />
              <span>Allow Transition</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
