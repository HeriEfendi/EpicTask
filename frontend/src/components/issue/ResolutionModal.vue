<script setup>
import { ref, watch, onMounted, onUnmounted } from 'vue';
import {
  CheckCircle2,
  X,
  Check,
  AlertTriangle,
  HelpCircle,
  Copy,
  Ban,
  ClipboardCheck
} from 'lucide-vue-next';

const props = defineProps({
  isOpen: {
    type: Boolean,
    default: false,
  },
  issue: {
    type: Object,
    default: null,
  },
  targetStatusName: {
    type: String,
    default: 'Done',
  },
});

const emit = defineEmits(['close', 'confirm']);

const RESOLUTION_OPTIONS = [
  {
    value: 'Fixed',
    label: 'Fixed (Terselesaikan)',
    desc: 'Pekerjaan selesai atau perbaikan bug telah diimplementasikan.',
    icon: CheckCircle2,
    badgeClass: 'bg-emerald-50 text-emerald-700 border-emerald-200',
    borderSelected: 'border-emerald-500 ring-1 ring-emerald-500 bg-emerald-50/30',
  },
  {
    value: 'Done',
    label: 'Done (Selesai Dikerjakan)',
    desc: 'Tugas telah rampung sesuai kriteria penerimaan kerja.',
    icon: ClipboardCheck,
    badgeClass: 'bg-blue-50 text-blue-700 border-blue-200',
    borderSelected: 'border-blue-500 ring-1 ring-blue-500 bg-blue-50/30',
  },
  {
    value: "Won't Fix",
    label: "Won't Fix (Tidak Dikerjakan)",
    desc: 'Diputuskan untuk tidak dikerjakan atau di luar ruang lingkup.',
    icon: Ban,
    badgeClass: 'bg-slate-100 text-slate-700 border-slate-300',
    borderSelected: 'border-slate-500 ring-1 ring-slate-500 bg-slate-50',
  },
  {
    value: 'Duplicate',
    label: 'Duplicate (Duplikat)',
    desc: 'Tiket ini merupakan duplikasi dari tiket lain yang sudah ada.',
    icon: Copy,
    badgeClass: 'bg-amber-50 text-amber-700 border-amber-200',
    borderSelected: 'border-amber-500 ring-1 ring-amber-500 bg-amber-50/30',
  },
  {
    value: 'Cannot Reproduce',
    label: 'Cannot Reproduce (Tidak Replikasi)',
    desc: 'Kendala tidak dapat direplikasi setelah pengujian teknis.',
    icon: HelpCircle,
    badgeClass: 'bg-rose-50 text-rose-700 border-rose-200',
    borderSelected: 'border-rose-500 ring-1 ring-rose-500 bg-rose-50/30',
  },
  {
    value: 'Incomplete',
    label: 'Incomplete (Tidak Lengkap)',
    desc: 'Informasi atau prasyarat tiket tidak memadai untuk diselesaikan.',
    icon: AlertTriangle,
    badgeClass: 'bg-orange-50 text-orange-700 border-orange-200',
    borderSelected: 'border-orange-500 ring-1 ring-orange-500 bg-orange-50/30',
  },
];

const selectedResolution = ref('Fixed');

watch(
  () => props.issue,
  (newIssue) => {
    if (newIssue) {
      if (newIssue.resolution) {
        selectedResolution.value = newIssue.resolution;
      } else if (newIssue.issue_type === 'BUG') {
        selectedResolution.value = 'Fixed';
      } else {
        selectedResolution.value = 'Done';
      }
    }
  },
  { immediate: true }
);

function handleConfirm() {
  emit('confirm', selectedResolution.value);
}

function handleKeydown(e) {
  if (!props.isOpen) return;
  if (e.key === 'Escape') {
    emit('close');
  } else if (e.key === 'Enter' && !e.shiftKey) {
    handleConfirm();
  }
}

