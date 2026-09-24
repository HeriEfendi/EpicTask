<script setup>
import { ref, onMounted } from 'vue';
import { useAutomationStore } from '@/stores/automation';
import { useProjectStore } from '@/stores/project';
import {
  Bot,
  Zap,
  ArrowRight,
  Plus,
  Trash2,
  Sparkles,
  ToggleLeft,
  ToggleRight,
  X
} from 'lucide-vue-next';

const autoStore = useAutomationStore();
const projectStore = useProjectStore();

const isCreating = ref(false);
const ruleName = ref('');
const selectedTrigger = ref('STATUS_CHANGED');
const selectedAction = ref('CASCADE_SUBTASKS_DONE');

onMounted(() => {
  if (projectStore.currentProject) {
    autoStore.fetchRules(projectStore.currentProject.id);
  }
});

async function handleToggle(rule) {
  try {
    await autoStore.toggleRule(
      projectStore.currentProject.id,
      rule.id,
      !rule.is_active
    );
  } catch (e) {
    alert(e.message || 'Gagal memperbarui aturan');
  }
}

async function handleDelete(ruleId) {
  if (confirm('Hapus aturan otomasi ini?')) {
    await autoStore.deleteRule(projectStore.currentProject.id, ruleId);
  }
}

async function handleCreateRule() {
  if (!ruleName.value.trim()) return;

  const triggerConfig = selectedTrigger.value === 'STATUS_CHANGED'
    ? { category: 'DONE' }
    : { hours: 24 };

  try {
    await autoStore.createRule(projectStore.currentProject.id, {
      name: ruleName.value.trim(),
      trigger_type: selectedTrigger.value,
      trigger_config: triggerConfig,
      action_type: selectedAction.value,
      action_config: {},
      is_active: true,
    });
    ruleName.value = '';
    isCreating.value = false;
  } catch (e) {
    alert(e.message || 'Gagal membuat aturan otomasi');
  }
}

function applyTemplate(name, trigger, action) {
  ruleName.value = name;
  selectedTrigger.value = trigger;
  selectedAction.value = action;
  isCreating.value = true;
}
</script>

