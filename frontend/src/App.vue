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

const isSidebarCollapsed = ref(false);
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
  <div class="h-screen bg-slate-50 text-slate-900 flex font-sans overflow-hidden">
    <!-- Unauthenticated State -->
    <AuthModal v-if="!authStore.isAuthenticated" />

    <!-- Authenticated App Layout with Sidebar -->
    <template v-else>
      <!-- Side Menu -->
      <Sidebar
        :is-collapsed="isSidebarCollapsed"
        @toggle-collapse="isSidebarCollapsed = !isSidebarCollapsed"
        @open-workflow-modal="isWorkflowModalOpen = true"
      />

      <!-- Main Application Container -->
      <div class="flex-1 flex flex-col overflow-hidden min-w-0">
        <!-- Streamlined Header -->
        <Navbar
          :is-sidebar-collapsed="isSidebarCollapsed"
          @toggle-sidebar="isSidebarCollapsed = !isSidebarCollapsed"
          @open-settings="isSettingsModalOpen = true"
        />

        <!-- Secondary Filter/View Tabs -->
        <ViewTabs @open-workflow-modal="isWorkflowModalOpen = true" />

        <!-- Views Viewport -->
        <main class="flex-1 flex flex-col overflow-hidden relative">
          <KanbanBoard v-if="projectStore.activeView === 'kanban'" />
          <TimelineView v-else-if="projectStore.activeView === 'timeline'" />
          <ListView v-else-if="projectStore.activeView === 'list'" />
        </main>
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
