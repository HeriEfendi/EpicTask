<script setup>
import { ref, watch, computed, onMounted, onUnmounted } from 'vue';
import { useWatcherStore } from '@/stores/watcher';
import { useAuthStore } from '@/stores/auth';
import {
  Eye,
  EyeOff,
  Users,
  UserPlus,
  Trash2,
  ChevronDown,
  Check,
  X
} from 'lucide-vue-next';

const props = defineProps({
  issue: {
    type: Object,
    required: true,
  },
});

const watcherStore = useWatcherStore();
const authStore = useAuthStore();

const isDropdownOpen = ref(false);
const dropdownRef = ref(null);
const isAddMemberOpen = ref(false);

watch(
  () => props.issue?.id,
  (newId) => {
    if (newId) {
      watcherStore.fetchWatchers(newId);
    }
  },
  { immediate: true }
);

function handleClickOutside(e) {
  if (dropdownRef.value && !dropdownRef.value.contains(e.target)) {
    isDropdownOpen.value = false;
    isAddMemberOpen.value = false;
  }
}

onMounted(() => document.addEventListener('mousedown', handleClickOutside));
onUnmounted(() => document.removeEventListener('mousedown', handleClickOutside));

const availableUsersToAdd = computed(() => {
  const currentIds = watcherStore.watchers.map((w) => w.user_id);
  return authStore.allUsers.filter((u) => !currentIds.includes(u.id));
});

async function handleAddWatcher(userId) {
  await watcherStore.addWatcher(props.issue.id, userId);
  isAddMemberOpen.value = false;
}

async function handleRemoveWatcher(userId) {
  await watcherStore.removeWatcher(props.issue.id, userId);
}
</script>

