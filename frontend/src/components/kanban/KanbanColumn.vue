<script setup>
import { Plus } from 'lucide-vue-next';
import KanbanCard from './KanbanCard.vue';

const props = defineProps({
  status: {
    type: Object,
    required: true,
  },
  issues: {
    type: Array,
    default: () => [],
  },
  isDragOver: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['dragover', 'dragleave', 'drop', 'create-issue']);
</script>

<template>
  <div
    class="w-80 shrink-0 bg-slate-100/90 rounded-2xl border border-slate-200/90 flex flex-col max-h-[calc(100vh-190px)] transition-all kanban-column shadow-2xs"
    :class="{ 'drag-over': isDragOver }"
    @dragover="(e) => emit('dragover', e)"
    @dragleave="(e) => emit('dragleave', e)"
    @drop="(e) => emit('drop', e)"
  >
    <!-- Column Header -->
    <div class="p-3.5 border-b border-slate-200/70 flex items-center justify-between select-none">
      <div class="flex items-center space-x-2">
        <span
          class="w-3 h-3 rounded-full"
          :style="{ backgroundColor: status.color || '#3b82f6' }"
        ></span>
        <span class="text-sm font-bold text-slate-800 tracking-wide uppercase">
          {{ status.name }}
        </span>
        <span class="text-xs font-mono font-semibold px-2 py-0.5 rounded-full bg-white text-slate-600 border border-slate-200/80 shadow-2xs">
          {{ issues.length }}
        </span>
      </div>

      <button
        @click="emit('create-issue', status.id)"
        class="p-1 rounded-md hover:bg-slate-200/80 text-slate-400 hover:text-slate-700 transition"
        title="Tambah tiket di kolom ini"
      >
        <Plus class="w-4 h-4" />
      </button>
    </div>

    <!-- Cards List -->
    <div class="p-3 overflow-y-auto space-y-3 flex-1 min-h-[140px]">
      <KanbanCard
        v-for="issue in issues"
        :key="issue.id"
        :issue="issue"
      />

      <!-- Empty Column Drop Zone Placeholder -->
      <div
        v-if="issues.length === 0"
        class="h-32 border border-dashed border-slate-300 rounded-xl flex flex-col items-center justify-center text-slate-400 text-xs select-none"
      >
        <span class="font-medium">Belum ada tiket di kolom ini</span>
        <span class="text-slate-400 mt-1">Tarik & lepas tiket ke sini</span>
      </div>
    </div>

    <!-- Column Footer: Quick Add Button -->
    <div class="p-2 border-t border-slate-200/60 bg-slate-100/70 rounded-b-2xl">
      <button
        @click="emit('create-issue', status.id)"
        class="w-full py-2 px-3 rounded-xl text-sm font-semibold text-slate-600 hover:text-slate-900 hover:bg-white hover:border-slate-300 hover:shadow-2xs border border-transparent flex items-center justify-center space-x-1.5 transition"
      >
        <Plus class="w-4 h-4 text-slate-500" />
        <span>Tambah Tiket</span>
      </button>
    </div>
  </div>
</template>
