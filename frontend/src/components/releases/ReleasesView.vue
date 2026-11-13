<script setup>
import { ref, computed, onMounted } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useVersionStore } from '@/stores/version';
import {
  Rocket,
  Plus,
  Search,
  Filter,
  CheckCircle2,
  Clock,
  Archive,
  Layers,
  Sparkles
} from 'lucide-vue-next';
import ReleaseCard from './ReleaseCard.vue';
import CreateVersionModal from './CreateVersionModal.vue';
import ReleaseVersionModal from './ReleaseVersionModal.vue';

const projectStore = useProjectStore();
const versionStore = useVersionStore();

const activeFilterTab = ref('ALL'); // 'ALL' | 'UNRELEASED' | 'RELEASED' | 'ARCHIVED'
const searchQuery = ref('');

// Modals state
const isCreateModalOpen = ref(false);
const versionToEdit = ref(null);
const isReleaseModalOpen = ref(false);
const versionToRelease = ref(null);

onMounted(() => {
  if (projectStore.currentProject?.id) {
    versionStore.fetchVersions(projectStore.currentProject.id);
  }
});

// Stats
const totalCount = computed(() => versionStore.versions.length);
const unreleasedCount = computed(
  () => versionStore.versions.filter((v) => v.status === 'UNRELEASED').length
);
const releasedCount = computed(
  () => versionStore.versions.filter((v) => v.status === 'RELEASED').length
);
const archivedCount = computed(
  () => versionStore.versions.filter((v) => v.status === 'ARCHIVED').length
);

// Filtered versions
const filteredVersions = computed(() => {
  return versionStore.versions.filter((v) => {
    if (activeFilterTab.value !== 'ALL' && v.status !== activeFilterTab.value) {
      return false;
    }
    if (searchQuery.value.trim()) {
      const q = searchQuery.value.toLowerCase();
      const matchesName = v.name.toLowerCase().includes(q);
      const matchesDesc = (v.description || '').toLowerCase().includes(q);
      if (!matchesName && !matchesDesc) return false;
    }
    return true;
  });
});

function handleOpenCreate() {
  versionToEdit.value = null;
  isCreateModalOpen.value = true;
}

function handleOpenEdit(version) {
  versionToEdit.value = version;
  isCreateModalOpen.value = true;
}

function handleOpenRelease(version) {
  versionToRelease.value = version;
  isReleaseModalOpen.value = true;
}

async function handleToggleArchive(version) {
  await versionStore.archiveVersion(version.id);
}

async function handleDelete(version) {
  const confirmed = window.confirm(
    `Apakah Anda yakin ingin menghapus versi "${version.name}"? Tiket yang terhubung akan dilepaskan dari versi ini.`
  );
  if (confirmed) {
    await versionStore.deleteVersion(version.id);
  }
}
</script>

