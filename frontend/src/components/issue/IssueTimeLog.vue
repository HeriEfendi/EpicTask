<script setup>
import { ref } from 'vue';

const props = defineProps({
  timeLogs: {
    type: Array,
    default: () => []
  },
  totalTimeSpentSeconds: {
    type: Number,
    default: 0
  }
});

const emit = defineEmits(['log-work']);

const isLoggingWork = ref(false);
const logHours = ref(1);
const logDescription = ref('');

function handleSubmit() {
  if (logHours.value <= 0) return;
  emit('log-work', {
    hours: logHours.value,
    description: logDescription.value.trim()
  });
  isLoggingWork.value = false;
  logHours.value = 1;
  logDescription.value = '';
}
</script>

<template>
  <div class="space-y-3.5">
    <div class="flex items-center justify-between p-3.5 bg-slate-50 rounded-xl border border-slate-200">
      <div>
        <span class="text-sm text-slate-600">Total Waktu Dikerjakan: </span>
        <span class="text-base font-bold font-mono text-emerald-600">
          {{ (totalTimeSpentSeconds / 3600).toFixed(1) }} jam
        </span>
      </div>
      <button
        @click="isLoggingWork = !isLoggingWork"
        class="px-3 py-1.5 bg-white hover:bg-emerald-50 border border-emerald-500 text-emerald-700 rounded-lg text-sm font-semibold shadow-2xs transition"
      >
        + Catat Waktu
      </button>
    </div>

    <!-- Log Work Form -->
    <div v-if="isLoggingWork" class="p-3.5 bg-white border border-emerald-400 rounded-xl space-y-3 shadow-xs">
      <div class="flex items-center space-x-3">
        <div class="w-36">
          <label class="block text-xs font-semibold text-slate-600 mb-1">Durasi (Jam)</label>
          <input
            v-model.number="logHours"
            type="number"
            step="0.25"
            min="0.1"
            class="w-full bg-slate-50 border border-slate-300 rounded-lg px-2.5 py-1.5 text-sm text-slate-900 shadow-2xs font-semibold"
          />
        </div>
        <div class="flex-1">
          <label class="block text-xs font-semibold text-slate-600 mb-1">Deskripsi Pengerjaan</label>
          <input
            v-model="logDescription"
            placeholder="Pekerjaan apa yang sudah diselesaikan?..."
            class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-1.5 text-sm text-slate-900 shadow-2xs"
          />
        </div>
      </div>
      <div class="flex justify-end space-x-2">
        <button
          @click="isLoggingWork = false"
          class="px-3 py-1.5 text-sm font-medium text-slate-600 hover:text-slate-900"
        >
          Batal
        </button>
        <button
          @click="handleSubmit"
          class="px-4 py-1.5 bg-emerald-600 hover:bg-emerald-700 active:bg-emerald-800 text-white rounded-lg text-sm font-semibold shadow-xs"
        >
          Simpan Catatan
        </button>
      </div>
    </div>

    <!-- Logs History -->
    <div class="space-y-2 max-h-48 overflow-y-auto">
      <div
        v-for="tl in timeLogs"
        :key="tl.id"
        class="p-3 bg-white border border-slate-200 rounded-lg flex items-center justify-between text-sm shadow-2xs"
      >
        <div>
          <span class="font-bold text-slate-800">{{ tl.user_name || 'Anggota' }}: </span>
          <span class="text-slate-600">{{ tl.description || 'Mengerjakan tiket' }}</span>
        </div>
        <span class="font-mono font-bold text-emerald-600">
          {{ (tl.time_spent_seconds / 3600).toFixed(1) }}h
        </span>
      </div>
    </div>
  </div>
</template>

