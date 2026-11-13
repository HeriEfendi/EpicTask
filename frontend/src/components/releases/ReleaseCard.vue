<script setup>
import { computed } from 'vue';
import {
  Rocket,
  Calendar,
  CheckCircle2,
  Archive,
  MoreVertical,
  Edit2,
  Trash2,
  Clock,
  AlertCircle
} from 'lucide-vue-next';
import ReleaseProgressBar from './ReleaseProgressBar.vue';

const props = defineProps({
  version: {
    type: Object,
    required: true,
  },
});

const emit = defineEmits(['release', 'edit', 'archive', 'delete']);

const isOverdue = computed(() => {
  if (!props.version.release_date || props.version.status === 'RELEASED') return false;
  const target = new Date(props.version.release_date);
  const now = new Date();
  now.setHours(0, 0, 0, 0);
  return target < now;
});

const formattedReleaseDate = computed(() => {
  if (!props.version.release_date) return 'Tanpa tanggal rilis';
  return new Date(props.version.release_date).toLocaleDateString('id-ID', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
  });
});

const formattedStartDate = computed(() => {
  if (!props.version.start_date) return null;
  return new Date(props.version.start_date).toLocaleDateString('id-ID', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
  });
});

const statusBadgeClasses = computed(() => {
  switch (props.version.status) {
    case 'RELEASED':
      return 'bg-emerald-50 text-emerald-700 border-emerald-200';
    case 'ARCHIVED':
      return 'bg-slate-100 text-slate-600 border-slate-200';
    default:
      return 'bg-blue-50 text-blue-700 border-blue-200';
  }
});

const statusLabel = computed(() => {
  switch (props.version.status) {
    case 'RELEASED':
      return 'Sudah Dirilis';
    case 'ARCHIVED':
      return 'Diarsipkan';
    default:
      return 'Belum Dirilis';
  }
});
</script>

<template>
  <div
    class="bg-white rounded-xl border border-slate-200 hover:border-slate-300 shadow-2xs hover:shadow-md transition-all duration-200 p-5 space-y-4"
  >
    <!-- Header: Title, Status, and Action Buttons -->
    <div class="flex items-start justify-between gap-4">
      <div class="space-y-1">
        <div class="flex items-center gap-2.5 flex-wrap">
          <div class="w-8 h-8 rounded-lg bg-indigo-50 border border-indigo-100 flex items-center justify-center text-indigo-600 shrink-0">
            <Rocket class="w-4 h-4" />
          </div>
          <h3 class="text-base font-bold text-slate-800 hover:text-blue-600 transition">
            {{ version.name }}
          </h3>
          <span
            class="text-xs px-2.5 py-0.5 rounded-full font-semibold border"
            :class="statusBadgeClasses"
          >
            {{ statusLabel }}
          </span>
          <span
            v-if="isOverdue"
            class="text-xs px-2 py-0.5 rounded-full font-semibold bg-rose-50 text-rose-700 border border-rose-200 flex items-center gap-1"
          >
            <AlertCircle class="w-3 h-3" />
            Terlambat
          </span>
        </div>

        <!-- Description -->
        <p v-if="version.description" class="text-xs text-slate-500 leading-relaxed max-w-2xl pl-10">
          {{ version.description }}
        </p>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center gap-1.5 shrink-0">
        <!-- Rilis Button (if UNRELEASED) -->
        <button
          v-if="version.status === 'UNRELEASED'"
          @click="emit('release', version)"
          class="flex items-center gap-1.5 px-3 py-1.5 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-xs font-semibold shadow-2xs transition"
          title="Rilis versi ini sekarang"
        >
          <CheckCircle2 class="w-3.5 h-3.5" />
          <span>Rilis Versi</span>
        </button>

        <!-- Archive Button -->
        <button
          @click="emit('archive', version)"
          class="p-1.5 text-slate-500 hover:text-slate-800 hover:bg-slate-100 rounded-lg transition"
          :title="version.status === 'ARCHIVED' ? 'Buka dari arsip' : 'Arsipkan versi'"
        >
          <Archive class="w-4 h-4" />
        </button>

        <!-- Edit Button -->
        <button
          @click="emit('edit', version)"
          class="p-1.5 text-slate-500 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition"
          title="Edit informasi versi"
        >
          <Edit2 class="w-4 h-4" />
        </button>

        <!-- Delete Button -->
        <button
          @click="emit('delete', version)"
          class="p-1.5 text-slate-500 hover:text-rose-600 hover:bg-rose-50 rounded-lg transition"
          title="Hapus versi ini"
        >
          <Trash2 class="w-4 h-4" />
        </button>
      </div>
    </div>

    <!-- Date Row -->
    <div class="flex items-center gap-4 text-xs text-slate-500 font-medium pl-10">
      <div v-if="formattedStartDate" class="flex items-center gap-1.5">
        <Calendar class="w-3.5 h-3.5 text-slate-400" />
        <span>Mulai: <strong class="text-slate-700">{{ formattedStartDate }}</strong></span>
      </div>
      <div class="flex items-center gap-1.5">
        <Clock class="w-3.5 h-3.5 text-slate-400" />
        <span>Rilis: <strong :class="isOverdue ? 'text-rose-600' : 'text-slate-700'">{{ formattedReleaseDate }}</strong></span>
      </div>
    </div>

    <!-- Progress Bar & Breakdown -->
    <div class="pt-2 border-t border-slate-100 pl-10">
      <ReleaseProgressBar
        :done-issues="Number(version.done_issues) || 0"
        :in-progress-issues="Number(version.in_progress_issues) || 0"
        :todo-issues="Number(version.todo_issues) || 0"
        :total-issues="Number(version.total_issues) || 0"
      />
    </div>
  </div>
</template>