<template>
  <div class="flex-1 flex flex-col h-full bg-slate-50 overflow-y-auto">
    <!-- Top Header Bar -->
    <div class="bg-white border-b border-slate-200 px-6 py-4">
      <div class="flex items-center justify-between gap-4">
        <div>
          <div class="flex items-center gap-2">
            <h1 class="text-xl font-bold text-slate-800">Release Hub (Rilis & Versi)</h1>
            <span class="text-xs font-semibold px-2.5 py-0.5 rounded-full bg-indigo-50 text-indigo-700 border border-indigo-200">
              {{ projectStore.currentProject?.key || 'PROJ' }}
            </span>
          </div>
          <p class="text-xs text-slate-500 mt-0.5">
            Kelola siklus rilis fitur, milestone deployment, dan riwayat changelog versi
          </p>
        </div>

        <button
          @click="handleOpenCreate"
          class="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-xl text-xs font-semibold shadow-2xs transition"
        >
          <Plus class="w-4 h-4" />
          <span>Buat Versi Baru</span>
        </button>
      </div>

      <!-- Metric Cards Grid -->
      <div class="grid grid-cols-4 gap-4 mt-5">
        <!-- Card 1: Total -->
        <div class="bg-slate-50 border border-slate-200 rounded-xl p-3.5 flex items-center justify-between">
          <div>
            <p class="text-xs font-bold text-slate-500 uppercase tracking-wider">Total Versi</p>
            <p class="text-xl font-extrabold text-slate-800 mt-0.5 font-mono">{{ totalCount }}</p>
          </div>
          <div class="w-9 h-9 rounded-lg bg-white border border-slate-200 flex items-center justify-center text-slate-600">
            <Layers class="w-5 h-5" />
          </div>
        </div>

        <!-- Card 2: Unreleased -->
        <div class="bg-blue-50/60 border border-blue-200 rounded-xl p-3.5 flex items-center justify-between">
          <div>
            <p class="text-xs font-bold text-blue-700 uppercase tracking-wider">Belum Dirilis</p>
            <p class="text-xl font-extrabold text-blue-900 mt-0.5 font-mono">{{ unreleasedCount }}</p>
          </div>
          <div class="w-9 h-9 rounded-lg bg-white border border-blue-200 flex items-center justify-center text-blue-600">
            <Clock class="w-5 h-5" />
          </div>
        </div>

        <!-- Card 3: Released -->
        <div class="bg-emerald-50/60 border border-emerald-200 rounded-xl p-3.5 flex items-center justify-between">
          <div>
            <p class="text-xs font-bold text-emerald-700 uppercase tracking-wider">Sudah Dirilis</p>
            <p class="text-xl font-extrabold text-emerald-900 mt-0.5 font-mono">{{ releasedCount }}</p>
          </div>
          <div class="w-9 h-9 rounded-lg bg-white border border-emerald-200 flex items-center justify-center text-emerald-600">
            <CheckCircle2 class="w-5 h-5" />
          </div>
        </div>

        <!-- Card 4: Archived -->
        <div class="bg-slate-100/70 border border-slate-200 rounded-xl p-3.5 flex items-center justify-between">
          <div>
            <p class="text-xs font-bold text-slate-600 uppercase tracking-wider">Diarsipkan</p>
            <p class="text-xl font-extrabold text-slate-700 mt-0.5 font-mono">{{ archivedCount }}</p>
          </div>
          <div class="w-9 h-9 rounded-lg bg-white border border-slate-200 flex items-center justify-center text-slate-500">
            <Archive class="w-5 h-5" />
          </div>
        </div>
      </div>
    </div>

    <!-- Filter & Search Toolbar -->
    <div class="px-6 py-4 flex items-center justify-between gap-4">
      <!-- Tabs Filter -->
      <div class="inline-flex items-center bg-white p-1 rounded-xl border border-slate-200 shadow-2xs gap-1">
        <button
          @click="activeFilterTab = 'ALL'"
          class="px-3 py-1.5 rounded-lg text-xs font-semibold transition"
          :class="activeFilterTab === 'ALL' ? 'bg-blue-600 text-white shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          Semua ({{ totalCount }})
        </button>
        <button
          @click="activeFilterTab = 'UNRELEASED'"
          class="px-3 py-1.5 rounded-lg text-xs font-semibold transition"
          :class="activeFilterTab === 'UNRELEASED' ? 'bg-blue-600 text-white shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          Belum Dirilis ({{ unreleasedCount }})
        </button>
        <button
          @click="activeFilterTab = 'RELEASED'"
          class="px-3 py-1.5 rounded-lg text-xs font-semibold transition"
          :class="activeFilterTab === 'RELEASED' ? 'bg-blue-600 text-white shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          Sudah Dirilis ({{ releasedCount }})
        </button>
        <button
          @click="activeFilterTab = 'ARCHIVED'"
          class="px-3 py-1.5 rounded-lg text-xs font-semibold transition"
          :class="activeFilterTab === 'ARCHIVED' ? 'bg-blue-600 text-white shadow-2xs' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'"
        >
          Diarsipkan ({{ archivedCount }})
        </button>
      </div>

      <!-- Search Input -->
      <div class="relative w-64">
        <Search class="w-3.5 h-3.5 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Cari versi atau rilis..."
          class="w-full bg-white border border-slate-200 rounded-xl pl-9 pr-3 py-1.5 text-xs text-slate-800 placeholder-slate-400 focus:outline-none focus:border-blue-600 shadow-2xs font-medium"
        />
      </div>
    </div>

    <!-- Versions List Content -->
    <div class="px-6 pb-8 space-y-4">
      <div v-if="filteredVersions.length > 0" class="space-y-3.5">
        <ReleaseCard
          v-for="ver in filteredVersions"
          :key="ver.id"
          :version="ver"
          @edit="handleOpenEdit"
          @release="handleOpenRelease"
          @archive="handleToggleArchive"
          @delete="handleDelete"
        />
      </div>

      <!-- Empty State -->
      <div
        v-else
        class="bg-white border border-dashed border-slate-300 rounded-2xl p-12 text-center max-w-lg mx-auto mt-6 space-y-3 shadow-2xs"
      >
        <div class="w-12 h-12 rounded-xl bg-indigo-50 border border-indigo-100 text-indigo-600 flex items-center justify-center mx-auto">
          <Rocket class="w-6 h-6" />
        </div>
        <h3 class="text-base font-bold text-slate-800">Belum Ada Versi Rilis</h3>
        <p class="text-xs text-slate-500 max-w-sm mx-auto leading-relaxed">
          Buat versi rilis pertama untuk merencanakan paket peluncuran fitur, melacak progres implementasi, dan merilis update ke pengguna.
        </p>
        <div class="pt-2">
          <button
            @click="handleOpenCreate"
            class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-xl text-xs font-semibold shadow-2xs transition inline-flex items-center gap-1.5"
          >
            <Plus class="w-4 h-4" />
            <span>Buat Versi Sekarang</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Create / Edit Modal -->
    <CreateVersionModal
      :is-open="isCreateModalOpen"
      :version-to-edit="versionToEdit"
      :project-id="projectStore.currentProject?.id || 0"
      @close="isCreateModalOpen = false"
      @saved="versionStore.fetchVersions(projectStore.currentProject?.id)"
    />

    <!-- Release Confirmation Modal -->
    <ReleaseVersionModal
      :is-open="isReleaseModalOpen"
      :version="versionToRelease"
      :available-versions="versionStore.versions"
      @close="isReleaseModalOpen = false"
      @released="versionStore.fetchVersions(projectStore.currentProject?.id)"
    />
  </div>
</template>
