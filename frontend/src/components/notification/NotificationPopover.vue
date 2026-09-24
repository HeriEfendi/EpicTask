<script setup>
import { useNotificationStore } from '@/stores/notification';
import { useProjectStore } from '@/stores/project';
import { Bell, CheckCheck, X, MessageSquare, AlertCircle, Sparkles } from 'lucide-vue-next';

const notifStore = useNotificationStore();
const projectStore = useProjectStore();

async function handleNotificationClick(notif) {
  if (!notif.is_read) {
    await notifStore.markAsRead(notif.id);
  }
  if (notif.issue_id) {
    await projectStore.openIssueDetail(notif.issue_id);
    notifStore.isOpen = false;
  }
}
</script>

<template>
  <div
    v-if="notifStore.isOpen"
    class="fixed inset-0 z-40"
    @click="notifStore.isOpen = false"
  >
    <div
      class="absolute right-6 top-16 w-80 sm:w-96 glass-dropdown rounded-2xl border border-slate-200 bg-white shadow-2xl p-4 z-50 animate-slide-up"
      @click.stop
    >
      <!-- Popover Header -->
      <div class="flex items-center justify-between pb-3 border-b border-slate-200 select-none">
        <div class="flex items-center space-x-2">
          <Bell class="w-4 h-4 text-blue-600" />
          <h3 class="text-xs font-bold text-slate-800 uppercase tracking-wider">Notifications</h3>
          <span
            v-if="notifStore.unreadCount > 0"
            class="px-1.5 py-0.2 bg-blue-600 text-white text-[10px] font-bold rounded-full"
          >
            {{ notifStore.unreadCount }} new
          </span>
        </div>

        <div class="flex items-center space-x-2">
          <button
            v-if="notifStore.unreadCount > 0"
            @click="notifStore.markAllAsRead"
            class="text-[11px] text-blue-600 hover:text-blue-700 font-semibold flex items-center space-x-1"
          >
            <CheckCheck class="w-3.5 h-3.5" />
            <span>Mark read</span>
          </button>
          <button
            @click="notifStore.isOpen = false"
            class="text-slate-400 hover:text-slate-700 p-0.5"
          >
            <X class="w-4 h-4" />
          </button>
        </div>
      </div>

      <!-- Notifications List -->
      <div class="max-h-80 overflow-y-auto divide-y divide-slate-100 mt-2">
        <div
          v-if="notifStore.notifications.length === 0"
          class="py-8 text-center text-slate-400 text-xs flex flex-col items-center justify-center"
        >
          <Bell class="w-8 h-8 text-slate-300 mb-2" />
          <span>No notifications yet</span>
        </div>

        <div
          v-for="notif in notifStore.notifications"
          :key="notif.id"
          @click="handleNotificationClick(notif)"
          class="p-3 hover:bg-slate-50 rounded-xl transition cursor-pointer flex items-start space-x-3 text-xs"
          :class="!notif.is_read ? 'bg-blue-50/50' : ''"
        >
          <!-- Unread dot -->
          <div
            class="w-2 h-2 rounded-full mt-1.5 shrink-0"
            :class="!notif.is_read ? 'bg-blue-600 shadow-xs shadow-blue-500' : 'bg-transparent'"
          ></div>

          <div class="flex-1">
            <div class="flex items-center justify-between mb-0.5">
              <span class="font-bold text-slate-800 truncate">{{ notif.title }}</span>
              <span class="text-[10px] text-slate-400">{{ new Date(notif.created_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) }}</span>
            </div>
            <p class="text-slate-600 text-[11px] line-clamp-2 leading-relaxed">{{ notif.message }}</p>
            <div v-if="notif.issue_key" class="mt-1">
              <span class="font-mono text-[10px] font-bold px-1.5 py-0.2 rounded bg-slate-100 border border-slate-200 text-blue-600">
                {{ notif.issue_key }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
