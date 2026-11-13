<script setup>
import { computed } from 'vue';
import { useProjectStore } from '@/stores/project';
import { useAuthStore } from '@/stores/auth';
import {
  Flame,
  User,
  HelpCircle,
  FolderGit2,
  AlertTriangle,
  ArrowUp,
  ArrowRight,
  ArrowDown
} from 'lucide-vue-next';
import KanbanSwimlaneRow from './KanbanSwimlaneRow.vue';

const props = defineProps({
  statuses: {
    type: Array,
    required: true,
  },
  issues: {
    type: Array,
    required: true,
  },
  dragOverCellKey: {
    type: String,
    default: null,
  },
});

const emit = defineEmits(['dragover', 'dragleave', 'drop', 'create-issue']);

const projectStore = useProjectStore();
const authStore = useAuthStore();

// Column counts across all swimlanes
const columnCounts = computed(() => {
  const counts = {};
  props.statuses.forEach((s) => {
    counts[s.id] = 0;
  });
  props.issues.forEach((i) => {
    if (counts[i.status_id] !== undefined) {
      counts[i.status_id]++;
    }
  });
  return counts;
});

// Swimlane groups calculation based on mode
const swimlaneGroups = computed(() => {
  const mode = projectStore.swimlaneMode;

  if (mode === 'assignee') {
    // 1. Group by Assignee
    const userMap = new Map();
    const unassigned = [];

    props.issues.forEach((issue) => {
      if (!issue.assignee_id) {
        unassigned.push(issue);
      } else {
        if (!userMap.has(issue.assignee_id)) {
          userMap.set(issue.assignee_id, {
            user_id: issue.assignee_id,
            name: issue.assignee_name || 'Anggota Tim',
            avatar: issue.assignee_avatar || `https://api.dicebear.com/7.x/avataaars/svg?seed=${issue.assignee_id}`,
            issues: [],
          });
        }
        userMap.get(issue.assignee_id).issues.push(issue);
      }
    });

    const groups = Array.from(userMap.values()).map((u) => ({
      key: `user-${u.user_id}`,
      title: u.name,
      subtitle: `${u.issues.length} tiket`,
      badgeColor: 'bg-blue-100 text-blue-800',
      avatarUrl: u.avatar,
      isExpedite: false,
      issues: u.issues,
    }));

    if (unassigned.length > 0) {
      groups.push({
        key: 'unassigned',
        title: 'Belum Ditugaskan (Unassigned)',
        subtitle: `${unassigned.length} tiket`,
        badgeColor: 'bg-slate-200 text-slate-700',
        icon: HelpCircle,
        isExpedite: false,
        issues: unassigned,
      });
    }

    return groups;
  }

  if (mode === 'epic') {
    // 2. Group by Epic
    const epicMap = new Map();
    const noEpic = [];

    props.issues.forEach((issue) => {
      if (issue.issue_type === 'EPIC') {
        // Epics themselves are usually parents, but if shown, keep them or group them
        return;
      }
      if (!issue.epic_id) {
        noEpic.push(issue);
      } else {
        if (!epicMap.has(issue.epic_id)) {
          epicMap.set(issue.epic_id, {
            epic_id: issue.epic_id,
            summary: issue.epic_summary || `Epic #${issue.epic_id}`,
            issues: [],
          });
        }
        epicMap.get(issue.epic_id).issues.push(issue);
      }
    });

    const groups = Array.from(epicMap.values()).map((ep) => ({
      key: `epic-${ep.epic_id}`,
      title: ep.summary,
      subtitle: '🟣 Epic',
      badgeColor: 'bg-purple-100 text-purple-800',
      icon: FolderGit2,
      isExpedite: false,
      issues: ep.issues,
    }));

    if (noEpic.length > 0) {
      groups.push({
        key: 'no-epic',
        title: 'Tiket Tanpa Epic',
        subtitle: `${noEpic.length} tiket`,
        badgeColor: 'bg-slate-200 text-slate-700',
        icon: HelpCircle,
        isExpedite: false,
        issues: noEpic,
      });
    }

    return groups;
  }

  if (mode === 'priority') {
    // 3. Group by Priority (Expedite at Top)
    const expedite = [];
    const high = [];
    const medium = [];
    const low = [];

    props.issues.forEach((issue) => {
      const p = (issue.priority || 'MEDIUM').toUpperCase();
      if (p === 'HIGHEST') {
        expedite.push(issue);
      } else if (p === 'HIGH') {
        high.push(issue);
      } else if (p === 'MEDIUM') {
        medium.push(issue);
      } else {
        low.push(issue);
      }
    });

    const groups = [];

    // Expedite Swimlane (Jira Highlighted Priority Lane)
    if (expedite.length > 0) {
      groups.push({
        key: 'priority-highest',
        title: '🚨 Expedite (Paling Mendesak)',
        subtitle: 'Prioritas Tertinggi',
        badgeColor: 'bg-rose-100 text-rose-800 font-bold',
        icon: Flame,
        isExpedite: true,
        issues: expedite,
      });
    }

    if (high.length > 0) {
      groups.push({
        key: 'priority-high',
        title: '🟠 Prioritas Tinggi',
        subtitle: `${high.length} tiket`,
        badgeColor: 'bg-amber-100 text-amber-800',
        icon: ArrowUp,
        isExpedite: false,
        issues: high,
      });
    }

    if (medium.length > 0) {
      groups.push({
        key: 'priority-medium',
        title: '🟡 Prioritas Sedang',
        subtitle: `${medium.length} tiket`,
        badgeColor: 'bg-yellow-100 text-yellow-800',
        icon: ArrowRight,
        isExpedite: false,
        issues: medium,
      });
    }

    if (low.length > 0) {
      groups.push({
        key: 'priority-low',
        title: '🔵 Prioritas Rendah',
        subtitle: `${low.length} tiket`,
        badgeColor: 'bg-blue-100 text-blue-800',
        icon: ArrowDown,
        isExpedite: false,
        issues: low,
      });
    }

    return groups;
  }

  return [];
});
</script>

