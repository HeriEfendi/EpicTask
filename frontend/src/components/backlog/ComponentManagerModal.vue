<script setup>
import { ref } from 'vue';
import { X, Boxes, Plus, Trash2, Edit2, User, Check, AlertCircle } from 'lucide-vue-next';
import { useComponentStore } from '@/stores/component';
import { useAuthStore } from '@/stores/auth';
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
});

const emit = defineEmits(['close']);

const compStore = useComponentStore();
const authStore = useAuthStore();

const newName = ref('');
const newDescription = ref('');
const newLeadUserId = ref(null);
const isAdding = ref(false);

const editingId = ref(null);
const editName = ref('');
const editDescription = ref('');
const editLeadUserId = ref(null);

async function handleCreateComponent() {
  if (!newName.value.trim()) {
    toast.warning('Nama komponen wajib diisi');
    return;
  }

  try {
    await compStore.createComponent(props.projectId, {
      name: newName.value.trim(),
      description: newDescription.value.trim() || null,
      lead_user_id: newLeadUserId.value ? Number(newLeadUserId.value) : null,
    });
    newName.value = '';
    newDescription.value = '';
    newLeadUserId.value = null;
    isAdding.value = false;
  } catch (err) {
    // Toast handled in store
  }
}

function startEdit(comp) {
  editingId.value = comp.id;
  editName.value = comp.name;
  editDescription.value = comp.description || '';
  editLeadUserId.value = comp.lead_user_id || null;
}

function cancelEdit() {
  editingId.value = null;
}

async function handleUpdateComponent(comp) {
  if (!editName.value.trim()) {
    toast.warning('Nama komponen wajib diisi');
    return;
  }

  try {
    await compStore.updateComponent(comp.id, {
      name: editName.value.trim(),
      description: editDescription.value.trim() || null,
      lead_user_id: editLeadUserId.value ? Number(editLeadUserId.value) : null,
    });
    editingId.value = null;
  } catch (err) {
    // Handled in store
  }
}

