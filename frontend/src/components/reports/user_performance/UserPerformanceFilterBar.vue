<script setup>
import { ref, computed } from 'vue';
import {
  User,
  Calendar,
  RefreshCw,
  Download,
  Filter,
  ChevronDown
} from 'lucide-vue-next';

const props = defineProps({
  availableUsers: {
    type: Array,
    default: () => []
  },
  selectedUser: {
    type: Object,
    default: null
  },
  loading: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['filter-changed', 'refresh', 'export-csv']);

const activePreset = ref('all'); // 'all', '7d', '30d', 'this_month', 'custom'
const startDate = ref('');
const endDate = ref('');
const isUserDropdownOpen = ref(false);

function applyPreset(preset) {
  activePreset.value = preset;
  const now = new Date();
  const formatYmd = (d) => d.toISOString().split('T')[0];

  if (preset === 'all') {
    startDate.value = '';
    endDate.value = '';
  } else if (preset === '7d') {
    const start = new Date();
    start.setDate(now.getDate() - 7);
    startDate.value = formatYmd(start);
    endDate.value = formatYmd(now);
  } else if (preset === '30d') {
    const start = new Date();
    start.setDate(now.getDate() - 30);
    startDate.value = formatYmd(start);
    endDate.value = formatYmd(now);
  } else if (preset === 'this_month') {
    const start = new Date(now.getFullYear(), now.getMonth(), 1);
    startDate.value = formatYmd(start);
    endDate.value = formatYmd(now);
  }

  triggerChange();
}

function handleCustomDateApply() {
  activePreset.value = 'custom';
  triggerChange();
}

function selectUser(user) {
  isUserDropdownOpen.value = false;
  emit('filter-changed', {
    userId: user.id,
    startDate: startDate.value || null,
    endDate: endDate.value || null
  });
}

function triggerChange() {
  emit('filter-changed', {
    userId: props.selectedUser?.id || null,
    startDate: startDate.value || null,
    endDate: endDate.value || null
  });
}
</script>

<template>
  <div class="bg-white rounded-xl border border-slate-200/90 shadow-2xs p-4 sm:p-5">
    <div class="flex flex-col lg:flex-row lg:items-center justify-between gap-4">
      <!-- Left: User Selection Dropdown -->
      <div class="flex flex-wrap items-center gap-3">
        <div class="relative">
          <label class="block text-[11px] font-bold uppercase tracking-wider text-slate-500 mb-1">
            Pilih Anggota Tim
          </label>
          <button
            type="button"
            @click="isUserDropdownOpen = !isUserDropdownOpen"
            class="flex items-center space-x-2.5 px-3.5 py-2 rounded-lg border border-slate-300 hover:border-slate-400 bg-white text-left transition shadow-2xs min-w-[240px]"
          >
            <img
              v-if="selectedUser?.avatar_url"
              :src="selectedUser.avatar_url"
              class="w-6 h-6 rounded-full border border-slate-200 object-cover"
              alt="Avatar"
            />
            <div
              v-else
              class="w-6 h-6 rounded-full bg-blue-100 text-blue-700 flex items-center justify-center font-bold text-xs"
            >
              {{ selectedUser?.full_name ? selectedUser.full_name.charAt(0) : 'U' }}
            </div>
            <div class="flex-1 truncate">
              <span class="text-xs font-bold text-slate-800 block truncate">
                {{ selectedUser?.full_name || 'Pilih Anggota' }}
              </span>
              <span class="text-[10px] text-slate-500 block truncate leading-none">
                {{ selectedUser?.email || 'Semua tugas' }}
              </span>
            </div>
            <ChevronDown class="w-4 h-4 text-slate-400 shrink-0" />
          </button>

          <!-- Dropdown Menu -->
          <div
            v-if="isUserDropdownOpen"
            class="absolute left-0 mt-1 w-72 bg-white rounded-xl shadow-xl border border-slate-200 py-1.5 z-40 max-h-80 overflow-y-auto"
          >
            <div class="px-3 py-1.5 text-[11px] font-bold text-slate-400 uppercase tracking-wider">
              Anggota Proyek ({{ availableUsers.length }})
            </div>
            <button
              v-for="u in availableUsers"
              :key="u.id"
              @click="selectUser(u)"
              class="w-full flex items-center space-x-2.5 px-3 py-2 text-left hover:bg-slate-50 transition"
              :class="{ 'bg-blue-50/70 text-blue-700 font-semibold': selectedUser?.id === u.id }"
            >
              <img
                :src="u.avatar_url || `https://api.dicebear.com/7.x/avataaars/svg?seed=${u.full_name}`"
                class="w-7 h-7 rounded-full border border-slate-200 object-cover shrink-0"
                alt="Avatar"
              />
              <div class="flex-1 min-w-0">
                <div class="text-xs font-semibold text-slate-800 truncate">{{ u.full_name }}</div>
                <div class="text-[11px] text-slate-400 truncate">{{ u.email }}</div>
              </div>
              <span
                v-if="u.assigned_count > 0"
                class="text-[10px] font-bold px-1.5 py-0.5 rounded-full bg-slate-100 text-slate-600 border border-slate-200"
              >
                {{ u.assigned_count }} tiket
              </span>
            </button>
          </div>
        </div>

        <!-- Date Range Filter Presets -->
        <div>
          <label class="block text-[11px] font-bold uppercase tracking-wider text-slate-500 mb-1">
            Periode Laporan
          </label>
          <div class="flex items-center space-x-1 bg-slate-100/80 p-1 rounded-lg border border-slate-200/80">
            <button
              type="button"
              @click="applyPreset('all')"
              class="px-2.5 py-1 text-xs font-semibold rounded-md transition"
              :class="activePreset === 'all' ? 'bg-white text-blue-700 shadow-2xs font-bold' : 'text-slate-600 hover:text-slate-900'"
            >
              Semua Waktu
            </button>
            <button
              type="button"
              @click="applyPreset('7d')"
              class="px-2.5 py-1 text-xs font-semibold rounded-md transition"
              :class="activePreset === '7d' ? 'bg-white text-blue-700 shadow-2xs font-bold' : 'text-slate-600 hover:text-slate-900'"
            >
              7 Hari
            </button>
            <button
              type="button"
              @click="applyPreset('30d')"
              class="px-2.5 py-1 text-xs font-semibold rounded-md transition"
              :class="activePreset === '30d' ? 'bg-white text-blue-700 shadow-2xs font-bold' : 'text-slate-600 hover:text-slate-900'"
            >
              30 Hari
            </button>
            <button
              type="button"
              @click="applyPreset('this_month')"
              class="px-2.5 py-1 text-xs font-semibold rounded-md transition"
              :class="activePreset === 'this_month' ? 'bg-white text-blue-700 shadow-2xs font-bold' : 'text-slate-600 hover:text-slate-900'"
            >
              Bulan Ini
            </button>
          </div>
        </div>

        <!-- Custom Date Range Inputs -->
        <div class="flex items-end space-x-2">
          <div>
            <label class="block text-[11px] font-bold uppercase tracking-wider text-slate-500 mb-1">
              Dari Tanggal
            </label>
            <input
              type="date"
              v-model="startDate"
              @change="handleCustomDateApply"
              class="px-2.5 py-1.5 text-xs rounded-lg border border-slate-300 bg-white text-slate-700 focus:outline-hidden focus:ring-2 focus:ring-blue-500 shadow-2xs"
            />
          </div>
          <div>
            <label class="block text-[11px] font-bold uppercase tracking-wider text-slate-500 mb-1">
              Sampai
            </label>
            <input
              type="date"
              v-model="endDate"
              @change="handleCustomDateApply"
              class="px-2.5 py-1.5 text-xs rounded-lg border border-slate-300 bg-white text-slate-700 focus:outline-hidden focus:ring-2 focus:ring-blue-500 shadow-2xs"
            />
          </div>
        </div>
      </div>

      <!-- Right: Action Buttons -->
      <div class="flex items-center space-x-2 self-end lg:self-center">
        <button
          type="button"
          @click="emit('refresh')"
          :disabled="loading"
          class="px-3 py-1.5 rounded-lg border border-slate-300 hover:border-slate-400 bg-white hover:bg-slate-50 text-slate-700 text-xs font-semibold flex items-center space-x-1.5 transition shadow-2xs disabled:opacity-50"
          title="Segarkan metrik kinerja anggota ini"
        >
          <RefreshCw class="w-3.5 h-3.5" :class="{ 'animate-spin': loading }" />
          <span>Muat Ulang</span>
        </button>

        <button
          type="button"
          @click="emit('export-csv')"
          class="px-3.5 py-1.5 rounded-lg bg-emerald-600 hover:bg-emerald-700 text-white text-xs font-semibold flex items-center space-x-1.5 transition shadow-xs"
          title="Unduh laporan kinerja user ke file CSV"
        >
          <Download class="w-3.5 h-3.5" />
          <span>Ekspor CSV</span>
        </button>
      </div>
    </div>
  </div>
</template>

