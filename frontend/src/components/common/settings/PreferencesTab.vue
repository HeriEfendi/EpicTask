<script setup>
import { ref } from 'vue';
import { wsClient } from '@/services/websocket';
import { Volume2, VolumeX, Bell, Radio, Check, RefreshCw } from 'lucide-vue-next';

const soundEnabled = ref(localStorage.getItem('epictask_sound_enabled') !== 'false');
const desktopNotifStatus = ref(typeof Notification !== 'undefined' ? Notification.permission : 'default');

function playChime() {
  try {
    const AudioContext = window.AudioContext || window.webkitAudioContext;
    if (!AudioContext) return;
    const ctx = new AudioContext();
    const osc = ctx.createOscillator();
    const gain = ctx.createGain();

    osc.type = 'sine';
    osc.frequency.setValueAtTime(587.33, ctx.currentTime);
    osc.frequency.exponentialRampToValueAtTime(880, ctx.currentTime + 0.15);

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

async function requestDesktopNotification() {
  if (typeof Notification === 'undefined') return;
  try {
    const permission = await Notification.requestPermission();
    desktopNotifStatus.value = permission;
  } catch (e) {
    console.warn(e);
  }
}
</script>

<template>
  <div class="space-y-4">
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
</template>

