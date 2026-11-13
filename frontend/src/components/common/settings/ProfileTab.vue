<script setup>
import { ref, computed, onMounted } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { toast } from '@/utils/toast';
import {
  Sparkles,
  Bot,
  User,
  Upload,
  Check,
  RefreshCw,
  Image as ImageIcon,
  Palette
} from 'lucide-vue-next';

const authStore = useAuthStore();

// Sub-tabs: 'initials' | 'characters' | 'robots' | 'upload'
const activeCategory = ref('initials');

// 1. Initials state
const customInitials = ref('');

onMounted(() => {
  if (authStore.user?.full_name) {
    customInitials.value = computeDefaultInitials(authStore.user.full_name);
  }
});

function computeDefaultInitials(name) {
  if (!name) return 'ET';
  const clean = name.replace(/\([^)]*\)/g, '').trim(); // Remove roles in parentheses like (Tech Lead)
  const parts = clean.split(/\s+/).filter(Boolean);
  if (parts.length >= 2) {
    return (parts[0][0] + parts[1][0]).toUpperCase();
  } else if (parts.length === 1 && parts[0].length >= 2) {
    return parts[0].substring(0, 2).toUpperCase();
  }
  return (name[0] || 'U').toUpperCase();
}

const initialsColorPalettes = [
  { name: 'Indigo Royal', bg: '4f46e5', text: 'ffffff', ring: 'ring-indigo-500' },
  { name: 'Sky Blue', bg: '0284c7', text: 'ffffff', ring: 'ring-sky-500' },
  { name: 'Emerald Green', bg: '059669', text: 'ffffff', ring: 'ring-emerald-500' },
  { name: 'Violet Purple', bg: '7c3aed', text: 'ffffff', ring: 'ring-purple-500' },
  { name: 'Rose Red', bg: 'e11d48', text: 'ffffff', ring: 'ring-rose-500' },
  { name: 'Amber Warm', bg: 'd97706', text: 'ffffff', ring: 'ring-amber-500' },
  { name: 'Dark Slate', bg: '0f172a', text: 'ffffff', ring: 'ring-slate-700' },
  { name: 'Teal Ocean', bg: '0d9488', text: 'ffffff', ring: 'ring-teal-500' }
];

// 2. Character presets (24 diverse avatars)
const characterSeeds = [
  'Sarah', 'Alex', 'Elena', 'David', 'Felix', 'Aneka',
  'Max', 'Oliver', 'Mia', 'Sophia', 'Liam', 'Jack',
  'Zoe', 'Lucas', 'Emma', 'Noah', 'Chloe', 'Leo',
  'Ethan', 'Harper', 'Mason', 'Ella', 'Aiden', 'Maya'
];

// 3. Robot & Tech presets (16 cool bots)
const robotSeeds = [
  'Cyborg', 'Matrix', 'Sparky', 'Bender', 'Gizmo', 'Byte',
  'Pixel', 'Robo', 'Glitch', 'Bolt', 'Titan', 'Echo',
  'Neon', 'Rust', 'Cyber', 'Apex'
];

// 4. File upload state
const fileInput = ref(null);
const uploadedPreview = ref(null);
const isDragging = ref(false);

const isSaving = ref(false);

async function applyAvatar(newUrl) {
  if (!newUrl) return;
  isSaving.value = true;
  try {
    await authStore.updateProfile({ avatarUrl: newUrl });
    toast.success('Avatar berhasil diperbarui!');
  } catch (err) {
    toast.error('Gagal memperbarui avatar: ' + (err.message || 'Kesalahan server'));
  } finally {
    isSaving.value = false;
  }
}

function selectInitials(palette) {
  const seed = (customInitials.value || computeDefaultInitials(authStore.user?.full_name)).trim();
  const url = `https://api.dicebear.com/7.x/initials/svg?seed=${encodeURIComponent(seed)}&backgroundColor=${palette.bg}&textColor=${palette.text}&fontWeight=700`;
  applyAvatar(url);
}

function selectCharacter(seed) {
  const url = `https://api.dicebear.com/7.x/avataaars/svg?seed=${encodeURIComponent(seed)}`;
  applyAvatar(url);
}

function selectRobot(seed) {
  const url = `https://api.dicebear.com/7.x/bottts/svg?seed=${encodeURIComponent(seed)}`;
  applyAvatar(url);
}

