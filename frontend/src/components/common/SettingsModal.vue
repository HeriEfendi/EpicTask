<script setup>
import { ref, computed, watch } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { useProjectStore } from '@/stores/project';
import { api } from '@/services/api';
import { wsClient } from '@/services/websocket';
import {
  Settings,
  X,
  Building,
  User,
  Sliders,
  Database,
  Bell,
  Volume2,
  VolumeX,
  ShieldCheck,
  Plus,
  RefreshCw,
  Check,
  CheckCircle2,
  Radio,
  Sparkles,
  Users
} from 'lucide-vue-next';
import { toast } from '@/utils/toast';

const props = defineProps({
  isOpen: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['close']);

const authStore = useAuthStore();
const projectStore = useProjectStore();

const activeTab = ref('workspace'); // 'workspace' | 'profile' | 'preferences' | 'system'
const loadingMembers = ref(false);
const members = ref([]);
const currentUserRole = ref('MEMBER');

// New member form
const selectedUserId = ref('');
const selectedRole = ref('MEMBER');
const isAddingMember = ref(false);
const memberActionMsg = ref('');

// Preferences (stored in localStorage)
const soundEnabled = ref(localStorage.getItem('epictask_sound_enabled') !== 'false');
const compactMode = ref(localStorage.getItem('epictask_compact_mode') === 'true');
const desktopNotifStatus = ref(typeof Notification !== 'undefined' ? Notification.permission : 'default');

// Reset seed state
const isResetting = ref(false);
const resetSuccess = ref(false);

// Preset Avatar seeds
const avatarSeeds = ['Felix', 'Aneka', 'Alex', 'Max', 'Oliver', 'Mia', 'Sophia', 'Liam'];
const selectedSeed = ref('');

// Web Audio synthesizer for test sound chime
function playChime() {
  try {
    const AudioContext = window.AudioContext || window.webkitAudioContext;
    if (!AudioContext) return;
    const ctx = new AudioContext();
    const osc = ctx.createOscillator();
    const gain = ctx.createGain();

    osc.type = 'sine';
    osc.frequency.setValueAtTime(587.33, ctx.currentTime); // D5
    osc.frequency.exponentialRampToValueAtTime(880, ctx.currentTime + 0.15); // A5

    gain.gain.setValueAtTime(0.12, ctx.currentTime);
    gain.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + 0.35);

    osc.connect(gain);
    gain.connect(ctx.destination);

    osc.start();
    osc.stop(ctx.currentTime + 0.35);
  } catch (e) {
    console.warn('Audio not available', e);
  }
}

function toggleSound() {
  soundEnabled.value = !soundEnabled.value;
  localStorage.setItem('epictask_sound_enabled', soundEnabled.value ? 'true' : 'false');
  if (soundEnabled.value) {
    playChime();
  }
}

function toggleCompactMode() {
  compactMode.value = !compactMode.value;
  localStorage.setItem('epictask_compact_mode', compactMode.value ? 'true' : 'false');
}

async function requestDesktopNotification() {
  if (typeof Notification === 'undefined') return;
  try {
    const permission = await Notification.requestPermission();
    desktopNotifStatus.value = permission;
  } catch (e) {
    console.warn(e);
  }
}

async function fetchWorkspaceDetails() {
  if (!authStore.currentWorkspace?.id) return;
  loadingMembers.value = true;
  memberActionMsg.value = '';
  try {
    const data = await api.get(`/workspaces/${authStore.currentWorkspace.id}`);
    members.value = data.members || [];
    currentUserRole.value = data.current_user_role || 'MEMBER';
  } catch (e) {
    console.warn('Failed to load workspace members', e);
  } finally {
    loadingMembers.value = false;
  }
}

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
    await fetchWorkspaceDetails();
  } catch (e) {
    memberActionMsg.value = 'Gagal menambahkan: ' + (e.message || 'Error');
  } finally {
    isAddingMember.value = false;
  }
}