<template>
  <div class="space-y-2 relative" ref="dropdownRef">
    <div class="flex items-center justify-between">
      <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1.5">
        <Eye class="w-3.5 h-3.5 text-slate-500" />
        <span>Pengamat (Watchers)</span>
      </label>

      <!-- Watchers Count Badge & Dropdown Trigger -->
      <button
        type="button"
        @click="isDropdownOpen = !isDropdownOpen"
        class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-bold transition select-none cursor-pointer border shadow-2xs"
        :class="[
          watcherStore.watcherCount > 0
            ? 'bg-blue-50 text-blue-700 border-blue-200 hover:bg-blue-100'
            : 'bg-slate-100 text-slate-600 border-slate-200 hover:bg-slate-200/70'
        ]"
        title="Klik untuk melihat daftar pengamat tiket"
      >
        <Users class="w-3 h-3" />
        <span>{{ watcherStore.watcherCount }}</span>
        <ChevronDown class="w-3 h-3 text-slate-400" />
      </button>
    </div>

    <!-- Toggle Watch / Unwatch Primary Action Button -->
    <button
      type="button"
      @click="watcherStore.toggleWatch(issue.id)"
      class="w-full flex items-center justify-center gap-2 px-3 py-2 rounded-xl text-xs font-bold border transition shadow-2xs select-none cursor-pointer"
      :class="[
        watcherStore.isCurrentUserWatching
          ? 'bg-blue-50 text-blue-700 border-blue-300 hover:bg-blue-100 hover:border-blue-400'
          : 'bg-white text-slate-700 border-slate-300 hover:bg-slate-50 hover:border-slate-400'
      ]"
    >
      <EyeOff v-if="watcherStore.isCurrentUserWatching" class="w-3.5 h-3.5 text-blue-600" />
      <Eye v-else class="w-3.5 h-3.5 text-slate-500" />
      <span>{{ watcherStore.isCurrentUserWatching ? 'Berhenti Mengamati (Watching)' : 'Amati Tiket Ini (Watch)' }}</span>
    </button>

    <!-- Watchers List Dropdown Popover -->
    <transition
      enter-active-class="transition duration-150 ease-out"
      enter-from-class="opacity-0 translate-y-1"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="transition duration-100 ease-in"
      leave-from-class="opacity-100 translate-y-0"
      leave-to-class="opacity-0 translate-y-1"
    >
      <div
        v-if="isDropdownOpen"
        class="absolute left-0 right-0 z-30 mt-1 bg-white border border-slate-200 rounded-xl shadow-xl p-3 space-y-3"
      >
        <div class="flex items-center justify-between border-b border-slate-100 pb-2">
          <div class="flex items-center gap-1.5">
            <Users class="w-4 h-4 text-slate-500" />
            <span class="text-xs font-bold text-slate-800">Daftar Pengamat ({{ watcherStore.watcherCount }})</span>
          </div>
          <button
            type="button"
            @click="isDropdownOpen = false"
            class="text-slate-400 hover:text-slate-600 p-0.5 rounded"
          >
            <X class="w-3.5 h-3.5" />
          </button>
        </div>

        <!-- Watchers List -->
        <div class="space-y-1.5 max-h-40 overflow-y-auto pr-1">
          <div
            v-if="watcherStore.watchers.length === 0"
            class="text-center py-3 text-xs text-slate-400 italic"
          >
            Belum ada anggota yang mengamati tiket ini.
          </div>

          <div
            v-for="w in watcherStore.watchers"
            :key="w.user_id"
            class="flex items-center justify-between p-1.5 rounded-lg hover:bg-slate-50 text-xs transition group"
          >
            <div class="flex items-center space-x-2 min-w-0 pr-2">
              <img
                v-if="w.avatar_url"
                :src="w.avatar_url"
                :alt="w.full_name"
                class="w-5 h-5 rounded-full ring-1 ring-slate-200 shrink-0"
              />
              <div
                v-else
                class="w-5 h-5 rounded-full bg-blue-100 text-blue-700 flex items-center justify-center font-bold text-[10px] shrink-0"
              >
                {{ w.full_name?.charAt(0) || 'U' }}
              </div>
              <div class="min-w-0">
                <p class="font-bold text-slate-800 truncate">
                  {{ w.full_name }}
                  <span v-if="w.user_id === authStore.user?.id" class="text-[10px] text-blue-600 font-semibold">(Anda)</span>
                </p>
                <p class="text-[10px] text-slate-400 truncate">{{ w.email }}</p>
              </div>
            </div>

            <!-- Remove Button -->
            <button
              type="button"
              @click="handleRemoveWatcher(w.user_id)"
              title="Hapus pengamat"
              class="text-slate-400 hover:text-red-600 p-1 rounded hover:bg-red-50 opacity-0 group-hover:opacity-100 transition shrink-0"
            >
              <Trash2 class="w-3.5 h-3.5" />
            </button>
          </div>
        </div>

        <!-- Add Watcher Section -->
        <div class="border-t border-slate-100 pt-2">
          <button
            v-if="!isAddMemberOpen"
            type="button"
            @click="isAddMemberOpen = true"
            class="w-full flex items-center justify-center gap-1.5 px-2.5 py-1.5 rounded-lg text-xs font-semibold text-slate-600 hover:text-slate-800 bg-slate-50 hover:bg-slate-100 border border-slate-200 transition"
          >
            <UserPlus class="w-3.5 h-3.5 text-slate-500" />
            <span>+ Tambah Anggota Lain</span>
          </button>

          <!-- Add Member Selector -->
          <div v-else class="space-y-1.5 animate-in fade-in duration-100">
            <div class="flex items-center justify-between text-2xs font-bold text-slate-500 uppercase">
              <span>Pilih Anggota Tim:</span>
              <button
                type="button"
                @click="isAddMemberOpen = false"
                class="text-slate-400 hover:text-slate-600"
              >
                Batal
              </button>
            </div>
            <select
              @change="(e) => e.target.value && handleAddWatcher(Number(e.target.value))"
              class="w-full bg-white border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-800 focus:outline-none focus:border-blue-600 shadow-2xs font-medium cursor-pointer"
            >
              <option value="">Pilih anggota...</option>
              <option
                v-for="u in availableUsersToAdd"
                :key="u.id"
                :value="u.id"
              >
                {{ u.full_name }} ({{ u.email }})
              </option>
            </select>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