function triggerFileSelect() {
  fileInput.value?.click();
}

function handleFileChange(event) {
  const file = event.target.files?.[0];
  if (file) processImageFile(file);
}

function handleDrop(event) {
  isDragging.value = false;
  const file = event.dataTransfer?.files?.[0];
  if (file) processImageFile(file);
}

function processImageFile(file) {
  if (!file.type.startsWith('image/')) {
    toast.error('Silakan pilih berkas gambar (PNG, JPG, WebP).');
    return;
  }
  if (file.size > 5 * 1024 * 1024) {
    toast.error('Ukuran gambar maksimal 5MB.');
    return;
  }

  const reader = new FileReader();
  reader.onload = (e) => {
    const img = new Image();
    img.onload = () => {
      // Auto-crop to square 256x256 using Canvas for high quality & fast loading
      const canvas = document.createElement('canvas');
      const size = 256;
      canvas.width = size;
      canvas.height = size;
      const ctx = canvas.getContext('2d');

      const minDim = Math.min(img.width, img.height);
      const sx = (img.width - minDim) / 2;
      const sy = (img.height - minDim) / 2;
      ctx.drawImage(img, sx, sy, minDim, minDim, 0, 0, size, size);

      const base64Url = canvas.toDataURL('image/jpeg', 0.88);
      uploadedPreview.value = base64Url;
    };
    img.src = e.target.result;
  };
  reader.readAsDataURL(file);
}

function applyUploadedImage() {
  if (uploadedPreview.value) {
    applyAvatar(uploadedPreview.value);
  }
}
</script>

