<script setup>
import { ref } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { useProjectStore } from '@/stores/project';
import { useNotificationStore } from '@/stores/notification';
import { wsClient } from '@/services/websocket';
import {
  Plus,
  Bell,
  Search,
  LogOut,
  PanelLeft,
  FolderKanban,
  Settings
} from 'lucide-vue-next';

const props = defineProps({
  isSidebarCollapsed: {
    type: Boolean,
    default: false,
  }
});

const emit = defineEmits(['toggle-sidebar', 'open-settings']);

const authStore = useAuthStore();
const projectStore = useProjectStore();
const notifStore = useNotificationStore();

const isUserMenuOpen = ref(false);
</script>

<template>
  <header class="h-14 border-b border-slate-200 bg-white px-5 flex items-center justify-between z-20 sticky top-0 shadow-2xs">
    <!-- Left: Sidebar Toggle & Active Project Breadcrumb -->
    <div class="flex items-center space-x-3">
      <button
        v-if="isSidebarCollapsed"
        @click="emit('toggle-sidebar')"
        class="p-2 rounded-lg text-slate-500 hover:text-slate-800 hover:bg-slate-100 border border-slate-200 shadow-2xs transition"
        title="Buka Side Menu"
      >
        <PanelLeft class="w-4 h-4" />
      </button>

      <div class="flex items-center space-x-1.5 font-bold text-slate-900 text-base">
        <FolderKanban class="w-4 h-4 text-blue-600 shrink-0" />
        <span class="truncate max-w-[200px]">
          {{ projectStore.currentProject ? projectStore.currentProject.name : 'Pilih Projek' }}
        </span>
        <span
          v-if="projectStore.currentProject?.key"
          class="text-xs font-mono font-semibold px-2 py-0.5 rounded bg-slate-100 border border-slate-200 text-slate-600"
        >
          {{ projectStore.currentProject.key }}
        </span>
      </div>
    </div>

    <!-- Center: Search Input (Enhanced 1 Level Larger Font) -->
    <div class="flex-1 max-w-lg mx-6">
      <div class="relative">
        <Search class="w-4 h-4 text-slate-400 absolute left-3.5 top-1/2 -translate-y-1/2" />
        <input
          v-model="projectStore.searchQuery"
          type="text"
          placeholder="Cari tiket, ringkasan, atau ID (misal: EPIC-1)..."
          class="w-full bg-slate-100 hover:bg-slate-100/70 border border-slate-200 rounded-lg pl-10 pr-4 py-2 text-sm text-slate-900 placeholder-slate-400 focus:outline-none focus:border-blue-600 focus:bg-white focus:ring-1 focus:ring-blue-600 transition shadow-2xs"
        />
        <span class="absolute right-3 top-1/2 -translate-y-1/2 text-xs text-slate-400 font-mono border border-slate-200 bg-white rounded px-1.5 py-0.5 shadow-2xs">
          /
        </span>
      </div>
    </div>

    <!-- Right: Create Button, Live Sync, Notification, User -->
    <div class="flex items-center space-x-3">
      <!-- Quick "+ Buat Tiket" Action Button -->
      <button
        @click="projectStore.isCreateModalOpen = true"
        class="bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white font-semibold text-sm px-3.5 py-2 rounded-lg shadow-xs flex items-center space-x-1.5 transition active:scale-95"
      >
        <Plus class="w-4 h-4" />
        <span>Buat Tiket</span>
      </button>

      <div class="h-5 w-[1px] bg-slate-200"></div>

      <!-- Real-time WS Sync Indicator -->
      <div
        class="flex items-center space-x-1.5 px-2.5 py-1.5 rounded-lg text-xs font-semibold transition cursor-default"
        :class="wsClient.isConnected ? 'bg-emerald-50 text-emerald-700 border border-emerald-200' : 'bg-amber-50 text-amber-700 border border-amber-200'"
        :title="wsClient.isConnected ? 'Real-time WebSocket sinkron aktif' : 'Menghubungkan ke server...'"
      >
        <span class="relative flex h-2 w-2">
          <span v-if="wsClient.isConnected" class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
          <span class="relative inline-flex rounded-full h-2 w-2" :class="wsClient.isConnected ? 'bg-emerald-500' : 'bg-amber-500'"></span>
        </span>
        <span class="hidden md:inline">{{ wsClient.isConnected ? 'Sinkron' : 'Koneksi' }}</span>
      </div>

      <!-- Notification Bell with Badge -->
      <button
        @click="notifStore.isOpen = !notifStore.isOpen"
        class="relative p-2 rounded-lg text-slate-600 hover:text-slate-900 hover:bg-slate-100 border border-slate-200/60 shadow-2xs transition"
        title="Notifikasi"
      >
        <Bell class="w-4 h-4" />
        <span
          v-if="notifStore.unreadCount > 0"
          class="absolute top-1.5 right-1.5 w-2 h-2 bg-red-500 rounded-full"
        ></span>
      </button>

      <!-- Settings Button -->
      <button
        @click="emit('open-settings')"
        class="p-2 rounded-lg text-slate-600 hover:text-slate-900 hover:bg-slate-100 border border-slate-200/60 shadow-2xs transition"
        title="Pengaturan"
      >
        <Settings class="w-4 h-4" />
      </button>

      <!-- User Avatar / Profile Menu -->
      <div class="relative">
        <button
          @click="isUserMenuOpen = !isUserMenuOpen"
          class="flex items-center justify-center p-0.5 rounded-full hover:ring-2 hover:ring-blue-500/30 transition focus:outline-none"
          :title="authStore.user?.full_name || 'Profil Pengguna'"
        >
          <img
            :src="authStore.user?.avatar_url || 'https://api.dicebear.com/7.x/avataaars/svg?seed=user'"
            alt="Avatar"
            class="w-8 h-8 rounded-full bg-slate-200 ring-1 ring-slate-300 object-cover"
          />
        </button>

        <!-- Backdrop to close on click outside -->
        <div
          v-if="isUserMenuOpen"
          class="fixed inset-0 z-40"
          @click="isUserMenuOpen = false"
        ></div>

        <!-- User Menu Dropdown -->
        <div
          v-if="isUserMenuOpen"
          class="absolute right-0 mt-2 w-60 glass-dropdown rounded-xl p-2 z-50 animate-slide-up shadow-xl"
          @click.stop
        >
          <div class="px-3.5 py-2.5 border-b border-slate-100">
            <p class="text-sm font-bold text-slate-900 truncate">{{ authStore.user?.full_name }}</p>
            <p class="text-xs text-slate-500 truncate mt-0.5">{{ authStore.user?.email }}</p>
          </div>

          <div class="py-1">
            <button
              @click="authStore.logout(); isUserMenuOpen = false"
              class="w-full text-left px-3 py-2 rounded-lg text-sm text-red-600 hover:bg-red-50 flex items-center space-x-2.5 transition font-semibold"
            >
              <LogOut class="w-4 h-4" />
              <span>Keluar (Log Out)</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>