onMounted(() => window.addEventListener('keydown', handleKeydown));
onUnmounted(() => window.removeEventListener('keydown', handleKeydown));
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/50 backdrop-blur-xs transition-opacity"
    @click.self="emit('close')"
  >
    <div
      class="bg-white rounded-2xl shadow-2xl border border-slate-200 w-full max-w-lg overflow-hidden flex flex-col animate-in fade-in zoom-in-95 duration-150"
    >
      <!-- Header -->
      <div class="px-6 py-4.5 border-b border-slate-100 flex items-center justify-between bg-slate-50/50">
        <div class="flex items-center space-x-2.5">
          <div class="w-8 h-8 rounded-lg bg-emerald-100 flex items-center justify-center text-emerald-600">
            <CheckCircle2 class="w-5 h-5" />
          </div>
          <div>
            <h3 class="text-base font-bold text-slate-800">
              Selesaikan Tiket (Resolve Issue)
            </h3>
            <p class="text-xs text-slate-500">
              Target status: <span class="font-semibold text-emerald-700">{{ targetStatusName }}</span>
              <span v-if="issue" class="ml-1 font-mono font-bold text-slate-600">({{ issue.key }})</span>
            </p>
          </div>
        </div>
        <button
          @click="emit('close')"
          class="text-slate-400 hover:text-slate-600 p-1 rounded-lg hover:bg-slate-100 transition"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Body: Resolution Cards -->
      <div class="p-6 space-y-4">
        <div>
          <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider mb-1.5">
            Pilih Alasan Resolusi (Resolution)
          </label>
          <p class="text-xs text-slate-500 mb-3">
            Standar audit mutu software Jira untuk mencatat alasan mengapa tiket ini ditutup.
          </p>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5 max-h-[320px] overflow-y-auto pr-1">
            <div
              v-for="opt in RESOLUTION_OPTIONS"
              :key="opt.value"
              @click="selectedResolution = opt.value"
              class="border rounded-xl p-3 cursor-pointer transition flex items-start space-x-2.5 select-none"
              :class="[
                selectedResolution === opt.value
                  ? opt.borderSelected
                  : 'border-slate-200 hover:border-slate-300 hover:bg-slate-50/60'
              ]"
            >
              <div class="mt-0.5 shrink-0">
                <component
                  :is="opt.icon"
                  class="w-4 h-4"
                  :class="selectedResolution === opt.value ? 'text-emerald-600' : 'text-slate-400'"
                />
              </div>
              <div class="flex-1 min-w-0">
                <div class="flex items-center justify-between">
                  <span class="text-xs font-bold text-slate-800 truncate">
                    {{ opt.value }}
                  </span>
                  <span
                    v-if="selectedResolution === opt.value"
                    class="w-3.5 h-3.5 rounded-full bg-emerald-500 text-white flex items-center justify-center text-[9px] shrink-0"
                  >
                    <Check class="w-2.5 h-2.5 stroke-[3]" />
                  </span>
                </div>
                <p class="text-[11px] text-slate-500 leading-tight mt-0.5 line-clamp-2">
                  {{ opt.desc }}
                </p>
              </div>
            </div>
          </div>
        </div>

        <!-- Issue Summary Preview -->
        <div v-if="issue" class="p-3 bg-slate-50 rounded-xl border border-slate-200 text-xs text-slate-600">
          <span class="font-bold text-slate-700">Ringkasan:</span> {{ issue.summary }}
        </div>
      </div>

      <!-- Footer Buttons -->
      <div class="px-6 py-4 border-t border-slate-100 bg-slate-50 flex items-center justify-end space-x-3">
        <button
          type="button"
          @click="emit('close')"
          class="px-4 py-2 text-xs font-semibold text-slate-600 hover:text-slate-800 hover:bg-slate-200/60 rounded-xl transition"
        >
          Batal
        </button>
        <button
          type="button"
          @click="handleConfirm"
          class="px-5 py-2 text-xs font-bold text-white bg-emerald-600 hover:bg-emerald-700 active:bg-emerald-800 rounded-xl shadow-xs transition flex items-center space-x-1.5"
        >
          <CheckCircle2 class="w-4 h-4" />
          <span>Selesaikan Tiket</span>
        </button>
      </div>
    </div>
  </div>
</template>
