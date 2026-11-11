<script setup>
import { ref } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { Sparkles } from 'lucide-vue-next';

const authStore = useAuthStore();

const avatarSeeds = ['Felix', 'Aneka', 'Alex', 'Max', 'Oliver', 'Mia', 'Sophia', 'Liam'];
const selectedSeed = ref('');

function setAvatarSeed(seed) {
  selectedSeed.value = seed;
  const newUrl = `https://api.dicebear.com/7.x/avataaars/svg?seed=${encodeURIComponent(seed)}`;
  if (authStore.user) {
    authStore.user.avatar_url = newUrl;
  }
}
</script>

<template>
  <div class="space-y-6">
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
</template>

