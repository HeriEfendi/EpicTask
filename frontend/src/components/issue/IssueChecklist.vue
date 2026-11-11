<script setup>
import { ref } from 'vue';
import { CheckSquare, Plus } from 'lucide-vue-next';

const props = defineProps({
  subtasks: {
    type: Array,
    default: () => []
  },
  subtaskStats: {
    type: Object,
    default: null
  }
});

const emit = defineEmits(['toggle-done', 'add-subtask']);

const newSubtaskSummary = ref('');

function handleAdd() {
  if (!newSubtaskSummary.value.trim()) return;
  emit('add-subtask', newSubtaskSummary.value.trim());
  newSubtaskSummary.value = '';
}
</script>

<template>
  <div class="space-y-3.5 bg-slate-50 border border-slate-200 rounded-xl p-4">
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
        v-for="sub in subtasks"
        :key="sub.id"
        class="flex items-center justify-between p-2.5 rounded-lg bg-white border border-slate-200 hover:border-slate-300 transition shadow-2xs"
      >
        <div class="flex items-center space-x-3 truncate">
          <input
            type="checkbox"
            :checked="sub.status_category === 'DONE'"
            @change="emit('toggle-done', sub)"
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
        @keyup.enter="handleAdd"
      />
      <button
        @click="handleAdd"
        class="px-4 py-2 bg-white hover:bg-slate-50 text-slate-700 border border-slate-300 rounded-lg text-sm font-semibold flex items-center space-x-1 shadow-2xs transition"
      >
        <Plus class="w-4 h-4" />
        <span>Tambah</span>
      </button>
    </div>
  </div>
</template>

