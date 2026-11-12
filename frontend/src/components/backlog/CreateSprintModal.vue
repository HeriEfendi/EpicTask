<script setup>
import { ref, watch } from 'vue';
import { X, Play, Target, Calendar } from 'lucide-vue-next';
import { useSprintStore } from '@/stores/sprint';
import { toast } from '@/utils/toast';

const props = defineProps({
  isOpen: {
    type: Boolean,
    default: false,
  },
  projectId: {
    type: Number,
    required: true,
  },
  projectKey: {
    type: String,
    default: 'PROJ',
  },
  existingSprintCount: {
    type: Number,
    default: 0,
  },
});

const emit = defineEmits(['close', 'created']);

const sprintStore = useSprintStore();

const sprintName = ref('');
const sprintGoal = ref('');
const startDate = ref('');
const endDate = ref('');
const isSubmitting = ref(false);

watch(
  () => props.isOpen,
  (open) => {
    if (open) {
      sprintName.value = `${props.projectKey} Sprint ${props.existingSprintCount + 1}`;
      sprintGoal.value = '';
      startDate.value = '';
      endDate.value = '';
      isSubmitting.value = false;
    }
  }
);

async function handleCreate() {
  if (!sprintName.value.trim()) {
    toast.warning('Nama sprint wajib diisi');
    return;
  }

  isSubmitting.value = true;
  try {
    await sprintStore.createSprint(props.projectId, {
      name: sprintName.value.trim(),
      goal: sprintGoal.value.trim() || null,
      start_date: startDate.value || null,
      end_date: endDate.value || null,
    });
    emit('created');
    emit('close');
  } catch (err) {
    // Toast handled in store
  } finally {
    isSubmitting.value = false;
  }
}
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 z-50 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center p-4"
  >
    <div class="bg-white rounded-2xl border border-slate-200 shadow-2xl w-full max-w-md overflow-hidden animate-slide-up">
      <!-- Header -->
      <div class="px-5 py-4 border-b border-slate-100 flex items-center justify-between">
        <div class="flex items-center space-x-2">
          <div class="w-8 h-8 rounded-lg bg-blue-50 border border-blue-200 flex items-center justify-center">
            <Play class="w-4 h-4 text-blue-600 fill-current" />
          </div>
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Buat Sprint Baru</h3>
            <p class="text-xs text-slate-500">Rencanakan iterasi pekerjaan tim Anda</p>
          </div>
        </div>

        <button
          @click="emit('close')"
          class="text-slate-400 hover:text-slate-700 p-1 rounded-lg transition"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <!-- Form Body -->
      <div class="p-5 space-y-4">
        <!-- Sprint Name -->
        <div class="space-y-1">
          <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider">
            Nama Sprint <span class="text-red-500">*</span>
          </label>
          <input
            v-model="sprintName"
            type="text"
            placeholder="Contoh: EPIC Sprint 1"
            class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs font-medium"
            @keyup.enter="handleCreate"
          />
        </div>

        <!-- Sprint Goal -->
        <div class="space-y-1">
          <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider flex items-center gap-1">
            <Target class="w-3 h-3 text-slate-500" />
            <span>Target / Sprint Goal</span>
          </label>
          <textarea
            v-model="sprintGoal"
            rows="2"
            placeholder="Tujuan utama yang ingin dicapai pada sprint ini..."
            class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs"
          ></textarea>
        </div>

        <!-- Dates -->
        <div class="grid grid-cols-2 gap-3">
          <div class="space-y-1">
            <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider flex items-center gap-1">
              <Calendar class="w-3 h-3 text-slate-500" />
              <span>Tanggal Mulai</span>
            </label>
            <input
              v-model="startDate"
              type="date"
              class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs"
            />
          </div>

          <div class="space-y-1">
            <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider flex items-center gap-1">
              <Calendar class="w-3 h-3 text-slate-500" />
              <span>Tanggal Selesai</span>
            </label>
            <input
              v-model="endDate"
              type="date"
              class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs"
            />
          </div>
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
          @click="handleCreate"
          :disabled="isSubmitting"
          type="button"
          class="px-4 py-1.5 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 disabled:opacity-50 text-white rounded-lg text-xs font-bold transition shadow-xs"
        >
          {{ isSubmitting ? 'Menyimpan...' : 'Buat Sprint' }}
        </button>
      </div>
    </div>
  </div>
</template>

