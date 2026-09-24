<script setup>
import { ref, onMounted } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { useProjectStore } from '@/stores/project';
import { useNotificationStore } from '@/stores/notification';
import { wsClient } from '@/services/websocket';

import Sidebar from '@/components/common/Sidebar.vue';
import Navbar from '@/components/common/Navbar.vue';
import ViewTabs from '@/components/common/ViewTabs.vue';
import KanbanBoard from '@/components/kanban/KanbanBoard.vue';
import TimelineView from '@/components/timeline/TimelineView.vue';
import ListView from '@/components/list/ListView.vue';
import IssueDetailModal from '@/components/issue/IssueDetailModal.vue';
import CreateIssueModal from '@/components/issue/CreateIssueModal.vue';
import AutomationModal from '@/components/automation/AutomationModal.vue';
import WorkflowModal from '@/components/project/WorkflowModal.vue';
import NotificationPopover from '@/components/notification/NotificationPopover.vue';
import SettingsModal from '@/components/common/SettingsModal.vue';
import AuthModal from '@/components/auth/AuthModal.vue';

const authStore = useAuthStore();
const projectStore = useProjectStore();
const notifStore = useNotificationStore();

// Sidebar only has 2 states: open (true) or hidden (false)
const isSidebarOpen = ref(true);
const isWorkflowModalOpen = ref(false);
const isSettingsModalOpen = ref(false);

onMounted(async () => {
  const ok = await authStore.fetchMe();
  if (ok) {
    wsClient.connect();
    if (authStore.currentWorkspace) {
      await projectStore.fetchProjects(authStore.currentWorkspace.id);
    }
  }
});
</script>

<template>
  <div class="h-screen bg-slate-50 text-slate-900 flex flex-col font-sans overflow-hidden">
    <!-- Unauthenticated State -->
    <AuthModal v-if="!authStore.isAuthenticated" />

    <!-- Authenticated App Layout -->
    <template v-else>
      <!-- Navbar: always full-width at top, never shifts with sidebar -->
      <Navbar
        :is-sidebar-open="isSidebarOpen"
        @toggle-sidebar="isSidebarOpen = !isSidebarOpen"
        @open-settings="isSettingsModalOpen = true"
      />

      <!-- Body row: Sidebar left + Main right, fills remaining height -->
      <div class="flex flex-1 overflow-hidden min-h-0">
        <!-- Sidebar: open = w-64, hidden = w-0 with overflow-hidden -->
        <Sidebar
          :is-open="isSidebarOpen"
          @open-workflow-modal="isWorkflowModalOpen = true"
        />

        <!-- Main content column -->
        <div class="flex-1 flex flex-col overflow-hidden min-w-0">
          <ViewTabs @open-workflow-modal="isWorkflowModalOpen = true" />

          <main class="flex-1 flex flex-col overflow-hidden relative">
            <KanbanBoard v-if="projectStore.activeView === 'kanban'" />
            <TimelineView v-else-if="projectStore.activeView === 'timeline'" />
            <ListView v-else-if="projectStore.activeView === 'list'" />
          </main>
        </div>
      </div>

      <!-- Modals & Overlays -->
      <IssueDetailModal />
      <CreateIssueModal />
      <AutomationModal />
      <WorkflowModal
        :is-open="isWorkflowModalOpen"
        @close="isWorkflowModalOpen = false"
      />
      <SettingsModal
        :is-open="isSettingsModalOpen"
        @close="isSettingsModalOpen = false"
      />
      <NotificationPopover />
    </template>
  </div>
</template>
