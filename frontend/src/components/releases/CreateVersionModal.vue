<script setup>
import { ref, watch } from 'vue';
import { X, Rocket, Calendar, FileText } from 'lucide-vue-next';
import { useVersionStore } from '@/stores/version';
import { toast } from '@/utils/toast';

const props = defineProps({
  isOpen: {
    type: Boolean,
    default: false,
  },
  versionToEdit: {
    type: Object,
    default: null,
  },
  projectId: {
    type: Number,
    required: true,
  },
});

const emit = defineEmits(['close', 'saved']);
const versionStore = useVersionStore();

const name = ref('');
const description = ref('');
const startDate = ref('');
const releaseDate = ref('');
const isSubmitting = ref(false);

watch(
  () => props.isOpen,
  (val) => {
    if (val) {
      if (props.versionToEdit) {
        name.value = props.versionToEdit.name || '';
        description.value = props.versionToEdit.description || '';
        startDate.value = props.versionToEdit.start_date || '';
        releaseDate.value = props.versionToEdit.release_date || '';
      } else {
        name.value = '';
        description.value = '';
        startDate.value = '';
        releaseDate.value = '';
      }
    }
  }
);

async function handleSubmit() {
  if (!name.value.trim()) {
    toast.warning('Nama versi rilis wajib diisi');
    return;
  }

  isSubmitting.value = true;
  try {
    const payload = {
      name: name.value.trim(),
      description: description.value.trim() || null,
      start_date: startDate.value || null,
      release_date: releaseDate.value || null,
    };

    if (props.versionToEdit) {
      await versionStore.updateVersion(props.versionToEdit.id, payload);
    } else {
      await versionStore.createVersion(props.projectId, payload);
    }
    emit('saved');
    emit('close');
  } catch (err) {
    // Error is toasted in store
  } finally {
    isSubmitting.value = false;
  }
}
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/50 backdrop-blur-xs p-4"
    @click.self="emit('close')"
  >
    <div class="bg-white rounded-2xl shadow-2xl max-w-lg w-full overflow-hidden border border-slate-200">
      <!-- Header -->
      <div class="flex items-center justify-between px-6 py-4 border-b border-slate-100 bg-slate-50/60">
        <div class="flex items-center gap-2.5">
          <div class="w-8 h-8 rounded-lg bg-indigo-50 border border-indigo-100 flex items-center justify-center text-indigo-600">
            <Rocket class="w-4 h-4" />
          </div>
          <h2 class="text-base font-bold text-slate-800">
            {{ versionToEdit ? 'Edit Versi Rilis' : 'Buat Versi Rilis Baru' }}
          </h2>
        </div>
        <button
          @click="emit('close')"
          class="p-1 rounded-lg text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Form Body -->
      <form @submit.prevent="handleSubmit" class="p-6 space-y-4">
        <!-- Name -->
        <div class="space-y-1.5">
          <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider">
            Nama Versi <span class="text-rose-500">*</span>
          </label>
          <input
            v-model="name"
            type="text"
            required
            placeholder="Contoh: v1.0.0, Sprint 24 Release, Beta-2"
            class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 placeholder-slate-400 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs font-semibold"
          />
        </div>

        <!-- Dates Grid -->
        <div class="grid grid-cols-2 gap-3">
          <!-- Start Date -->
          <div class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider flex items-center gap-1">
              <Calendar class="w-3.5 h-3.5 text-slate-400" />
              Tanggal Mulai
            </label>
            <input
              v-model="startDate"
              type="date"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-800 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
            />
          </div>

          <!-- Release Date -->
          <div class="space-y-1.5">
            <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider flex items-center gap-1">
              <Calendar class="w-3.5 h-3.5 text-slate-400" />
              Target Rilis
            </label>
            <input
              v-model="releaseDate"
              type="date"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-800 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
            />
          </div>
        </div>

        <!-- Description -->
        <div class="space-y-1.5">
          <label class="block text-xs font-bold text-slate-700 uppercase tracking-wider flex items-center gap-1">
            <FileText class="w-3.5 h-3.5 text-slate-400" />
            Deskripsi / Catatan Rilis
          </label>
          <textarea
            v-model="description"
            rows="3"
            placeholder="Tujuan utama rilis, cakupan fitur, atau milestone..."
            class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-800 placeholder-slate-400 focus:outline-none focus:border-blue-600 shadow-2xs leading-relaxed"
          ></textarea>
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
            :disabled="isSubmitting || !name.trim()"
            class="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white text-sm font-semibold rounded-lg shadow-2xs transition flex items-center gap-1.5"
          >
            <Rocket class="w-4 h-4" />
            <span>{{ versionToEdit ? 'Simpan Perubahan' : 'Buat Versi' }}</span>
          </button>
        </div>
      </form>
    </div>
  </div>
</template>
