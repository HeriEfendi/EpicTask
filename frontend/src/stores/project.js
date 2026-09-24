import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { api } from '@/services/api';
import { wsClient } from '@/services/websocket';
import { sendDesktopNotification } from '@/services/tauri';

export const useProjectStore = defineStore('project', () => {
  const projects = ref([]);
  const currentProject = ref(null);
  const statuses = ref([]);
  const transitions = ref([]);
  const issues = ref([]);
  const activeView = ref('kanban'); // 'kanban' | 'timeline' | 'list'
  const loading = ref(false);
  const error = ref(null);

  // Filters
  const searchQuery = ref('');
  const filterAssignee = ref(null);
  const filterPriority = ref(null);
  const filterEpic = ref(null);
  const filterType = ref(null);

  // Issue Detail Modal State
  const activeIssue = ref(null);
  const activeIssueSubtasks = ref([]);
  const activeIssueLinks = ref([]);
  const activeIssueComments = ref([]);
  const activeIssueTimeLogs = ref([]);
  const totalTimeSpentSeconds = ref(0);
  const isDetailModalOpen = ref(false);

  // Quick Create Modal State
  const isCreateModalOpen = ref(false);
  const defaultCreateStatusId = ref(null);
  const defaultCreateParentId = ref(null);

  // Filtered issues
  const filteredIssues = computed(() => {
    return issues.value.filter((issue) => {
      if (searchQuery.value.trim()) {
        const q = searchQuery.value.toLowerCase();
        const matchesSummary = issue.summary.toLowerCase().includes(q);
        const matchesKey = issue.key.toLowerCase().includes(q);
        if (!matchesSummary && !matchesKey) return false;
      }
      if (filterAssignee.value && issue.assignee_id !== filterAssignee.value) {
        return false;
      }
      if (filterPriority.value && issue.priority !== filterPriority.value) {
        return false;
      }
      if (filterType.value && issue.issue_type !== filterType.value) {
        return false;
      }
      if (filterEpic.value && issue.epic_id !== filterEpic.value) {
        return false;
      }
      return true;
    });
  });

  // Issues grouped by status column for Kanban
  const issuesByStatus = computed(() => {
    const map = {};
    statuses.value.forEach((s) => {
      map[s.id] = [];
    });

    filteredIssues.value.forEach((issue) => {
      // Don't show standalone subtasks in Kanban top-level columns if they have a parent, or show them grouped
      if (map[issue.status_id]) {
        map[issue.status_id].push(issue);
      }
    });

    // Sort by position inside each column
    Object.keys(map).forEach((sid) => {
      map[sid].sort((a, b) => (a.position || 0) - (b.position || 0));
    });

    return map;
  });

  // Epics list for filtering / assignment
  const epics = computed(() => {
    return issues.value.filter((i) => i.issue_type === 'EPIC');
  });

  async function fetchProjects(workspaceId) {
    if (!workspaceId) return;
    loading.value = true;
    try {
      const data = await api.get(`/workspaces/${workspaceId}/projects`);
      projects.value = data || [];
      if (projects.value.length > 0) {
        // Select saved project or first
        const savedPid = localStorage.getItem('epictask_current_project');
        const found = projects.value.find((p) => p.id === Number(savedPid));
        await selectProject(found || projects.value[0]);
      } else {
        currentProject.value = null;
        statuses.value = [];
        issues.value = [];
      }
    } catch (err) {
      error.value = err.message;
    } finally {
      loading.value = false;
    }
  }

  async function selectProject(proj) {
    currentProject.value = proj;
    localStorage.setItem('epictask_current_project', proj.id);
    await Promise.all([
      fetchProjectStatuses(proj.id),
      fetchIssues(proj.id),
    ]);
  }

  async function createProject(workspaceId, payload) {
    const proj = await api.post(`/workspaces/${workspaceId}/projects`, payload);
    projects.value.push(proj);
    await selectProject(proj);
    return proj;
  }

  async function fetchProjectStatuses(projectId) {
    try {
      const data = await api.get(`/projects/${projectId}`);
      statuses.value = data.statuses || [];
      transitions.value = data.transitions || [];
    } catch (e) {
      console.warn('Failed to load statuses', e);
    }
  }

  async function createStatus(name, category, color) {
    if (!currentProject.value) return;
    const s = await api.post(`/projects/${currentProject.value.id}/statuses`, {
      name,
      category,
      color,
    });
    statuses.value.push(s);
    return s;
  }

  async function createTransition(fromStatusId, toStatusId) {
    if (!currentProject.value) return;
    await api.post(`/projects/${currentProject.value.id}/transitions`, {
      from_status_id: fromStatusId,
      to_status_id: toStatusId,
    });
    await fetchProjectStatuses(currentProject.value.id);
  }

  async function deleteTransition(transitionId) {
    if (!currentProject.value) return;
    await api.delete(`/projects/${currentProject.value.id}/transitions/${transitionId}`);
    await fetchProjectStatuses(currentProject.value.id);
  }

  async function fetchIssues(projectId) {
    if (!projectId) return;
    loading.value = true;
    try {
      const data = await api.get(`/projects/${projectId}/issues`);
      issues.value = data || [];
    } catch (err) {
      error.value = err.message;
    } finally {
      loading.value = false;
    }
  }

  async function createIssue(payload) {
    if (!currentProject.value) return;
    const issue = await api.post(`/projects/${currentProject.value.id}/issues`, payload);
    // Optimistic / Real-time insert
    const idx = issues.value.findIndex((i) => i.id === issue.id);
    if (idx === -1) {
      issues.value.push(issue);
    }
    return issue;
  }

  async function updateIssue(id, payload) {
    const updated = await api.put(`/issues/${id}`, payload);
    const idx = issues.value.findIndex((i) => i.id === id);
    if (idx !== -1) {
      issues.value[idx] = updated;
    }
    if (activeIssue.value && activeIssue.value.id === id) {
      activeIssue.value = updated;
    }
    return updated;
  }

  async function moveIssue(issueId, targetStatusId, targetPosition = 0) {
    const issue = issues.value.find((i) => i.id === issueId);
    if (!issue) return;

    const oldStatusId = issue.status_id;
    const oldPosition = issue.position;

    // Optimistic local update
    issue.status_id = targetStatusId;
    issue.position = targetPosition;

    try {
      const updated = await api.post(`/issues/${issueId}/move`, {
        target_status_id: targetStatusId,
        target_position: targetPosition,
      });
      const idx = issues.value.findIndex((i) => i.id === issueId);
      if (idx !== -1) {
        issues.value[idx] = updated;
      }
      return updated;
    } catch (err) {
      // Rollback on rejection (e.g. workflow transition rule violation)
      issue.status_id = oldStatusId;
      issue.position = oldPosition;
      throw err;
    }
  }

  async function deleteIssue(issueId) {
    await api.delete(`/issues/${issueId}`);
    issues.value = issues.value.filter((i) => i.id !== issueId);
    if (activeIssue.value && activeIssue.value.id === issueId) {
      closeDetailModal();
    }
  }

  async function openIssueDetail(issueId) {
    isDetailModalOpen.value = true;
    try {
      const data = await api.get(`/issues/${issueId}`);
      activeIssue.value = data.issue;
      activeIssueSubtasks.value = data.subtasks || [];
      activeIssueLinks.value = data.links || [];
      await Promise.all([
        fetchComments(issueId),
        fetchTimeLogs(issueId),
      ]);
    } catch (e) {
      console.error('Failed to load issue detail', e);
    }
  }

  function closeDetailModal() {
    isDetailModalOpen.value = false;
    activeIssue.value = null;
    activeIssueSubtasks.value = [];
    activeIssueComments.value = [];
    activeIssueTimeLogs.value = [];
  }

  async function fetchComments(issueId) {
    const data = await api.get(`/issues/${issueId}/comments`);
    activeIssueComments.value = data || [];
  }

  async function addComment(issueId, body) {
    const comment = await api.post(`/issues/${issueId}/comments`, { body });
    activeIssueComments.value.push(comment);
    return comment;
  }

  async function fetchTimeLogs(issueId) {
    const data = await api.get(`/issues/${issueId}/timelogs`);
    activeIssueTimeLogs.value = data.time_logs || [];
    totalTimeSpentSeconds.value = data.total_seconds || 0;
  }

  async function logTime(issueId, seconds, description) {
    await api.post(`/issues/${issueId}/timelogs`, {
      time_spent_seconds: seconds,
      description,
    });
    await fetchTimeLogs(issueId);
  }

  async function createIssueLink(sourceIssueId, targetIssueId, linkType) {
    await api.post(`/issues/${sourceIssueId}/links`, {
      target_issue_id: targetIssueId,
      link_type: linkType,
    });
    if (activeIssue.value && activeIssue.value.id === sourceIssueId) {
      await openIssueDetail(sourceIssueId);
    }
  }

  // Handle Real-time WebSocket events (FR-6.1)
  function handleWsMessage(msg) {
    if (!msg || !msg.event) return;

    if (msg.event === 'ISSUE_CREATED' && msg.data) {
      if (currentProject.value && msg.project_id === currentProject.value.id) {
        const exists = issues.value.some((i) => i.id === msg.data.id);
        if (!exists) {
          issues.value.push(msg.data);
        }
      }
    } else if ((msg.event === 'ISSUE_UPDATED' || msg.event === 'ISSUE_MOVED') && msg.data) {
      const idx = issues.value.findIndex((i) => i.id === msg.data.id);
      if (idx !== -1) {
        issues.value[idx] = msg.data;
      }
      if (activeIssue.value && activeIssue.value.id === msg.data.id) {
        activeIssue.value = msg.data;
      }
    } else if (msg.event === 'ISSUE_DELETED' && msg.data) {
      issues.value = issues.value.filter((i) => i.id !== msg.data.issue_id);
      if (activeIssue.value && activeIssue.value.id === msg.data.issue_id) {
        closeDetailModal();
      }
    } else if (msg.event === 'AUTOMATION_TRIGGERED') {
      if (currentProject.value && msg.project_id === currentProject.value.id) {
        // Refresh issues after automation
        fetchIssues(currentProject.value.id);
      }
    } else if (msg.event === 'COMMENT_ADDED' && msg.data) {
      if (activeIssue.value && activeIssue.value.id === msg.data.issue_id) {
        const exists = activeIssueComments.value.some((c) => c.id === msg.data.id);
        if (!exists) {
          activeIssueComments.value.push(msg.data);
        }
      }
    }
  }

  // Set up WebSocket listener
  wsClient.subscribe(handleWsMessage);

  return {
    projects,
    currentProject,
    statuses,
    transitions,
    issues,
    filteredIssues,
    issuesByStatus,
    epics,
    activeView,
    loading,
    error,
    searchQuery,
    filterAssignee,
    filterPriority,
    filterEpic,
    filterType,
    activeIssue,
    activeIssueSubtasks,
    activeIssueLinks,
    activeIssueComments,
    activeIssueTimeLogs,
    totalTimeSpentSeconds,
    isDetailModalOpen,
    isCreateModalOpen,
    defaultCreateStatusId,
    defaultCreateParentId,
    fetchProjects,
    selectProject,
    createProject,
    fetchProjectStatuses,
    createStatus,
    createTransition,
    deleteTransition,
    fetchIssues,
    createIssue,
    updateIssue,
    moveIssue,
    deleteIssue,
    openIssueDetail,
    closeDetailModal,
    addComment,
    logTime,
    createIssueLink,
  };
});