async function handleDeleteComponent(comp) {
  if (confirm(`Hapus komponen "${comp.name}"? Tiket yang terhubung akan dilepas komponennya.`)) {
    await compStore.deleteComponent(comp.id);
  }
}
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 z-50 bg-slate-900/40 backdrop-blur-xs flex items-center justify-center p-4"
  >
    <div class="bg-white rounded-2xl border border-slate-200 shadow-2xl w-full max-w-2xl max-h-[85vh] flex flex-col overflow-hidden animate-slide-up">
      <!-- Header -->
      <div class="px-6 py-4 border-b border-slate-100 flex items-center justify-between shrink-0">
        <div class="flex items-center space-x-2.5">
          <div class="w-8 h-8 rounded-lg bg-cyan-50 border border-cyan-200 flex items-center justify-center">
            <Boxes class="w-4 h-4 text-cyan-700" />
          </div>
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Kelola Komponen Proyek</h3>
            <p class="text-xs text-slate-500">Kelompokkan pekerjaan berdasarkan modul dan sub-sistem teknis</p>
          </div>
        </div>

        <button
          @click="emit('close')"
          class="text-slate-400 hover:text-slate-700 p-1 rounded-lg transition"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <!-- Content Scrollable Body -->
      <div class="flex-1 overflow-y-auto p-6 space-y-6">
        <!-- New Component Button / Form -->
        <div class="bg-slate-50 border border-slate-200 rounded-xl p-4">
          <div v-if="!isAdding" class="flex items-center justify-between">
            <div>
              <p class="text-xs font-bold text-slate-800 uppercase tracking-wider">Tambah Komponen Baru</p>
              <p class="text-xs text-slate-500">Contoh: Backend API, Frontend UI, Mobile App, Auth & Security</p>
            </div>
            <button
              @click="isAdding = true"
              class="flex items-center space-x-1.5 px-3 py-1.5 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-xs font-bold transition shadow-xs"
            >
              <Plus class="w-3.5 h-3.5" />
              <span>Tambah Komponen</span>
            </button>
          </div>

          <!-- Add Form -->
          <div v-else class="space-y-3">
            <div class="flex items-center justify-between">
              <span class="text-xs font-bold text-slate-800 uppercase tracking-wider">Form Komponen Baru</span>
              <button
                @click="isAdding = false"
                class="text-xs text-slate-400 hover:text-slate-600"
              >
                Batal
              </button>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <div class="space-y-1">
                <label class="block text-2xs font-bold text-slate-600 uppercase tracking-wider">Nama Komponen *</label>
                <input
                  v-model="newName"
                  type="text"
                  placeholder="Misal: Payment Gateway"
                  class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
                />
              </div>

              <div class="space-y-1">
                <label class="block text-2xs font-bold text-slate-600 uppercase tracking-wider">Component Lead</label>
                <select
                  v-model="newLeadUserId"
                  class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
                >
                  <option :value="null">Belum Ada Lead</option>
                  <option v-for="u in authStore.allUsers" :key="u.id" :value="u.id">
                    {{ u.full_name }}
                  </option>
                </select>
              </div>
            </div>

            <div class="space-y-1">
              <label class="block text-2xs font-bold text-slate-600 uppercase tracking-wider">Deskripsi</label>
              <input
                v-model="newDescription"
                type="text"
                placeholder="Penjelasan cakupan modul atau repositori..."
                class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs"
              />
            </div>

            <div class="flex justify-end space-x-2 pt-1">
              <button
                @click="isAdding = false"
                class="px-3 py-1 text-xs text-slate-600 hover:text-slate-800"
              >
                Batal
              </button>
              <button
                @click="handleCreateComponent"
                class="px-3.5 py-1 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-xs font-bold transition shadow-xs"
              >
                Simpan Komponen
              </button>
            </div>
          </div>
        </div>

        <!-- Components Table / List -->
        <div class="space-y-2">
          <div class="flex items-center justify-between text-xs font-bold text-slate-500 uppercase tracking-wider px-1">
            <span>Daftar Komponen ({{ compStore.components.length }})</span>
          </div>

          <div v-if="compStore.components.length === 0" class="text-center py-10 bg-slate-50 border border-slate-200 rounded-xl text-xs text-slate-400 italic">
            Belum ada komponen di proyek ini. Buat komponen pertama di atas.
          </div>

          <div v-else class="border border-slate-200 rounded-xl divide-y divide-slate-100 overflow-hidden bg-white shadow-2xs">
            <div
              v-for="c in compStore.components"
              :key="c.id"
              class="p-3.5 hover:bg-slate-50/80 transition"
            >
              <!-- View Row -->
              <div v-if="editingId !== c.id" class="flex items-center justify-between">
                <div class="space-y-0.5 min-w-0 pr-4 flex-1">
                  <div class="flex items-center space-x-2">
                    <span class="font-bold text-sm text-slate-900">🧩 {{ c.name }}</span>
                    <span class="text-2xs font-semibold px-2 py-0.5 rounded-full bg-slate-100 text-slate-600">
                      {{ c.issue_count || 0 }} tiket
                    </span>
                  </div>
                  <p v-if="c.description" class="text-xs text-slate-500 truncate">
                    {{ c.description }}
                  </p>
                  <p v-if="c.lead_name" class="text-xs text-slate-600 flex items-center space-x-1 pt-0.5">
                    <User class="w-3 h-3 text-slate-400" />
                    <span>Lead: <strong>{{ c.lead_name }}</strong></span>
                  </p>
                </div>

                <div class="flex items-center space-x-1 shrink-0">
                  <button
                    @click="startEdit(c)"
                    class="p-1.5 text-slate-400 hover:text-slate-700 rounded-lg hover:bg-slate-100 transition"
                    title="Edit Komponen"
                  >
                    <Edit2 class="w-3.5 h-3.5" />
                  </button>
                  <button
                    @click="handleDeleteComponent(c)"
                    class="p-1.5 text-rose-400 hover:text-rose-600 rounded-lg hover:bg-rose-50 transition"
                    title="Hapus Komponen"
                  >
                    <Trash2 class="w-3.5 h-3.5" />
                  </button>
                </div>
              </div>

              <!-- Inline Edit Row -->
              <div v-else class="space-y-2.5 py-1">
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                  <input
                    v-model="editName"
                    type="text"
                    class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600 font-bold"
                  />
                  <select
                    v-model="editLeadUserId"
                    class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600"
                  >
                    <option :value="null">Belum Ada Lead</option>
                    <option v-for="u in authStore.allUsers" :key="u.id" :value="u.id">
                      {{ u.full_name }}
                    </option>
                  </select>
                </div>
                <input
                  v-model="editDescription"
                  type="text"
                  placeholder="Deskripsi..."
                  class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600"
                />
                <div class="flex justify-end space-x-2">
                  <button
                    @click="cancelEdit"
                    class="px-2.5 py-1 text-xs text-slate-500 hover:text-slate-700"
                  >
                    Batal
                  </button>
                  <button
                    @click="handleUpdateComponent(c)"
                    class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-xs font-bold shadow-xs"
                  >
                    Simpan
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