<template>
  <div class="flex-1 flex flex-col min-w-max pb-6">
    <!-- Sticky Column Headers (Synchronized with Swimlane columns below) -->
    <div class="sticky top-0 z-20 bg-slate-50/95 backdrop-blur-xs pb-3 pt-1">
      <div class="flex gap-5 items-center px-1">
        <div
          v-for="status in statuses"
          :key="status.id"
          class="w-80 shrink-0 bg-white rounded-xl border border-slate-200/90 px-3.5 py-2.5 flex items-center justify-between shadow-2xs select-none"
        >
          <div class="flex items-center space-x-2">
            <span
              class="w-2.5 h-2.5 rounded-full"
              :style="{ backgroundColor: status.color || '#3b82f6' }"
            ></span>
            <span class="text-xs font-bold text-slate-800 uppercase tracking-wide">
              {{ status.name }}
            </span>
          </div>
          <span class="text-xs font-mono font-semibold px-2 py-0.5 rounded-full bg-slate-100 text-slate-600 border border-slate-200">
            {{ columnCounts[status.id] || 0 }}
          </span>
        </div>
      </div>
    </div>

    <!-- Swimlane Rows -->
    <div class="space-y-4 pt-1">
      <KanbanSwimlaneRow
        v-for="grp in swimlaneGroups"
        :key="grp.key"
        :swimlane-key="grp.key"
        :title="grp.title"
        :subtitle="grp.subtitle"
        :badge-color="grp.badgeColor"
        :avatar-url="grp.avatarUrl"
        :icon="grp.icon"
        :is-expedite="grp.isExpedite"
        :statuses="statuses"
        :issues="grp.issues"
        :drag-over-column-id="dragOverCellKey"
        @dragover="(payload) => emit('dragover', payload)"
        @dragleave="(payload) => emit('dragleave', payload)"
        @drop="(payload) => emit('drop', payload)"
        @create-issue="(sid) => emit('create-issue', sid)"
      />

      <!-- Empty Swimlane State -->
      <div
        v-if="swimlaneGroups.length === 0"
        class="bg-white border border-dashed border-slate-300 rounded-2xl p-8 text-center text-slate-500 text-sm"
      >
        Tidak ada tiket yang cocok dengan kriteria swimlane ini.
      </div>
    </div>
  </div>
</template>
