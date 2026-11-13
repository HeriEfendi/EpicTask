<script setup>
import { ref, computed, watch } from 'vue';
import { X, CheckCircle2, AlertTriangle, Calendar, ArrowRight } from 'lucide-vue-next';
import { useVersionStore } from '@/stores/version';
import { toast } from '@/utils/toast';

const props = defineProps({
  isOpen: {
    type: Boolean,
    default: false,
  },
  version: {
    type: Object,
    default: null,
  },
  availableVersions: {
    type: Array,
    default: () => [],
  },
});

const emit = defineEmits(['close', 'released']);
const versionStore = useVersionStore();

const releaseDate = ref('');
const moveToVersionId = ref('');
const isSubmitting = ref(false);

const unfinishedCount = computed(() => {
  if (!props.version) return 0;
  const todo = Number(props.version.todo_issues) || 0;
  const prog = Number(props.version.in_progress_issues) || 0;
  return todo + prog;
});

const otherUnreleasedVersions = computed(() => {
  if (!props.version) return [];
  return props.availableVersions.filter(
    (v) => v.id !== props.version.id && v.status === 'UNRELEASED'
  );
});

watch(
  () => props.isOpen,
  (val) => {
    if (val) {
      const today = new Date().toISOString().split('T')[0];
      releaseDate.value = props.version?.release_date || today;
      moveToVersionId.value = otherUnreleasedVersions.value[0]?.id || '';
    }
  }
);

async function handleRelease() {
  if (!props.version) return;
  isSubmitting.value = true;
  try {
    const payload = {
      release_date: releaseDate.value || null,
      move_unreleased_to_version_id:
        unfinishedCount.value > 0 && moveToVersionId.value
          ? Number(moveToVersionId.value)
          : null,
    };

    await versionStore.releaseVersion(props.version.id, payload);
    emit('released');
    emit('close');
  } catch (err) {
    // Handled in store toast
  } finally {
    isSubmitting.value = false;
  }
}
</script>

<template>
  <div
    v-if="isOpen && version"
    class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/50 backdrop-blur-xs p-4"
    @click.self="emit('close')"
  >
    <div class="bg-white rounded-2xl shadow-2xl max-w-md w-full overflow-hidden border border-slate-200">
      <!-- Header -->
      <div class="flex items-center justify-between px-6 py-4 border-b border-slate-100 bg-slate-50/60">
        <div class="flex items-center gap-2.5">
          <div class="w-8 h-8 rounded-lg bg-emerald-50 border border-emerald-100 flex items-center justify-center text-emerald-600">
            <CheckCircle2 class="w-4 h-4" />
          </div>
          <div>
            <h2 class="text-base font-bold text-slate-800">
              Rilis {{ version.name }}
            </h2>
            <p class="text-xs text-slate-500">Konfirmasi perilisan versi ini ke pengguna</p>
          </div>
        </div>
        <button
          @click="emit('close')"
          class="p-1 rounded-lg text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Body -->
      <form @submit.prevent="handleRelease" class="p-6 space-y-4">
        <!-- Release Date -->
        <div class="space-y-1.5">
          <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider flex items-center gap-1">
            <Calendar class="w-3.5 h-3.5 text-slate-400" />
            Tanggal Perilisan
          </label>
          <input
            v-model="releaseDate"
            type="date"
            required
            class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-800 focus:outline-none focus:border-emerald-600 shadow-2xs font-medium"
          />
        </div>

        <!-- Unfinished Issues Notice (Jira Standard) -->
        <div v-if="unfinishedCount > 0" class="p-4 rounded-xl bg-amber-50/70 border border-amber-200/80 space-y-3">
          <div class="flex items-start gap-2 text-amber-800">
            <AlertTriangle class="w-4 h-4 shrink-0 mt-0.5 text-amber-600" />
            <div class="text-xs leading-relaxed">
              Terdapat <strong>{{ unfinishedCount }} tiket</strong> dalam versi ini yang belum selesai (To Do / In Progress).
            </div>
          </div>

          <div class="space-y-1.5 pt-1">
            <label class="block text-xs font-bold text-amber-900 uppercase tracking-wider">
              Pindahkan tiket yang belum selesai ke:
            </label>
            <select
              v-model="moveToVersionId"
              class="w-full bg-white border border-amber-300 rounded-lg px-3 py-2 text-xs text-slate-800 focus:outline-none focus:border-emerald-600 shadow-2xs font-medium"
            >
              <option value="">- Lepaskan dari versi (Hapus Fix Version) -</option>
              <option
                v-for="target in otherUnreleasedVersions"
                :key="target.id"
                :value="target.id"
              >
                🚀 {{ target.name }}
              </option>
            </select>
          </div>
        </div>

        <div v-else class="p-3.5 rounded-xl bg-emerald-50/60 border border-emerald-200 text-xs text-emerald-800 flex items-center gap-2">
          <CheckCircle2 class="w-4 h-4 text-emerald-600 shrink-0" />
          <span>Semua tiket ({{ version.done_issues || 0 }}) telah selesai dikerjakan!</span>
        </div>

        <!-- Footer Buttons -->
        <div class="flex items-center justify-end gap-2.5 pt-4 border-t border-slate-100">
          <button
            type="button"
            @click="emit('close')"
            class="px-4 py-2 text-sm font-semibold text-slate-600 hover:text-slate-800 hover:bg-slate-100 rounded-lg transition"
          >
            Batal
          </button>
          <button
            type="submit"
            :disabled="isSubmitting"
            class="px-4 py-2 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-50 text-white text-sm font-semibold rounded-lg shadow-2xs transition flex items-center gap-1.5"
          >
            <CheckCircle2 class="w-4 h-4" />
            <span>Konfirmasi Rilis</span>
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

