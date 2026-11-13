<script setup>
import { ref, computed } from 'vue';
import { useProjectStore } from '@/stores/project';
import {
  Search,
  CheckCircle2,
  Clock,
  Zap,
  Calendar,
  Layers,
  ChevronRight,
  ExternalLink,
  Tag
} from 'lucide-vue-next';

const props = defineProps({
  issues: {
    type: Array,
    default: () => []
  }
});

const projectStore = useProjectStore();

const activeFilter = ref('ALL'); // 'ALL', 'DONE', 'ON_TIME', 'OVERDUE', 'IN_PROGRESS', 'TODO'
const searchQuery = ref('');

const filteredIssues = computed(() => {
  let list = props.issues || [];

  // Filter by status category or timeliness
  if (activeFilter.value === 'DONE') {
    list = list.filter((i) => i.status_category === 'DONE' || i.resolution || i.is_done);
  } else if (activeFilter.value === 'ON_TIME') {
    list = list.filter((i) => (i.status_category === 'DONE' || i.resolution || i.is_done) && i.is_on_time);
  } else if (activeFilter.value === 'OVERDUE') {
    list = list.filter((i) => (i.status_category === 'DONE' || i.resolution || i.is_done) && i.is_overdue);
  } else if (activeFilter.value === 'IN_PROGRESS') {
    list = list.filter((i) => i.status_category === 'IN_PROGRESS');
  } else if (activeFilter.value === 'TODO') {
    list = list.filter((i) => i.status_category === 'TODO');
  }

  // Filter by search query
  const q = searchQuery.value.toLowerCase().trim();
  if (q) {
    list = list.filter((i) => {
      return (
        (i.key && i.key.toLowerCase().includes(q)) ||
        (i.summary && i.summary.toLowerCase().includes(q)) ||
        (i.component_name && i.component_name.toLowerCase().includes(q)) ||
        (i.fix_version_name && i.fix_version_name.toLowerCase().includes(q)) ||
        (i.sprint_name && i.sprint_name.toLowerCase().includes(q)) ||
        (i.labels && i.labels.some((lbl) => lbl.toLowerCase().includes(q)))
      );
    });
  }

  return list;
});

const counts = computed(() => {
  const all = props.issues.length;
  const doneIssues = props.issues.filter((i) => i.status_category === 'DONE' || i.resolution || i.is_done);
  const done = doneIssues.length;
  const onTime = doneIssues.filter((i) => i.is_on_time).length;
  const overdue = doneIssues.filter((i) => i.is_overdue).length;
  const inProgress = props.issues.filter((i) => i.status_category === 'IN_PROGRESS').length;
  const todo = props.issues.filter((i) => i.status_category === 'TODO').length;
  return { all, done, onTime, overdue, inProgress, todo };
});

function formatDate(d) {
  if (!d) return '-';
  try {
    const dt = new Date(d);
    return dt.toLocaleDateString('id-ID', { year: 'numeric', month: 'short', day: 'numeric' });
  } catch (e) {
    return d;
  }
}

function getTypeBadgeClass(type) {
  switch (type?.toUpperCase()) {
    case 'BUG':
      return 'bg-red-50 text-red-700 border-red-200';
    case 'STORY':
      return 'bg-emerald-50 text-emerald-700 border-emerald-200';
    case 'EPIC':
      return 'bg-purple-50 text-purple-700 border-purple-200';
    case 'SUBTASK':
      return 'bg-cyan-50 text-cyan-700 border-cyan-200';
    default:
      return 'bg-blue-50 text-blue-700 border-blue-200';
  }
}

function getPriorityBadgeClass(prio) {
  switch (prio?.toUpperCase()) {
    case 'HIGHEST':
      return 'text-red-700 bg-red-50 border-red-200';
    case 'HIGH':
      return 'text-amber-700 bg-amber-50 border-amber-200';
    case 'MEDIUM':
      return 'text-blue-700 bg-blue-50 border-blue-200';
    default:
      return 'text-slate-600 bg-slate-50 border-slate-200';
  }
}
</script>

