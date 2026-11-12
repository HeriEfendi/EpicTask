<script setup>
import { ref, watch } from 'vue';
import { X, Play, Target, Calendar, Clock, AlertCircle } from 'lucide-vue-next';
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
  issueCount: {
    type: Number,
    default: 0,
  },
});

const emit = defineEmits(['close', 'started']);

const sprintStore = useSprintStore();

const sprintName = ref('');
const durationDays = ref(14); // 2 weeks default
const startDate = ref('');
const endDate = ref('');
const sprintGoal = ref('');
const isSubmitting = ref(false);

function formatDateToIso(d) {
  const year = d.getFullYear();
  const month = String(d.getMonth() + 1).padStart(2, '0');
  const day = String(d.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}

function calculateEndDate(startStr, days) {
  if (!startStr) return '';
  const d = new Date(startStr);
  d.setDate(d.getDate() + Number(days));
  return formatDateToIso(d);
}

watch(
  () => props.isOpen,
  (open) => {
    if (open && props.sprint) {
      sprintName.value = props.sprint.name || '';
      sprintGoal.value = props.sprint.goal || '';
      const today = new Date();
      startDate.value = props.sprint.start_date || formatDateToIso(today);
      durationDays.value = 14;
      endDate.value = props.sprint.end_date || calculateEndDate(startDate.value, 14);
      isSubmitting.value = false;
    }
  }
);

function handleDurationChange() {
  if (durationDays.value !== 'custom') {
    endDate.value = calculateEndDate(startDate.value, durationDays.value);
  }
}

function handleStartDateChange() {
  if (durationDays.value !== 'custom') {
    endDate.value = calculateEndDate(startDate.value, durationDays.value);
  }
}

async function handleStart() {
  if (!sprintName.value.trim()) {
    toast.warning('Nama sprint wajib diisi');
    return;
  }
  if (!startDate.value || !endDate.value) {
    toast.warning('Tanggal mulai dan selesai wajib diisi');
    return;
  }

  isSubmitting.value = true;
  try {
    await sprintStore.startSprint(props.sprint.id, {
      name: sprintName.value.trim(),
      goal: sprintGoal.value.trim() || null,
      start_date: startDate.value,
      end_date: endDate.value,
    });
    emit('started');
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
    v-if="isOpen && sprint"
    class="fixed inset-0 z-50 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center p-4"
  >
    <div class="bg-white rounded-2xl border border-slate-200 shadow-2xl w-full max-w-md overflow-hidden animate-slide-up">
      <!-- Header -->
      <div class="px-5 py-4 border-b border-slate-100 flex items-center justify-between">
        <div class="flex items-center space-x-2">
          <div class="w-8 h-8 rounded-lg bg-emerald-50 border border-emerald-200 flex items-center justify-center">
            <Play class="w-4 h-4 text-emerald-600 fill-current" />
          </div>
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Mulai Sprint</h3>
            <p class="text-xs text-slate-500">{{ issueCount }} tiket akan diaktifkan di papan kerja</p>
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
      <div class="p-5 space-y-4 text-sm">
        <!-- Sprint Name -->
        <div class="space-y-1">
          <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider">
            Nama Sprint <span class="text-red-500">*</span>
          </label>
          <input
            v-model="sprintName"
            type="text"
            class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
          />
        </div>

        <!-- Duration preset -->
        <div class="space-y-1">
          <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider flex items-center gap-1">
            <Clock class="w-3 h-3 text-slate-500" />
            <span>Durasi Sprint</span>
          </label>
          <select
            v-model="durationDays"
            @change="handleDurationChange"
            class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs font-medium cursor-pointer"
          >
            <option :value="7">1 Minggu</option>
            <option :value="14">2 Minggu (Standar Scrum)</option>
            <option :value="21">3 Minggu</option>
            <option :value="28">4 Minggu</option>
            <option value="custom">Kustom</option>
          </select>
        </div>

        <!-- Start Date & End Date -->
        <div class="grid grid-cols-2 gap-3">
          <div class="space-y-1">
            <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider flex items-center gap-1">
              <Calendar class="w-3 h-3 text-slate-500" />
              <span>Tanggal Mulai</span>
            </label>
            <input
              v-model="startDate"
              type="date"
              @change="handleStartDateChange"
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
              :disabled="durationDays !== 'custom'"
              class="w-full bg-white border border-slate-300 disabled:bg-slate-50 disabled:text-slate-500 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs"
            />
          </div>
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
            placeholder="Keluaran atau milestone yang ingin diselesaikan tim dalam sprint ini..."
            class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs"
          ></textarea>
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
          @click="handleStart"
          :disabled="isSubmitting"
          type="button"
          class="px-4 py-1.5 bg-emerald-600 hover:bg-emerald-700 active:bg-emerald-800 disabled:opacity-50 text-white rounded-lg text-xs font-bold transition shadow-xs flex items-center space-x-1.5"
        >
          <Play class="w-3 h-3 fill-current" />
          <span>{{ isSubmitting ? 'Memulai...' : 'Mulai Sprint' }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