function setAvatarSeed(seed) {
  selectedSeed.value = seed;
  const newUrl = `https://api.dicebear.com/7.x/avataaars/svg?seed=${encodeURIComponent(seed)}`;
  if (authStore.user) {
    authStore.user.avatar_url = newUrl;
  }
}

async function handleResetDemoData() {
  if (!confirm('Apakah Anda yakin ingin mereset data demo ke default? Semua tiket dan perubahan akan dikembalikan ke data awal.')) {
    return;
  }
  isResetting.value = true;
  resetSuccess.value = false;
  try {
    await api.post('/seed');
    resetSuccess.value = true;
    toast.success('Data demo berhasil direset ke kondisi awal');
    await authStore.fetchMe();
    if (authStore.currentWorkspace) {
      await projectStore.fetchProjects(authStore.currentWorkspace.id);
    }
  } catch (e) {
    toast.error('Gagal mereset data: ' + (e.message || 'Error'));
  } finally {
    isResetting.value = false;
  }
}

// Available non-member users for invite
const availableUsersToInvite = computed(() => {
  const memberIds = new Set(members.value.map((m) => m.user_id));
  return authStore.allUsers.filter((u) => !memberIds.has(u.id));
});

watch(
  () => props.isOpen,
  (val) => {
    if (val) {
      fetchWorkspaceDetails();
      if (typeof Notification !== 'undefined') {
        desktopNotifStatus.value = Notification.permission;
      }
    }
  }
);
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/40 backdrop-blur-xs animate-fade-in"
    @click.self="emit('close')"
  >
    <div
      class="bg-white rounded-2xl border border-slate-200 shadow-2xl w-full max-w-2xl overflow-hidden flex flex-col max-h-[85vh] animate-scale-up"
    >
      <!-- Modal Header -->
      <div class="px-6 py-4 border-b border-slate-200 flex items-center justify-between bg-slate-50/70">
        <div class="flex items-center space-x-3">
          <div class="p-2 rounded-xl bg-blue-50 text-blue-600 border border-blue-100">
            <Settings class="w-5 h-5" />
          </div>
          <div>
            <h2 class="text-base font-bold text-slate-900">Pengaturan Sistem</h2>
            <p class="text-xs text-slate-500">Kelola workspace, profil, dan preferensi aplikasi</p>
          </div>
        </div>
        <button
          @click="emit('close')"
          class="p-1.5 text-slate-400 hover:text-slate-700 hover:bg-slate-200/60 rounded-lg transition"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Navigation Tabs -->
      <div class="flex border-b border-slate-200 px-6 bg-white overflow-x-auto">
        <button
          @click="activeTab = 'workspace'"
          class="py-3 px-4 text-sm font-semibold flex items-center space-x-2 border-b-2 transition whitespace-nowrap"
          :class="
            activeTab === 'workspace'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-slate-500 hover:text-slate-800'
          "
        >
          <Building class="w-4 h-4" />
          <span>Workspace & Anggota</span>
        </button>

        <button
          @click="activeTab = 'profile'"
          class="py-3 px-4 text-sm font-semibold flex items-center space-x-2 border-b-2 transition whitespace-nowrap"
          :class="
            activeTab === 'profile'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-slate-500 hover:text-slate-800'
          "
        >
          <User class="w-4 h-4" />
          <span>Profil Saya</span>
        </button>

        <button
          @click="activeTab = 'preferences'"
          class="py-3 px-4 text-sm font-semibold flex items-center space-x-2 border-b-2 transition whitespace-nowrap"
          :class="
            activeTab === 'preferences'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-slate-500 hover:text-slate-800'
          "
        >
          <Sliders class="w-4 h-4" />
          <span>Preferensi</span>
        </button>

        <button
          @click="activeTab = 'system'"
          class="py-3 px-4 text-sm font-semibold flex items-center space-x-2 border-b-2 transition whitespace-nowrap"
          :class="
            activeTab === 'system'
              ? 'border-blue-600 text-blue-600'
              : 'border-transparent text-slate-500 hover:text-slate-800'
          "
        >
          <Database class="w-4 h-4" />
          <span>Sistem & Data</span>
        </button>
      </div>

      <!-- Tab Content Area -->
      <div class="p-6 overflow-y-auto flex-1 space-y-6">
        <!-- 1. Workspace & Members -->
        <div v-if="activeTab === 'workspace'" class="space-y-6">
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

        <!-- 2. Profil Saya -->
        <div v-if="activeTab === 'profile'" class="space-y-6">
          <div class="flex items-center space-x-4 p-4 bg-slate-50 rounded-xl border border-slate-200">
            <img
              :src="authStore.user?.avatar_url || 'https://api.dicebear.com/7.x/avataaars/svg?seed=user'"
              class="w-16 h-16 rounded-full bg-white ring-2 ring-blue-500 shadow-sm object-cover"
              alt="Avatar saat ini"
            />
            <div>
              <h3 class="text-base font-bold text-slate-900">{{ authStore.user?.full_name || 'User' }}</h3>
              <p class="text-sm text-slate-500">{{ authStore.user?.email }}</p>
              <span class="inline-block mt-1 px-2 py-0.5 bg-slate-200 text-slate-700 text-xs font-semibold rounded">
                ID Pengguna: #{{ authStore.user?.id }}
              </span>
            </div>
          </div>

          <!-- Quick Avatar Switcher -->
          <div class="space-y-3">
            <label class="text-sm font-bold text-slate-800 flex items-center space-x-2">
              <Sparkles class="w-4 h-4 text-blue-600" />
              <span>Ganti Karakter Avatar Cepat</span>
            </label>
            <div class="grid grid-cols-4 sm:grid-cols-8 gap-2.5">
              <button
                v-for="seed in avatarSeeds"
                :key="seed"
                @click="setAvatarSeed(seed)"
                class="p-1.5 rounded-xl border transition flex flex-col items-center justify-center hover:bg-blue-50/50"
                :class="
                  selectedSeed === seed
                    ? 'border-blue-600 bg-blue-50 shadow-xs'
                    : 'border-slate-200 hover:border-slate-300'
                "
              >
                <img
                  :src="'https://api.dicebear.com/7.x/avataaars/svg?seed=' + seed"
                  class="w-10 h-10 rounded-full"
                  :alt="seed"
                />
                <span class="text-[11px] font-medium text-slate-600 mt-1 truncate max-w-full">
                  {{ seed }}
                </span>
              </button>
            </div>
          </div>
        </div>

        <!-- 3. Preferensi -->
        <div v-if="activeTab === 'preferences'" class="space-y-4">
          <!-- Sound Toggle -->
          <div class="p-4 bg-white rounded-xl border border-slate-200 flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-blue-50 text-blue-600">
                <Volume2 v-if="soundEnabled" class="w-5 h-5" />
                <VolumeX v-else class="w-5 h-5 text-slate-400" />
              </div>
              <div>
                <p class="text-sm font-bold text-slate-900">Efek Suara Notifikasi</p>
                <p class="text-xs text-slate-500">Mainkan audio lembut saat ada notifikasi atau tiket baru</p>
              </div>
            </div>
            <button
              @click="toggleSound"
              class="px-3.5 py-1.5 rounded-lg text-xs font-bold transition flex items-center space-x-1.5 border"
              :class="
                soundEnabled
                  ? 'bg-blue-600 text-white border-blue-600'
                  : 'bg-slate-100 text-slate-600 border-slate-200'
              "
            >
              <span>{{ soundEnabled ? 'Aktif' : 'Nonaktif' }}</span>
            </button>
          </div>

          <!-- Browser Notification Permission -->
          <div class="p-4 bg-white rounded-xl border border-slate-200 flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-amber-50 text-amber-600">
                <Bell class="w-5 h-5" />
              </div>
              <div>
                <p class="text-sm font-bold text-slate-900">Notifikasi Desktop Browser</p>
                <p class="text-xs text-slate-500">Izin pop-up notifikasi desktop saat tab tidak aktif</p>
              </div>
            </div>
            <div>
              <span
                v-if="desktopNotifStatus === 'granted'"
                class="px-3 py-1.5 rounded-lg text-xs font-bold bg-green-50 text-green-700 border border-green-200 flex items-center space-x-1"
              >
                <Check class="w-3.5 h-3.5" />
                <span>Diizinkan</span>
              </span>
              <button
                v-else
                @click="requestDesktopNotification"
                class="px-3 py-1.5 rounded-lg text-xs font-bold bg-blue-600 hover:bg-blue-700 text-white transition"
              >
                Minta Izin
              </button>
            </div>
          </div>

          <!-- Live Sync WebSocket -->
          <div class="p-4 bg-white rounded-xl border border-slate-200 flex items-center justify-between">
            <div class="flex items-center space-x-3">
              <div class="p-2 rounded-lg bg-green-50 text-green-600">
                <Radio class="w-5 h-5 animate-pulse" />
              </div>
              <div>
                <p class="text-sm font-bold text-slate-900">Real-time WebSocket Live Sync</p>
                <p class="text-xs text-slate-500">Sinkronisasi instan perpindahan kartu Kanban antar kolaborator</p>
              </div>
            </div>
            <button
              @click="wsClient.connect()"
              class="px-3 py-1.5 rounded-lg text-xs font-bold bg-slate-100 hover:bg-slate-200 text-slate-700 border border-slate-200 transition flex items-center space-x-1"
            >
              <RefreshCw class="w-3 h-3" />
              <span>Reconnect</span>
            </button>
          </div>
        </div>

        <!-- 4. Sistem & Data -->
        <div v-if="activeTab === 'system'" class="space-y-6">
          <div class="p-4 bg-slate-50 rounded-xl border border-slate-200 space-y-2">
            <h4 class="text-sm font-bold text-slate-900">Tentang Aplikasi</h4>
            <div class="grid grid-cols-2 gap-3 text-xs">
              <div>
                <span class="text-slate-400">Versi:</span>
                <span class="font-semibold text-slate-700 ml-1.5">EpicTask v0.1.0</span>
              </div>
              <div>
                <span class="text-slate-400">Backend:</span>
                <span class="font-semibold text-slate-700 ml-1.5">Rust (Axum + SQLx)</span>
              </div>
              <div>
                <span class="text-slate-400">Database:</span>
                <span class="font-semibold text-slate-700 ml-1.5">SQLite Embedded</span>
              </div>
              <div>
                <span class="text-slate-400">Frontend:</span>
                <span class="font-semibold text-slate-700 ml-1.5">Vue 3 + Vite + Tailwind</span>
              </div>
            </div>
          </div>

          <!-- Reset Seed Database -->
          <div class="p-4 bg-red-50/50 rounded-xl border border-red-200 space-y-3">
            <div>
              <h4 class="text-sm font-bold text-red-900">Reset Demo Data</h4>
              <p class="text-xs text-red-700 mt-0.5">
                Mengembalikan status projek, tiket contoh, dan rule otomatisasi ke kondisi awal bawaan database.
              </p>
            </div>

            <div class="flex items-center space-x-3">
              <button
                @click="handleResetDemoData"
                :disabled="isResetting"
                class="px-4 py-2 bg-red-600 hover:bg-red-700 disabled:opacity-50 text-white rounded-lg text-sm font-semibold transition flex items-center space-x-1.5"
              >
                <RefreshCw class="w-4 h-4" :class="isResetting ? 'animate-spin' : ''" />
                <span>{{ isResetting ? 'Mereset Database...' : 'Reset ke Data Demo' }}</span>
              </button>

              <span
                v-if="resetSuccess"
                class="text-xs font-semibold text-green-600 flex items-center space-x-1"
              >
                <CheckCircle2 class="w-4 h-4" />
                <span>Berhasil direset!</span>
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Modal Footer -->
      <div class="px-6 py-3 border-t border-slate-200 bg-slate-50 flex items-center justify-end">
        <button
          @click="emit('close')"
          class="px-4 py-2 rounded-lg text-sm font-semibold bg-white border border-slate-200 text-slate-700 hover:bg-slate-100 transition shadow-2xs"
        >
          Tutup
        </button>
      </div>
    </div>
  </div>
</template>
