import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { api } from '@/services/api';
import { wsClient } from '@/services/websocket';
import { sendDesktopNotification } from '@/services/tauri';
import { useAuthStore } from '@/stores/auth';
import { useSprintStore } from '@/stores/sprint';
import { useComponentStore } from '@/stores/component';
import { useVersionStore } from '@/stores/version';
import { useLabelStore } from '@/stores/label';

export const useProjectStore = defineStore('project', () => {
  const authStore = useAuthStore();
  const projects = ref([]);
  const currentProject = ref(null);
  const statuses = ref([]);
  const transitions = ref([]);
  const issues = ref([]);
  const activeView = ref('kanban'); // 'kanban' | 'timeline' | 'list' | 'reports' | 'backlog'
  const swimlaneMode = ref('none'); // 'none' | 'assignee' | 'epic' | 'priority'
  const loading = ref(false);
  const error = ref(null);

  // Filters
  const searchQuery = ref('');
  const filterAssignees = ref([]); // multi-user array: [userId, ...]
  const filterPriority = ref(null);
  const filterEpic = ref(null);
  const filterType = ref(null);
  const filterSprint = ref(null); // null = all, number = specific sprint
  const filterComponent = ref(null); // null = all, number = specific component
  const filterVersion = ref(null); // null = all, number = specific fix_version_id
  const filterLabel = ref(null); // null = all, string = specific label name
  const quickFilters = ref([]); // array of active keys: ['my_issues', 'only_bugs', 'highest_priority', 'unassigned', 'due_this_week', 'unresolved']

  // Jira Enterprise Advanced Multi-Dimension Filters
  const filterStatuses = ref([]);       // status IDs e.g. [1, 2]
  const filterCategories = ref([]);     // category strings e.g. ['TODO', 'IN_PROGRESS']
  const filterPriorities = ref([]);     // priority strings e.g. ['HIGH', 'HIGHEST']
  const filterTypes = ref([]);          // issue_type strings e.g. ['BUG', 'TASK']
  const filterEpics = ref([]);          // epic IDs e.g. [18]
  const filterSprints = ref([]);        // sprint IDs e.g. [1]
  const filterComponents = ref([]);     // component IDs e.g. [2]
  const filterVersions = ref([]);       // version IDs e.g. [1]
  const filterLabels = ref([]);         // label strings e.g. ['frontend']
  const filterResolutions = ref([]);    // resolution strings e.g. ['Fixed', 'UNRESOLVED']
  const isFilterModalOpen = ref(false);

  // Legacy single-assignee alias (backward compat for old code)
  const filterAssignee = ref(null);

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
      // Status filter
      if (filterStatuses.value.length > 0) {
        if (!filterStatuses.value.includes(issue.status_id)) return false;
      }
      // Status category filter
      if (filterCategories.value.length > 0) {
        if (!filterCategories.value.includes(issue.status_category)) return false;
      }
      // Multi-assignee filter
      if (filterAssignees.value.length > 0) {
        if (!filterAssignees.value.includes(issue.assignee_id)) return false;
      } else if (filterAssignee.value && issue.assignee_id !== filterAssignee.value) {
        return false;
      }
      // Priority filter
      if (filterPriorities.value.length > 0) {
        if (!filterPriorities.value.includes(issue.priority)) return false;
      } else if (filterPriority.value && issue.priority !== filterPriority.value) {
        return false;
      }
      // Work type / Issue type filter
      if (filterTypes.value.length > 0) {
        if (!filterTypes.value.includes(issue.issue_type)) return false;
      } else if (filterType.value && issue.issue_type !== filterType.value) {
        return false;
      }
      // Parent / Epic filter
      if (filterEpics.value.length > 0) {
        if (!filterEpics.value.includes(issue.epic_id)) return false;
      } else if (filterEpic.value && issue.epic_id !== filterEpic.value) {
        return false;
      }
      // Sprint filter
      if (filterSprints.value.length > 0) {
        if (!filterSprints.value.includes(issue.sprint_id)) return false;
      } else if (filterSprint.value !== null && filterSprint.value !== undefined) {
        if (issue.sprint_id !== filterSprint.value) return false;
      }
      // Component filter
      if (filterComponents.value.length > 0) {
        if (!filterComponents.value.includes(issue.component_id)) return false;
      } else if (filterComponent.value && issue.component_id !== filterComponent.value) {
        return false;
      }
      // Fix version / release filter
      if (filterVersions.value.length > 0) {
        if (!filterVersions.value.includes(issue.fix_version_id)) return false;
      } else if (filterVersion.value && issue.fix_version_id !== filterVersion.value) {
        return false;
      }
      // Label filter
      if (filterLabels.value.length > 0) {
        const issueLabels = issue.labels
          ? issue.labels.split(',').map((l) => l.trim().toLowerCase())
          : [];
        const hasMatch = filterLabels.value.some((lbl) =>
          issueLabels.includes(lbl.trim().toLowerCase())
        );
        if (!hasMatch) return false;
      } else if (filterLabel.value) {
        const issueLabels = issue.labels
          ? issue.labels.split(',').map((l) => l.trim().toLowerCase())
          : [];
        if (!issueLabels.includes(filterLabel.value.toLowerCase())) {
          return false;
        }
      }
      // Resolution filter
      if (filterResolutions.value.length > 0) {
        const isUnresolved = issue.resolution === null || issue.resolution === undefined;
        const matches = filterResolutions.value.some((res) => {
          if (res === 'UNRESOLVED') return isUnresolved;
          return issue.resolution === res;
        });
        if (!matches) return false;
      }

      // Quick Filters (Jira Standard)
      if (quickFilters.value.length > 0) {
        for (const qf of quickFilters.value) {
          if (qf === 'my_issues') {
            const myId = authStore?.user?.id;
            if (!myId || issue.assignee_id !== myId) return false;
          } else if (qf === 'only_bugs') {
            if (issue.issue_type !== 'BUG') return false;
          } else if (qf === 'highest_priority') {
            if (issue.priority !== 'HIGHEST' && issue.priority !== 'HIGH') return false;
          } else if (qf === 'unassigned') {
            if (issue.assignee_id !== null && issue.assignee_id !== undefined) return false;
          } else if (qf === 'due_this_week') {
            if (!issue.due_date) return false;
            const due = new Date(issue.due_date);
            const now = new Date();
            const dayOfWeek = now.getDay() === 0 ? 7 : now.getDay();
            const startOfWeek = new Date(now);
            startOfWeek.setDate(now.getDate() - dayOfWeek + 1);
            startOfWeek.setHours(0, 0, 0, 0);
            const endOfWeek = new Date(startOfWeek);
            endOfWeek.setDate(startOfWeek.getDate() + 6);
            endOfWeek.setHours(23, 59, 59, 999);
            if (due < startOfWeek || due > endOfWeek) return false;
          } else if (qf === 'unresolved') {
            if (issue.resolution !== null && issue.resolution !== undefined) return false;
            if (issue.status_category === 'DONE') return false;
          }
        }
      }

      return true;
    });
  });

  function toggleQuickFilter(key) {
    const idx = quickFilters.value.indexOf(key);
    if (idx === -1) {
      quickFilters.value.push(key);
    } else {
      quickFilters.value.splice(idx, 1);
    }
  }

  function isQuickFilterActive(key) {
    return quickFilters.value.includes(key);
  }

  function clearQuickFilters() {
    quickFilters.value = [];
  }

  function toggleFilterItem(dimensionKey, value) {
    let arr = null;
    switch (dimensionKey) {
      case 'status': arr = filterStatuses.value; break;
      case 'category': arr = filterCategories.value; break;
      case 'assignee': arr = filterAssignees.value; break;
      case 'priority': arr = filterPriorities.value; break;
      case 'type': arr = filterTypes.value; break;
      case 'epic': arr = filterEpics.value; break;
      case 'sprint': arr = filterSprints.value; break;
      case 'component': arr = filterComponents.value; break;
      case 'version': arr = filterVersions.value; break;
      case 'label': arr = filterLabels.value; break;
      case 'resolution': arr = filterResolutions.value; break;
    }
    if (!arr) return;
    const idx = arr.indexOf(value);
    if (idx === -1) {
      arr.push(value);
    } else {
      arr.splice(idx, 1);
    }
  }

  function isFilterItemSelected(dimensionKey, value) {
    switch (dimensionKey) {
      case 'status': return filterStatuses.value.includes(value);
      case 'category': return filterCategories.value.includes(value);
      case 'assignee': return filterAssignees.value.includes(value);
      case 'priority': return filterPriorities.value.includes(value);
      case 'type': return filterTypes.value.includes(value);
      case 'epic': return filterEpics.value.includes(value);
      case 'sprint': return filterSprints.value.includes(value);
      case 'component': return filterComponents.value.includes(value);
      case 'version': return filterVersions.value.includes(value);
      case 'label': return filterLabels.value.includes(value);
      case 'resolution': return filterResolutions.value.includes(value);
      default: return false;
    }
  }

  function clearDimensionFilter(dimensionKey) {
    switch (dimensionKey) {
      case 'status': filterStatuses.value = []; break;
      case 'category': filterCategories.value = []; break;
      case 'assignee': filterAssignees.value = []; filterAssignee.value = null; break;
      case 'priority': filterPriorities.value = []; filterPriority.value = null; break;
      case 'type': filterTypes.value = []; filterType.value = null; break;
      case 'epic': filterEpics.value = []; filterEpic.value = null; break;
      case 'sprint': filterSprints.value = []; filterSprint.value = null; break;
      case 'component': filterComponents.value = []; filterComponent.value = null; break;
      case 'version': filterVersions.value = []; filterVersion.value = null; break;
      case 'label': filterLabels.value = []; filterLabel.value = null; break;
      case 'resolution': filterResolutions.value = []; break;
    }
  }

  function clearAllAdvancedFilters() {
    searchQuery.value = '';
    filterStatuses.value = [];
    filterCategories.value = [];
    filterAssignees.value = [];
    filterAssignee.value = null;
    filterPriorities.value = [];
    filterPriority.value = null;
    filterTypes.value = [];
    filterType.value = null;
    filterEpics.value = [];
    filterEpic.value = null;
    filterSprints.value = [];
    filterSprint.value = null;
    filterComponents.value = [];
    filterComponent.value = null;
    filterVersions.value = [];
    filterVersion.value = null;
    filterLabels.value = [];
    filterLabel.value = null;
    filterResolutions.value = [];
    quickFilters.value = [];
  }

  const activeFilterDimensionsCount = computed(() => {
    let count = 0;
    if (filterStatuses.value.length > 0) count++;
    if (filterCategories.value.length > 0) count++;
    if (filterAssignees.value.length > 0 || filterAssignee.value) count++;
    if (filterPriorities.value.length > 0 || filterPriority.value) count++;
    if (filterTypes.value.length > 0 || filterType.value) count++;
    if (filterEpics.value.length > 0 || filterEpic.value) count++;
    if (filterSprints.value.length > 0 || (filterSprint.value !== null && filterSprint.value !== undefined)) count++;
    if (filterComponents.value.length > 0 || filterComponent.value) count++;
    if (filterVersions.value.length > 0 || filterVersion.value) count++;
    if (filterLabels.value.length > 0 || filterLabel.value) count++;
    if (filterResolutions.value.length > 0) count++;
    return count;
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
    const sprintStore = useSprintStore();
    const compStore = useComponentStore();
    const versionStore = useVersionStore();
    const labelStore = useLabelStore();
    await Promise.all([
      fetchProjectStatuses(proj.id),
      fetchIssues(proj.id),
      sprintStore.fetchSprints(proj.id),
      compStore.fetchComponents(proj.id),
      versionStore.fetchVersions(proj.id),
      labelStore.fetchLabels(proj.id),
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

  async function moveIssue(issueId, targetStatusId, targetPosition = 0, resolution = null) {
    const issue = issues.value.find((i) => i.id === issueId);
    if (!issue) return;

    const oldStatusId = issue.status_id;
    const oldPosition = issue.position;
    const oldResolution = issue.resolution;

    // Optimistic local update
    issue.status_id = targetStatusId;
    issue.position = targetPosition;
    if (resolution) issue.resolution = resolution;

    try {
      const payload = {
        target_status_id: targetStatusId,
        target_position: targetPosition,
      };
      if (resolution) {
        payload.resolution = resolution;
      }
      const updated = await api.post(`/issues/${issueId}/move`, payload);
      const idx = issues.value.findIndex((i) => i.id === issueId);
      if (idx !== -1) {
        issues.value[idx] = updated;
      }
      return updated;
    } catch (err) {
      // Rollback on rejection (e.g. workflow transition rule violation)
      issue.status_id = oldStatusId;
      issue.position = oldPosition;
      issue.resolution = oldResolution;
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
    swimlaneMode,
    loading,
    error,
    searchQuery,
    filterAssignee,
    filterAssignees,
    filterPriority,
    filterEpic,
    filterType,
    filterSprint,
    filterComponent,
    filterVersion,
    filterLabel,
    filterStatuses,
    filterCategories,
    filterPriorities,
    filterTypes,
    filterEpics,
    filterSprints,
    filterComponents,
    filterVersions,
    filterLabels,
    filterResolutions,
    isFilterModalOpen,
    toggleFilterItem,
    isFilterItemSelected,
    clearDimensionFilter,
    clearAllAdvancedFilters,
    activeFilterDimensionsCount,
    quickFilters,
    toggleQuickFilter,
    isQuickFilterActive,
    clearQuickFilters,
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
