<script setup>
import { ref } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { useProjectStore } from '@/stores/project';
import { Sparkles, Lock, Mail, User, ArrowRight, ShieldCheck } from 'lucide-vue-next';

const authStore = useAuthStore();
const projectStore = useProjectStore();

const isRegister = ref(false);
const email = ref('');
const password = ref('');
const fullName = ref('');
const error = ref('');
const loading = ref(false);

async function handleSubmit() {
  error.value = '';
  loading.value = true;
  try {
    if (isRegister.value) {
      if (!fullName.value.trim()) throw new Error('Full name is required');
      await authStore.register(email.value.trim(), password.value, fullName.value.trim());
    } else {
      await authStore.login(email.value.trim(), password.value);
    }
    if (authStore.currentWorkspace) {
      await projectStore.fetchProjects(authStore.currentWorkspace.id);
    }
  } catch (err) {
    error.value = err.message || 'Authentication failed';
  } finally {
    loading.value = false;
  }
}

async function quickDemoLogin(demoEmail) {
  email.value = demoEmail;
  password.value = 'password123';
  isRegister.value = false;
  await handleSubmit();
}
</script>

<template>
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/40 backdrop-blur-sm">
    <div class="glass-modal w-full max-w-md rounded-2xl border border-slate-200 bg-white p-8 shadow-2xl animate-slide-up">
      <!-- Logo Header -->
      <div class="text-center mb-6">
        <div class="inline-flex p-3 rounded-2xl bg-blue-600 text-white shadow-xs mb-3">
          <img src="/src/assets/logo.svg" alt="EpicTask" class="w-8 h-8" />
        </div>
        <h1 class="text-xl font-bold tracking-tight text-slate-900">
          Epic<span class="text-blue-600">Task</span>
        </h1>
        <p class="text-xs text-slate-500 mt-1">High-Performance Project & Issue Tracking</p>
      </div>

      <!-- Quick 1-Click Demo Logins -->
      <div class="mb-6 p-3.5 bg-slate-50 border border-slate-200 rounded-xl space-y-2">
        <div class="flex items-center space-x-1.5 text-xs font-semibold text-slate-700">
          <Sparkles class="w-3.5 h-3.5 text-blue-600" />
          <span>Quick 1-Click Demo Logins</span>
        </div>
        <div class="grid grid-cols-2 gap-2 text-xs">
          <button
            @click="quickDemoLogin('sarah@epictask.dev')"
            class="p-2.5 bg-white hover:bg-slate-50 border border-slate-200 hover:border-slate-300 rounded-xl text-left transition shadow-2xs"
          >
            <p class="font-bold text-slate-800">👑 Sarah Jenkins</p>
            <p class="text-[10px] text-slate-500">Tech Lead / Owner</p>
          </button>
          <button
            @click="quickDemoLogin('alex@epictask.dev')"
            class="p-2.5 bg-white hover:bg-slate-50 border border-slate-200 hover:border-slate-300 rounded-xl text-left transition shadow-2xs"
          >
            <p class="font-bold text-slate-800">💻 Alex Morgan</p>
            <p class="text-[10px] text-slate-500">Fullstack Dev</p>
          </button>
        </div>
      </div>

      <!-- Error Message -->
      <div v-if="error" class="mb-4 p-3 bg-red-50 border border-red-200 rounded-xl text-xs text-red-600">
        {{ error }}
      </div>

      <!-- Form -->
      <form @submit.prevent="handleSubmit" class="space-y-4 text-xs">
        <div v-if="isRegister">
          <label class="block text-slate-600 font-semibold mb-1">Full Name</label>
          <div class="relative">
            <User class="w-4 h-4 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
            <input
              v-model="fullName"
              type="text"
              placeholder="e.g. John Doe"
              class="w-full bg-white border border-slate-300 rounded-xl pl-9 pr-3 py-2 text-xs text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs"
              required
            />
          </div>
        </div>

        <div>
          <label class="block text-slate-600 font-semibold mb-1">Email Address</label>
          <div class="relative">
            <Mail class="w-4 h-4 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
            <input
              v-model="email"
              type="email"
              placeholder="user@epictask.dev"
              class="w-full bg-white border border-slate-300 rounded-xl pl-9 pr-3 py-2 text-xs text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs"
              required
            />
          </div>
        </div>

        <div>
          <label class="block text-slate-600 font-semibold mb-1">Password</label>
          <div class="relative">
            <Lock class="w-4 h-4 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
            <input
              v-model="password"
              type="password"
              placeholder="••••••••"
              class="w-full bg-white border border-slate-300 rounded-xl pl-9 pr-3 py-2 text-xs text-slate-900 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs"
              required
            />
          </div>
        </div>

        <button
          type="submit"
          :disabled="loading"
          class="w-full py-2.5 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-xl font-bold text-xs shadow-xs flex items-center justify-center space-x-1.5 transition active:scale-95 disabled:opacity-50"
        >
          <span>{{ loading ? 'Processing...' : (isRegister ? 'Create Account' : 'Sign In') }}</span>
          <ArrowRight class="w-3.5 h-3.5" />
        </button>
      </form>

      <!-- Toggle Register / Sign In -->
      <div class="text-center mt-6 pt-4 border-t border-slate-100 text-xs text-slate-500">
        <span v-if="isRegister">Already have an account? </span>
        <span v-else>Don't have an account? </span>
        <button
          @click="isRegister = !isRegister; error = ''"
          class="text-blue-600 hover:text-blue-700 font-bold ml-1"
        >
          {{ isRegister ? 'Sign In' : 'Register' }}
        </button>
      </div>
    </div>
  </div>
</template>
