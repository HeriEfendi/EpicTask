<script setup>
import { ref, computed } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { api } from '@/services/api';
import { ShieldCheck, Users, Plus } from 'lucide-vue-next';

const props = defineProps({
  members: {
    type: Array,
    default: () => []
  },
  loadingMembers: {
    type: Boolean,
    default: false
  },
  currentUserRole: {
    type: String,
    default: 'MEMBER'
  }
});

const emit = defineEmits(['refresh-members']);

const authStore = useAuthStore();

const selectedUserId = ref('');
const selectedRole = ref('MEMBER');
const isAddingMember = ref(false);
const memberActionMsg = ref('');

const availableUsersToInvite = computed(() => {
  const memberIds = new Set(props.members.map((m) => m.user_id));
  return authStore.allUsers.filter((u) => !memberIds.has(u.id));
});

async function handleAddMember() {
  if (!selectedUserId.value || !authStore.currentWorkspace?.id) return;
  isAddingMember.value = true;
  memberActionMsg.value = '';
  try {
    await api.post(`/workspaces/${authStore.currentWorkspace.id}/members`, {
      user_id: Number(selectedUserId.value),
      role: selectedRole.value,
    });
    memberActionMsg.value = 'Anggota berhasil ditambahkan!';
    selectedUserId.value = '';
    emit('refresh-members');
  } catch (e) {
    memberActionMsg.value = 'Gagal menambahkan: ' + (e.message || 'Error');
  } finally {
    isAddingMember.value = false;
  }
}
</script>

<template>
  <div class="space-y-6">
    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex items-center justify-between">
      <div>
        <p class="text-xs font-semibold text-slate-400 uppercase tracking-wider">Workspace Aktif</p>
        <h3 class="text-base font-bold text-slate-900 mt-0.5">
          {{ authStore.currentWorkspace?.name || 'Belum ada workspace' }}
        </h3>
        <p class="text-xs text-slate-500 mt-1">ID: #{{ authStore.currentWorkspace?.id }}</p>
      </div>
      <div class="text-right">
        <span class="inline-flex items-center space-x-1.5 px-2.5 py-1 rounded-full text-xs font-bold bg-blue-100 text-blue-700 border border-blue-200">
          <ShieldCheck class="w-3.5 h-3.5" />
          <span>Role: {{ currentUserRole }}</span>
        </span>
      </div>
    </div>

    <!-- Add Member Section -->
    <div class="space-y-3">
      <h4 class="text-sm font-bold text-slate-800 flex items-center space-x-2">
        <Users class="w-4 h-4 text-slate-500" />
        <span>Daftar Anggota Tim ({{ members.length }})</span>
      </h4>

      <div v-if="loadingMembers" class="py-6 text-center text-slate-400 text-sm">
        Memuat anggota workspace...
      </div>

      <div v-else class="divide-y divide-slate-100 border border-slate-200 rounded-xl overflow-hidden">
        <div
          v-for="m in members"
          :key="m.user_id"
          class="p-3.5 flex items-center justify-between bg-white hover:bg-slate-50 transition"
        >
          <div class="flex items-center space-x-3">
            <img
              :src="m.avatar_url || 'https://api.dicebear.com/7.x/avataaars/svg?seed=' + m.email"
              class="w-8 h-8 rounded-full bg-slate-100 ring-1 ring-slate-200 object-cover"
              alt="avatar"
            />
            <div>
              <p class="text-sm font-bold text-slate-900">{{ m.full_name || 'Pengguna' }}</p>
              <p class="text-xs text-slate-500">{{ m.email }}</p>
            </div>
          </div>
          <div>
            <span
              class="px-2.5 py-0.5 rounded-md text-xs font-semibold"
              :class="
                m.role === 'OWNER'
                  ? 'bg-amber-100 text-amber-800 border border-amber-200'
                  : m.role === 'ADMIN'
                  ? 'bg-purple-100 text-purple-800 border border-purple-200'
                  : 'bg-slate-100 text-slate-700 border border-slate-200'
              "
            >
              {{ m.role }}
            </span>
          </div>
        </div>
      </div>

      <!-- Invite / Add Member Form -->
      <div class="p-4 bg-slate-50 rounded-xl border border-slate-200 mt-4 space-y-3">
        <p class="text-xs font-bold text-slate-700 uppercase tracking-wider">Tambah Anggota ke Workspace</p>
        <div class="flex flex-col sm:flex-row items-center gap-2">
          <select
            v-model="selectedUserId"
            class="flex-1 w-full bg-white border border-slate-200 rounded-lg px-3 py-2 text-sm text-slate-800 focus:outline-blue-500"
          >
            <option value="" disabled>Pilih pengguna untuk ditambahkan</option>
            <option
              v-for="u in availableUsersToInvite"
              :key="u.id"
              :value="u.id"
            >
              {{ u.full_name }} ({{ u.email }})
            </option>
          </select>

          <select
            v-model="selectedRole"
            class="w-full sm:w-32 bg-white border border-slate-200 rounded-lg px-3 py-2 text-sm text-slate-800 focus:outline-blue-500"
          >
            <option value="MEMBER">MEMBER</option>
            <option value="ADMIN">ADMIN</option>
          </select>

          <button
            @click="handleAddMember"
            :disabled="!selectedUserId || isAddingMember"
            class="w-full sm:w-auto px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white rounded-lg text-sm font-semibold transition flex items-center justify-center space-x-1.5 shrink-0"
          >
            <Plus class="w-4 h-4" />
            <span>Tambah</span>
          </button>
        </div>

        <p v-if="memberActionMsg" class="text-xs font-semibold" :class="memberActionMsg.includes('berhasil') ? 'text-green-600' : 'text-red-500'">
          {{ memberActionMsg }}
        </p>
      </div>
    </div>
  </div>
</template>

