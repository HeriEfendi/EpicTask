<script setup>
import { ref, computed, watch } from 'vue';
import { X, CheckCircle, AlertTriangle, ArrowRight, Package } from 'lucide-vue-next';
import { useSprintStore } from '@/stores/sprint';
import { toast } from '@/utils/toast';

const props = defineProps({
  isOpen: {
    type: Boolean,
    default: false,
  },
  sprint: {
    type: Object,
    default: null,
  },
  issues: {
    type: Array,
    default: () => [],
  },
});

const emit = defineEmits(['close', 'completed']);

const sprintStore = useSprintStore();

const moveTarget = ref('backlog'); // 'backlog' or sprintId
const isSubmitting = ref(false);

const completedIssues = computed(() => {
  return props.issues.filter((i) => i.status_category === 'DONE');
});

const incompleteIssues = computed(() => {
  return props.issues.filter((i) => i.status_category !== 'DONE');
});

const otherPlannedSprints = computed(() => {
  if (!props.sprint) return [];
  return sprintStore.sprints.filter(
    (s) => s.status === 'PLANNED' && s.id !== props.sprint.id
  );
});

watch(
  () => props.isOpen,
  (open) => {
    if (open) {
      moveTarget.value = otherPlannedSprints.value.length > 0 ? String(otherPlannedSprints.value[0].id) : 'backlog';
      isSubmitting.value = false;
    }
  }
);

async function handleComplete() {
  isSubmitting.value = true;
  try {
    const targetSprintId = moveTarget.value === 'backlog' ? null : Number(moveTarget.value);
    await sprintStore.completeSprint(props.sprint.id, {
      move_incomplete_to_sprint_id: targetSprintId,
    });
    emit('completed');
    emit('close');
  } catch (err) {
    // Error handled in store
  } finally {
    isSubmitting.value = false;
  }
}
</script>

<template>
  <div
    v-if="isOpen && sprint"
    class="fixed inset-0 z-50 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center p-4"
  >
    <div class="bg-white rounded-2xl border border-slate-200 shadow-2xl w-full max-w-md overflow-hidden animate-slide-up">
      <!-- Header -->
      <div class="px-5 py-4 border-b border-slate-100 flex items-center justify-between">
        <div class="flex items-center space-x-2">
          <div class="w-8 h-8 rounded-lg bg-emerald-50 border border-emerald-200 flex items-center justify-center">
            <CheckCircle class="w-4 h-4 text-emerald-600" />
          </div>
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Selesaikan {{ sprint.name }}</h3>
            <p class="text-xs text-slate-500">Tutup iterasi kerja dan arsipkan hasil pencapaian</p>
          </div>
        </div>

        <button
          @click="emit('close')"
          class="text-slate-400 hover:text-slate-700 p-1 rounded-lg transition"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <!-- Body / Stats -->
      <div class="p-5 space-y-4 text-sm">
        <!-- Summary Stats Cards -->
        <div class="grid grid-cols-2 gap-3">
          <div class="p-3 bg-emerald-50 border border-emerald-200 rounded-xl">
            <p class="text-xs font-semibold text-emerald-700">Isu Selesai</p>
            <p class="text-2xl font-bold font-mono text-emerald-800 mt-1">
              {{ completedIssues.length }}
            </p>
          </div>

          <div class="p-3 bg-amber-50 border border-amber-200 rounded-xl">
            <p class="text-xs font-semibold text-amber-700">Isu Belum Selesai</p>
            <p class="text-2xl font-bold font-mono text-amber-800 mt-1">
              {{ incompleteIssues.length }}
            </p>
          </div>
        </div>

        <!-- Incomplete Issues Action (if any) -->
        <div v-if="incompleteIssues.length > 0" class="space-y-2 pt-2 border-t border-slate-100">
          <div class="flex items-center space-x-1.5 text-xs font-bold text-slate-700 uppercase tracking-wider">
            <AlertTriangle class="w-3.5 h-3.5 text-amber-500" />
            <span>Tindakan untuk {{ incompleteIssues.length }} Isu Belum Selesai</span>
          </div>

          <label class="block text-xs text-slate-500">
            Pindahkan seluruh tiket yang belum terselesaikan ke:
          </label>

          <select
            v-model="moveTarget"
            class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs font-medium cursor-pointer"
          >
            <option value="backlog">📦 Product Backlog</option>
            <option
              v-for="s in otherPlannedSprints"
              :key="s.id"
              :value="String(s.id)"
            >
              🚀 {{ s.name }} (Sprint Mendatang)
            </option>
          </select>
        </div>

        <div v-else class="text-xs text-emerald-700 bg-emerald-50 border border-emerald-200 rounded-lg p-2.5">
          🎉 Luar biasa! Seluruh {{ issues.length }} isu dalam sprint ini telah berhasil diselesaikan.
        </div>
      </div>

      <!-- Footer Buttons -->
      <div class="px-5 py-3 bg-slate-50 border-t border-slate-100 flex items-center justify-end space-x-2">
        <button
          @click="emit('close')"
          type="button"
          class="px-3.5 py-1.5 text-xs font-semibold text-slate-600 hover:text-slate-800 rounded-lg transition"
        >
          Batal
        </button>

        <button
          @click="handleComplete"
          :disabled="isSubmitting"
          type="button"
          class="px-4 py-1.5 bg-emerald-600 hover:bg-emerald-700 active:bg-emerald-800 disabled:opacity-50 text-white rounded-lg text-xs font-bold transition shadow-xs flex items-center space-x-1.5"
        >
          <CheckCircle class="w-3.5 h-3.5" />
          <span>{{ isSubmitting ? 'Menyelesaikan...' : 'Selesaikan Sprint' }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