<template>
  <div class="space-y-6">
    <!-- Current User Profile Card -->
    <div class="flex items-center space-x-4 p-4 sm:p-5 bg-white rounded-xl border border-slate-200 shadow-2xs">
      <div class="relative shrink-0">
        <img
          :src="authStore.user?.avatar_url || 'https://api.dicebear.com/7.x/avataaars/svg?seed=user'"
          class="w-16 h-16 rounded-full bg-slate-100 ring-3 ring-blue-500/80 shadow-xs object-cover"
          alt="Avatar saat ini"
        />
        <div
          v-if="isSaving"
          class="absolute inset-0 bg-slate-900/50 rounded-full flex items-center justify-center text-white"
        >
          <RefreshCw class="w-5 h-5 animate-spin" />
        </div>
      </div>

      <div class="flex-1 min-w-0">
        <div class="flex items-center space-x-2">
          <h3 class="text-base font-bold text-slate-900 truncate">
            {{ authStore.user?.full_name || 'Pengguna EpicTask' }}
          </h3>
          <span class="px-2 py-0.5 bg-blue-50 text-blue-700 border border-blue-200 text-[10px] font-bold rounded-full">
            Aktif
          </span>
        </div>
        <p class="text-xs text-slate-500 truncate mt-0.5">{{ authStore.user?.email }}</p>
        <span class="inline-block mt-1.5 px-2 py-0.5 bg-slate-100 text-slate-600 text-[11px] font-mono rounded border border-slate-200">
          User ID: #{{ authStore.user?.id }}
        </span>
      </div>
    </div>

    <!-- Avatar Picker Section with Tabs -->
    <div class="bg-white rounded-xl border border-slate-200/90 shadow-2xs overflow-hidden">
      <!-- Section Header & Navigation Tabs -->
      <div class="p-4 sm:p-5 border-b border-slate-100">
        <div class="flex items-center justify-between gap-4 mb-4">
          <div>
            <h4 class="text-sm font-bold text-slate-900 flex items-center space-x-2">
              <Sparkles class="w-4 h-4 text-blue-600" />
              <span>Pilihan Avatar & Karakter Profil</span>
            </h4>
            <p class="text-xs text-slate-500 mt-0.5">
              Pilih gaya inisial nama, karakter ilustrasi, robot tech, atau unggah foto dari perangkat Anda.
            </p>
          </div>
        </div>

        <!-- Category Tabs -->
        <div class="flex flex-wrap items-center gap-1.5 bg-slate-100/90 p-1 rounded-lg border border-slate-200/80 text-xs font-semibold">
          <button
            type="button"
            @click="activeCategory = 'initials'"
            class="px-3 py-1.5 rounded-md flex items-center space-x-1.5 transition"
            :class="activeCategory === 'initials' ? 'bg-white text-blue-700 shadow-2xs font-bold' : 'text-slate-600 hover:text-slate-900'"
          >
            <User class="w-3.5 h-3.5" />
            <span>Inisial Nama ({{ customInitials || 'SJ' }})</span>
          </button>

          <button
            type="button"
            @click="activeCategory = 'characters'"
            class="px-3 py-1.5 rounded-md flex items-center space-x-1.5 transition"
            :class="activeCategory === 'characters' ? 'bg-white text-blue-700 shadow-2xs font-bold' : 'text-slate-600 hover:text-slate-900'"
          >
            <Sparkles class="w-3.5 h-3.5" />
            <span>Karakter Ilustrasi (24)</span>
          </button>

          <button
            type="button"
            @click="activeCategory = 'robots'"
            class="px-3 py-1.5 rounded-md flex items-center space-x-1.5 transition"
            :class="activeCategory === 'robots' ? 'bg-white text-blue-700 shadow-2xs font-bold' : 'text-slate-600 hover:text-slate-900'"
          >
            <Bot class="w-3.5 h-3.5" />
            <span>Robot & Tech (16)</span>
          </button>

          <button
            type="button"
            @click="activeCategory = 'upload'"
            class="px-3 py-1.5 rounded-md flex items-center space-x-1.5 transition"
            :class="activeCategory === 'upload' ? 'bg-white text-blue-700 shadow-2xs font-bold' : 'text-slate-600 hover:text-slate-900'"
          >
            <Upload class="w-3.5 h-3.5" />
            <span>Unggah Foto Sendiri</span>
          </button>
        </div>
      </div>

      <!-- Tab Content Area -->
      <div class="p-4 sm:p-5">
        <!-- 1. Initials Avatar Tab -->
        <div v-if="activeCategory === 'initials'" class="space-y-4">
          <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 p-3 bg-slate-50/70 rounded-xl border border-slate-200">
            <div>
              <label class="block text-xs font-bold text-slate-800">Singkatan / Inisial Nama</label>
              <p class="text-[11px] text-slate-500">
                Secara otomatis diambil dari nama Anda, atau masukkan 2 huruf yang Anda inginkan (misal: "SJ").
              </p>
            </div>
            <div class="flex items-center space-x-2">
              <input
                type="text"
                v-model="customInitials"
                maxlength="3"
                placeholder="SJ"
                class="w-20 uppercase font-mono font-bold text-center text-sm px-3 py-1.5 bg-white border border-slate-300 rounded-lg focus:outline-hidden focus:ring-2 focus:ring-blue-500 shadow-2xs"
              />
            </div>
          </div>

          <div>
            <label class="block text-xs font-bold text-slate-700 mb-2">Pilih Warna Latar Inisial:</label>
            <div class="grid grid-cols-2 sm:grid-cols-4 lg:grid-cols-8 gap-3">
              <button
                v-for="p in initialsColorPalettes"
                :key="p.bg"
                type="button"
                @click="selectInitials(p)"
                :disabled="isSaving"
                class="p-2.5 rounded-xl border border-slate-200 hover:border-slate-300 hover:shadow-xs transition flex flex-col items-center justify-center group bg-white"
              >
                <img
                  :src="`https://api.dicebear.com/7.x/initials/svg?seed=${encodeURIComponent(customInitials || 'SJ')}&backgroundColor=${p.bg}&textColor=${p.text}&fontWeight=700`"
                  class="w-12 h-12 rounded-full shadow-2xs group-hover:scale-105 transition-transform"
                  alt="Avatar Inisial"
                />
                <span class="text-[11px] font-semibold text-slate-700 mt-2 truncate max-w-full">
                  {{ p.name }}
                </span>
              </button>
            </div>
          </div>
        </div>

        <!-- 2. Characters Tab (24 presets) -->
        <div v-else-if="activeCategory === 'characters'" class="space-y-3">
          <label class="block text-xs font-bold text-slate-700">Koleksi Karakter Ilustrasi Modern (24 Karakter):</label>
          <div class="grid grid-cols-3 sm:grid-cols-6 lg:grid-cols-8 gap-2.5">
            <button
              v-for="seed in characterSeeds"
              :key="seed"
              type="button"
              @click="selectCharacter(seed)"
              :disabled="isSaving"
              class="p-2 rounded-xl border border-slate-200 hover:border-blue-400 hover:bg-blue-50/40 hover:shadow-2xs transition flex flex-col items-center justify-center group bg-white"
            >
              <img
                :src="`https://api.dicebear.com/7.x/avataaars/svg?seed=${encodeURIComponent(seed)}`"
                class="w-11 h-11 rounded-full group-hover:scale-105 transition-transform bg-slate-50"
                :alt="seed"
              />
              <span class="text-[11px] font-medium text-slate-700 mt-1.5 truncate max-w-full">
                {{ seed }}
              </span>
            </button>
          </div>
        </div>

        <!-- 3. Robots Tab (16 presets) -->
        <div v-else-if="activeCategory === 'robots'" class="space-y-3">
          <label class="block text-xs font-bold text-slate-700">Koleksi Robot & Tech Avatars (16 Karakter):</label>
          <div class="grid grid-cols-2 sm:grid-cols-4 lg:grid-cols-8 gap-2.5">
            <button
              v-for="seed in robotSeeds"
              :key="seed"
              type="button"
              @click="selectRobot(seed)"
              :disabled="isSaving"
              class="p-2 rounded-xl border border-slate-200 hover:border-indigo-400 hover:bg-indigo-50/40 hover:shadow-2xs transition flex flex-col items-center justify-center group bg-white"
            >
              <img
                :src="`https://api.dicebear.com/7.x/bottts/svg?seed=${encodeURIComponent(seed)}`"
                class="w-11 h-11 rounded-full group-hover:scale-105 transition-transform bg-slate-50"
                :alt="seed"
              />
              <span class="text-[11px] font-medium text-slate-700 mt-1.5 truncate max-w-full">
                {{ seed }}
              </span>
            </button>
          </div>
        </div>

        <!-- 4. Upload Image Tab -->
        <div v-else-if="activeCategory === 'upload'" class="space-y-4">
          <input
            type="file"
            ref="fileInput"
            @change="handleFileChange"
            accept="image/png,image/jpeg,image/webp,image/gif"
            class="hidden"
          />

          <!-- Drag and Drop Box -->
          <div
            @dragover.prevent="isDragging = true"
            @dragleave.prevent="isDragging = false"
            @drop.prevent="handleDrop"
            @click="triggerFileSelect"
            class="border-2 border-dashed rounded-xl p-8 text-center cursor-pointer transition flex flex-col items-center justify-center"
            :class="isDragging ? 'border-blue-500 bg-blue-50/50' : 'border-slate-300 hover:border-slate-400 bg-slate-50/50 hover:bg-slate-50'"
          >
            <div class="w-12 h-12 rounded-full bg-blue-100 text-blue-600 flex items-center justify-center mb-3">
              <Upload class="w-6 h-6" />
            </div>
            <p class="text-sm font-bold text-slate-800">Klik untuk memilih gambar atau tarik berkas ke sini</p>
            <p class="text-xs text-slate-500 mt-1">Mendukung format PNG, JPG, WebP (Maksimal 5MB)</p>
          </div>

          <!-- Preview & Action if selected -->
          <div
            v-if="uploadedPreview"
            class="p-4 bg-emerald-50/60 rounded-xl border border-emerald-200 flex flex-col sm:flex-row sm:items-center justify-between gap-4"
          >
            <div class="flex items-center space-x-3">
              <img
                :src="uploadedPreview"
                class="w-14 h-14 rounded-full border-2 border-emerald-500 object-cover shadow-xs"
                alt="Pratinjau Foto"
              />
              <div>
                <h5 class="text-xs font-bold text-emerald-900">Foto Siap Digunakan</h5>
                <p class="text-[11px] text-emerald-700">Gambar telah dipotong persegi otomatis 256×256 px.</p>
              </div>
            </div>

            <div class="flex items-center space-x-2">
              <button
                type="button"
                @click="applyUploadedImage"
                :disabled="isSaving"
                class="px-4 py-2 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-xs font-bold flex items-center space-x-1.5 transition shadow-xs disabled:opacity-50"
              >
                <Check class="w-4 h-4" />
                <span>{{ isSaving ? 'Menyimpan...' : 'Gunakan Foto Ini' }}</span>
              </button>
              <button
                type="button"
                @click="uploadedPreview = null"
                class="px-3 py-2 bg-white hover:bg-slate-50 text-slate-600 border border-slate-300 rounded-lg text-xs font-semibold transition"
              >
                Batal
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
