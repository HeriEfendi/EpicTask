<script setup>
import { ref, onMounted } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { useProjectStore } from '@/stores/project';
import { useNotificationStore } from '@/stores/notification';
import { wsClient } from '@/services/websocket';

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
import AuthModal from '@/components/auth/AuthModal.vue';

const authStore = useAuthStore();
const projectStore = useProjectStore();
const notifStore = useNotificationStore();

const isWorkflowModalOpen = ref(false);

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
  <div class="min-h-screen bg-slate-50 text-slate-900 flex flex-col font-sans">
    <!-- Unauthenticated State -->
    <AuthModal v-if="!authStore.isAuthenticated" />

    <!-- Authenticated App Shell -->
    <template v-else>
      <Navbar />

      <main class="flex-1 flex flex-col overflow-hidden">
        <ViewTabs @open-workflow-modal="isWorkflowModalOpen = true" />

        <div class="flex-1 flex flex-col overflow-hidden relative">
          <!-- Views Switcher (FR-2.2 Multi-View Board) -->
          <KanbanBoard v-if="projectStore.activeView === 'kanban'" />
          <TimelineView v-else-if="projectStore.activeView === 'timeline'" />
          <ListView v-else-if="projectStore.activeView === 'list'" />
        </div>
      </main>

      <!-- Modals & Overlays -->
      <IssueDetailModal />
      <CreateIssueModal />
      <AutomationModal />
      <WorkflowModal
        :is-open="isWorkflowModalOpen"
        @close="isWorkflowModalOpen = false"
      />
      <NotificationPopover />
    </template>
  </div>
</template>