<template>
  <div
    v-if="autoStore.isRuleModalOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/40 backdrop-blur-sm"
  >
    <div class="glass-modal w-full max-w-2xl rounded-2xl p-6 border border-slate-200 bg-white max-h-[90vh] flex flex-col shadow-2xl animate-slide-up">
      <!-- Header -->
      <div class="flex items-center justify-between pb-4 border-b border-slate-200">
        <div class="flex items-center space-x-3">
          <div class="w-9 h-9 rounded-xl bg-purple-50 border border-purple-200 flex items-center justify-center text-purple-600">
            <Bot class="w-5 h-5" />
          </div>
          <div>
            <h2 class="text-lg font-bold text-slate-900">No-Code Automation Engine</h2>
            <p class="text-sm text-slate-500">Bangun aturan IF-THEN otomatis untuk mempercepat alur kerja</p>
          </div>
        </div>

        <button
          @click="autoStore.isRuleModalOpen = false"
          class="p-1.5 rounded-lg text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Main Body -->
      <div class="flex-1 overflow-y-auto py-4 space-y-4 text-sm">
        <!-- Pre-made Templates -->
        <div class="bg-gradient-to-r from-purple-50 to-indigo-50 border border-purple-200 rounded-xl p-4">
          <div class="flex items-center space-x-2 text-sm font-bold text-purple-900 mb-3">
            <Sparkles class="w-4 h-4 text-purple-600" />
            <span>Rekomendasi Template Otomasi</span>
          </div>
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
            <button
              @click="applyTemplate('Otomatis Tutup Subtask', 'STATUS_CHANGED', 'CASCADE_SUBTASKS_DONE')"
              class="text-left p-3 bg-white hover:bg-purple-50/80 border border-purple-100 rounded-xl text-sm transition shadow-2xs"
            >
              <p class="font-bold text-slate-900">Selesaikan Subtask Otomatis</p>
              <p class="text-xs text-slate-500 mt-1">Saat tiket induk ditandai Selesai (Done), seluruh subtask otomatis ditutup</p>
            </button>

            <button
              @click="applyTemplate('Tugaskan Balik ke Pembuat Saat Selesai', 'STATUS_CHANGED', 'ASSIGN_TO_REPORTER')"
              class="text-left p-3 bg-white hover:bg-purple-50/80 border border-purple-100 rounded-xl text-sm transition shadow-2xs"
            >
              <p class="font-bold text-slate-900">Serahkan Kembali ke Reporter</p>
              <p class="text-xs text-slate-500 mt-1">Saat tiket selesai dikerjakan, tugaskan kembali ke pembuat awal</p>
            </button>
          </div>
        </div>

        <!-- Rule Builder Form (IF-THEN) -->
        <div v-if="isCreating" class="bg-slate-50 border border-blue-200 rounded-xl p-4 space-y-4 animate-slide-up">
          <div class="flex items-center justify-between">
            <h3 class="text-sm font-bold text-blue-700 uppercase tracking-wider flex items-center gap-1.5">
              <Zap class="w-4 h-4" /> Rule Builder (JIKA - MAKA)
            </h3>
            <button @click="isCreating = false" class="text-sm font-medium text-slate-500 hover:text-slate-800">Batal</button>
          </div>

          <div>
            <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider mb-1.5">Nama Aturan</label>
            <input
              v-model="ruleName"
              placeholder="e.g. Tutup Subtask Otomatis Saat Epic Selesai"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-2 text-sm text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
            />
          </div>

          <!-- Visual IF block -->
          <div class="p-3.5 bg-white rounded-xl border border-slate-200 flex items-center space-x-3.5 shadow-2xs">
            <span class="px-2.5 py-1 rounded-md font-bold text-xs bg-amber-50 text-amber-700 border border-amber-200 shrink-0">
              JIKA (TRIGGER)
            </span>
            <div class="flex-1">
              <select
                v-model="selectedTrigger"
                class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-1.5 text-sm text-slate-800 focus:outline-none focus:border-blue-600 font-medium"
              >
                <option value="STATUS_CHANGED">Saat status tiket berubah menjadi "Done"</option>
                <option value="DUE_DATE_NEAR">Saat batas tenggat waktu mendekati 24 jam</option>
              </select>
            </div>
          </div>

          <!-- Arrow connector -->
          <div class="flex justify-center text-slate-400">
            <ArrowRight class="w-4 h-4 rotate-90" />
          </div>

          <!-- Visual THEN block -->
          <div class="p-3.5 bg-white rounded-xl border border-slate-200 flex items-center space-x-3.5 shadow-2xs">
            <span class="px-2.5 py-1 rounded-md font-bold text-xs bg-emerald-50 text-emerald-700 border border-emerald-200 shrink-0">
              MAKA (AKSI)
            </span>
            <div class="flex-1">
              <select
                v-model="selectedAction"
                class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-1.5 text-sm text-slate-800 focus:outline-none focus:border-blue-600 font-medium"
              >
                <option value="CASCADE_SUBTASKS_DONE">Otomatis setel semua subtask anak menjadi Done</option>
                <option value="ASSIGN_TO_REPORTER">Tugaskan tiket kembali ke Reporter awal</option>
              </select>
            </div>
          </div>

          <div class="flex justify-end space-x-2 pt-2">
            <button
              @click="isCreating = false"
              class="px-3.5 py-1.5 rounded-lg text-sm font-medium text-slate-600 hover:text-slate-800 hover:bg-slate-200/50"
            >
              Batal
            </button>
            <button
              @click="handleCreateRule"
              class="px-5 py-1.5 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-lg text-sm font-semibold shadow-xs"
            >
              Simpan Aturan
            </button>
          </div>
        </div>

        <!-- Create Button -->
        <div v-else class="flex justify-end">
          <button
            @click="isCreating = true"
            class="flex items-center space-x-1.5 px-4 py-2 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-lg text-sm font-semibold shadow-xs transition"
          >
            <Plus class="w-4 h-4" />
            <span>Buat Aturan Kustom</span>
          </button>
        </div>

        <!-- Existing Active Rules List -->
        <div class="space-y-2.5">
          <h3 class="text-xs font-bold text-slate-500 uppercase tracking-wider">Aturan Aktif di Projek Ini</h3>
          <div
            v-for="rule in autoStore.rules"
            :key="rule.id"
            class="bg-white border border-slate-200 rounded-xl p-4 flex items-center justify-between transition hover:border-slate-300 shadow-2xs"
          >
            <div class="flex items-start space-x-3.5">
              <button
                @click="handleToggle(rule)"
                class="mt-0.5 text-slate-400 hover:text-slate-700 transition"
                :title="rule.is_active ? 'Klik untuk menonaktifkan' : 'Klik untuk mengaktifkan'"
              >
                <ToggleRight v-if="rule.is_active" class="w-7 h-7 text-emerald-600" />
                <ToggleLeft v-else class="w-7 h-7 text-slate-400" />
              </button>

              <div>
                <p class="text-sm font-bold text-slate-900 flex items-center gap-2">
                  <span>{{ rule.name }}</span>
                  <span
                    class="text-xs px-2 py-0.5 rounded font-semibold"
                    :class="rule.is_active ? 'bg-emerald-50 text-emerald-700 border border-emerald-200' : 'bg-slate-100 text-slate-500'"
                  >
                    {{ rule.is_active ? 'AKTIF' : 'JEDA' }}
                  </span>
                </p>
                <div class="flex items-center space-x-2 text-xs text-slate-600 mt-1">
                  <span class="text-amber-700 font-semibold">JIKA: {{ rule.trigger_type }}</span>
                  <span>→</span>
                  <span class="text-emerald-700 font-semibold">MAKA: {{ rule.action_type }}</span>
                </div>
              </div>
            </div>

            <button
              @click="handleDelete(rule.id)"
              class="p-2 rounded-lg text-slate-400 hover:text-red-600 hover:bg-red-50 transition"
              title="Hapus Aturan"
            >
              <Trash2 class="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