<template>
  <div class="bg-white rounded-xl border border-slate-200/90 shadow-2xs overflow-hidden">
    <!-- Header with Tabs and Search -->
    <div class="p-4 sm:p-5 border-b border-slate-200 flex flex-col md:flex-row md:items-center justify-between gap-4">
      <div>
        <h3 class="text-base font-bold text-slate-900 tracking-tight flex items-center space-x-2">
          <span>Daftar Tiket & Pekerjaan Yang Dikerjakan</span>
          <span class="text-xs font-bold px-2 py-0.5 rounded-full bg-slate-100 text-slate-600 border border-slate-200">
            {{ filteredIssues.length }} tiket
          </span>
        </h3>
        <p class="text-xs text-slate-500 mt-0.5">
          Klik pada kartu untuk membuka detail tiket, status, dan riwayat aktivitas secara langsung.
        </p>
      </div>

      <!-- Search and Filter Controls -->
      <div class="flex flex-wrap items-center gap-2.5">
        <!-- Search Input -->
        <div class="relative min-w-[220px]">
          <Search class="w-4 h-4 text-slate-400 absolute left-3 top-2.5 pointer-events-none" />
          <input
            type="text"
            v-model="searchQuery"
            placeholder="Cari tiket, summary, label..."
            class="w-full pl-9 pr-3 py-1.5 text-xs rounded-lg border border-slate-300 focus:outline-hidden focus:ring-2 focus:ring-blue-500 bg-white shadow-2xs text-slate-800"
          />
        </div>

        <!-- Filter Sub-Tabs -->
        <div class="flex flex-wrap items-center space-x-1 bg-slate-100 p-1 rounded-lg border border-slate-200/80 text-xs font-semibold">
          <button
            @click="activeFilter = 'ALL'"
            class="px-2.5 py-1 rounded-md transition"
            :class="activeFilter === 'ALL' ? 'bg-white text-blue-700 shadow-2xs font-bold' : 'text-slate-600 hover:text-slate-900'"
          >
            Semua ({{ counts.all }})
          </button>
          <button
            @click="activeFilter = 'DONE'"
            class="px-2.5 py-1 rounded-md transition"
            :class="activeFilter === 'DONE' ? 'bg-white text-emerald-700 shadow-2xs font-bold' : 'text-slate-600 hover:text-slate-900'"
          >
            Selesai ({{ counts.done }})
          </button>
          <button
            @click="activeFilter = 'ON_TIME'"
            class="px-2.5 py-1 rounded-md transition flex items-center space-x-1"
            :class="activeFilter === 'ON_TIME' ? 'bg-white text-emerald-700 shadow-2xs font-bold' : 'text-slate-600 hover:text-slate-900'"
          >
            <span>✓ Tepat Waktu ({{ counts.onTime }})</span>
          </button>
          <button
            v-if="counts.overdue > 0"
            @click="activeFilter = 'OVERDUE'"
            class="px-2.5 py-1 rounded-md transition flex items-center space-x-1"
            :class="activeFilter === 'OVERDUE' ? 'bg-white text-rose-700 shadow-2xs font-bold' : 'text-rose-600 hover:text-rose-800'"
          >
            <span>⚠️ Overdue ({{ counts.overdue }})</span>
          </button>
          <button
            @click="activeFilter = 'IN_PROGRESS'"
            class="px-2.5 py-1 rounded-md transition"
            :class="activeFilter === 'IN_PROGRESS' ? 'bg-white text-amber-700 shadow-2xs font-bold' : 'text-slate-600 hover:text-slate-900'"
          >
            Proses ({{ counts.inProgress }})
          </button>
          <button
            @click="activeFilter = 'TODO'"
            class="px-2.5 py-1 rounded-md transition"
            :class="activeFilter === 'TODO' ? 'bg-white text-slate-700 shadow-2xs font-bold' : 'text-slate-600 hover:text-slate-900'"
          >
            To Do ({{ counts.todo }})
          </button>
        </div>
      </div>
    </div>

    <!-- Cards List Container -->
    <div class="divide-y divide-slate-100">
      <div
        v-for="issue in filteredIssues"
        :key="issue.id"
        @click="projectStore.openIssueDetail(issue.id)"
        class="p-4 sm:p-5 hover:bg-slate-50/80 transition cursor-pointer group flex flex-col md:flex-row md:items-center justify-between gap-4"
      >
        <!-- Left Side: Type, Key, Summary, Badges -->
        <div class="flex-1 min-w-0">
          <!-- Top Badges Row -->
          <div class="flex flex-wrap items-center gap-2 mb-1.5">
            <span
              class="text-[10px] font-bold px-2 py-0.5 rounded-md border uppercase tracking-wider"
              :class="getTypeBadgeClass(issue.issue_type)"
            >
              {{ issue.issue_type }}
            </span>

            <!-- Jira Key with strikethrough if resolved -->
            <span
              class="text-xs font-bold font-mono group-hover:text-blue-600 transition"
              :class="(issue.status_category === 'DONE' || issue.resolution) ? 'line-through text-slate-400' : 'text-slate-700'"
            >
              {{ issue.key }}
            </span>

            <!-- Priority Badge -->
            <span
              class="text-[10px] font-bold px-1.5 py-0.5 rounded-md border"
              :class="getPriorityBadgeClass(issue.priority)"
            >
              {{ issue.priority }}
            </span>

            <!-- Component Badge -->
            <span
              v-if="issue.component_name"
              class="text-[10px] font-semibold px-2 py-0.5 rounded-md bg-cyan-50 text-cyan-800 border border-cyan-200"
            >
              🧩 {{ issue.component_name }}
            </span>

            <!-- Fix Version Badge -->
            <span
              v-if="issue.fix_version_name"
              class="text-[10px] font-semibold px-2 py-0.5 rounded-md bg-indigo-50 text-indigo-800 border border-indigo-200"
            >
              🚀 {{ issue.fix_version_name }}
            </span>

            <!-- Sprint Badge -->
            <span
              v-if="issue.sprint_name"
              class="text-[10px] font-semibold px-2 py-0.5 rounded-md bg-purple-50 text-purple-800 border border-purple-200"
            >
              🏃 {{ issue.sprint_name }}
            </span>

            <!-- Timeliness Badge (On-Time vs Overdue) -->
            <span
              v-if="issue.is_done && issue.is_overdue"
              class="text-[10px] font-bold px-2 py-0.5 rounded-md bg-rose-50 text-rose-700 border border-rose-200"
            >
              ⚠️ Overdue +{{ issue.days_overdue }} hari
            </span>
            <span
              v-else-if="issue.is_done && issue.is_on_time"
              class="text-[10px] font-bold px-2 py-0.5 rounded-md bg-emerald-50 text-emerald-700 border border-emerald-200"
            >
              ✓ Selesai Tepat Waktu
            </span>
            <span
              v-else-if="!issue.is_done && issue.is_overdue"
              class="text-[10px] font-bold px-2 py-0.5 rounded-md bg-amber-50 text-amber-700 border border-amber-200"
            >
              ⏳ Melewati Tenggat (+{{ issue.days_overdue }}h)
            </span>
          </div>

          <!-- Summary Title -->
          <h4 class="text-sm font-bold text-slate-800 group-hover:text-blue-600 transition leading-snug line-clamp-2">
            {{ issue.summary }}
          </h4>

          <!-- Labels and Timestamps -->
          <div class="flex flex-wrap items-center gap-2 mt-2 text-xs text-slate-500">
            <!-- Labels -->
            <div v-if="issue.labels && issue.labels.length > 0" class="flex items-center space-x-1">
              <span
                v-for="lbl in issue.labels"
                :key="lbl"
                class="text-[10px] font-medium px-1.5 py-0.5 rounded bg-slate-100 text-slate-600 border border-slate-200"
              >
                #{{ lbl }}
              </span>
            </div>

            <!-- Date info -->
            <div class="flex items-center space-x-1 text-[11px] text-slate-400">
              <Calendar class="w-3 h-3 text-slate-400 shrink-0" />
              <span v-if="issue.resolved_at">
                Diselesaikan: <strong class="text-slate-600 font-semibold">{{ formatDate(issue.resolved_at) }}</strong>
              </span>
              <span v-else-if="issue.updated_at">
                Diperbarui: <strong class="text-slate-600 font-semibold">{{ formatDate(issue.updated_at) }}</strong>
              </span>
              <span v-if="issue.due_date" class="ml-2 pl-2 border-l border-slate-200 text-slate-500">
                Tenggat: <strong :class="issue.is_overdue ? 'text-rose-600' : 'text-slate-600'">{{ formatDate(issue.due_date) }}</strong>
              </span>
            </div>
          </div>
        </div>

        <!-- Right Side: Metrics, Status & Time Logged -->
        <div class="flex items-center space-x-4 shrink-0 border-t md:border-t-0 pt-2 md:pt-0 border-slate-100">
          <!-- Story Points Pill -->
          <div v-if="issue.story_points" class="text-right">
            <span class="text-xs font-bold px-2.5 py-1 rounded-full bg-purple-50 text-purple-700 border border-purple-200 flex items-center space-x-1 shadow-2xs">
              <Zap class="w-3 h-3 text-purple-600" />
              <span>{{ issue.story_points }} pts</span>
            </span>
          </div>

          <!-- Hours Logged Pill -->
          <div v-if="issue.user_logged_hours > 0" class="text-right">
            <span class="text-xs font-bold px-2.5 py-1 rounded-full bg-blue-50 text-blue-700 border border-blue-200 flex items-center space-x-1 shadow-2xs">
              <Clock class="w-3 h-3 text-blue-600" />
              <span>{{ issue.user_logged_hours.toFixed(1) }}h log</span>
            </span>
          </div>

          <!-- Status Badge -->
          <div class="text-right min-w-[90px]">
            <span
              class="inline-block text-xs font-bold px-2.5 py-1 rounded-full border shadow-2xs whitespace-nowrap"
              :style="{
                backgroundColor: `${issue.status_color || '#64748b'}15`,
                borderColor: `${issue.status_color || '#64748b'}40`,
                color: issue.status_color || '#475569'
              }"
            >
              {{ issue.status_name }}
            </span>
            <div v-if="issue.resolution" class="text-[10px] text-emerald-600 font-bold mt-0.5">
              ✓ {{ issue.resolution }}
            </div>
          </div>

          <!-- Action Chevron Icon -->
          <div class="text-slate-400 group-hover:text-blue-600 group-hover:translate-x-0.5 transition">
            <ChevronRight class="w-5 h-5" />
          </div>
        </div>
      </div>

      <!-- Empty State -->
      <div v-if="filteredIssues.length === 0" class="py-12 text-center">
        <Layers class="w-8 h-8 text-slate-300 mx-auto mb-2" />
        <p class="text-sm font-semibold text-slate-700">Tidak ada tiket yang ditemukan</p>
        <p class="text-xs text-slate-400 mt-0.5">
          Coba ganti filter status atau sesuaikan periode tanggal pencarian.
        </p>
      </div>
    </div>
  </div>
</template>

