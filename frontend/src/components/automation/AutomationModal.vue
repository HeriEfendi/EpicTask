<script setup>
import { ref, onMounted } from 'vue';
import { useAutomationStore } from '@/stores/automation';
import { useProjectStore } from '@/stores/project';
import {
  Bot,
  Zap,
  ArrowRight,
  Plus,
  Trash2,
  CheckCircle2,
  Clock,
  Sparkles,
  ToggleLeft,
  ToggleRight,
  X
} from 'lucide-vue-next';

const autoStore = useAutomationStore();
const projectStore = useProjectStore();

const isCreating = ref(false);
const ruleName = ref('');
const selectedTrigger = ref('STATUS_CHANGED');
const selectedAction = ref('CASCADE_SUBTASKS_DONE');

onMounted(() => {
  if (projectStore.currentProject) {
    autoStore.fetchRules(projectStore.currentProject.id);
  }
});

async function handleToggle(rule) {
  try {
    await autoStore.toggleRule(
      projectStore.currentProject.id,
      rule.id,
      !rule.is_active
    );
  } catch (e) {
    alert(e.message || 'Failed to update rule');
  }
}

async function handleDelete(ruleId) {
  if (confirm('Delete this automation rule?')) {
    await autoStore.deleteRule(projectStore.currentProject.id, ruleId);
  }
}

async function handleCreateRule() {
  if (!ruleName.value.trim()) return;

  const triggerConfig = selectedTrigger.value === 'STATUS_CHANGED'
    ? { category: 'DONE' }
    : { hours: 24 };

  try {
    await autoStore.createRule(projectStore.currentProject.id, {
      name: ruleName.value.trim(),
      trigger_type: selectedTrigger.value,
      trigger_config: triggerConfig,
      action_type: selectedAction.value,
      action_config: {},
      is_active: true,
    });
    ruleName.value = '';
    isCreating.value = false;
  } catch (e) {
    alert(e.message || 'Failed to create automation rule');
  }
}

function applyTemplate(name, trigger, action) {
  ruleName.value = name;
  selectedTrigger.value = trigger;
  selectedAction.value = action;
  isCreating.value = true;
}
</script>

<template>
  <div
    v-if="autoStore.isRuleModalOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/40 backdrop-blur-sm"
  >
    <div class="glass-modal w-full max-w-2xl rounded-2xl p-6 border border-slate-200 bg-white max-h-[90vh] flex flex-col shadow-2xl animate-slide-up">
      <!-- Header -->
      <div class="flex items-center justify-between pb-4 border-b border-slate-200">
        <div class="flex items-center space-x-2.5">
          <div class="w-8 h-8 rounded-lg bg-purple-50 border border-purple-200 flex items-center justify-center text-purple-600">
            <Bot class="w-4 h-4" />
          </div>
          <div>
            <h2 class="text-base font-bold text-slate-800">No-Code Automation Engine</h2>
            <p class="text-xs text-slate-500">Build IF-THEN rules to eliminate manual repetitive work</p>
          </div>
        </div>

        <button
          @click="autoStore.isRuleModalOpen = false"
          class="p-1 rounded-lg text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Main Body -->
      <div class="flex-1 overflow-y-auto py-4 space-y-4">
        <!-- Pre-made Templates -->
        <div class="bg-gradient-to-r from-purple-50 to-indigo-50 border border-purple-200 rounded-xl p-3.5">
          <div class="flex items-center space-x-2 text-xs font-semibold text-purple-800 mb-2.5">
            <Sparkles class="w-3.5 h-3.5 text-purple-600" />
            <span>Recommended Automation Templates</span>
          </div>
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
            <button
              @click="applyTemplate('Auto-Complete Subtasks', 'STATUS_CHANGED', 'CASCADE_SUBTASKS_DONE')"
              class="text-left p-2.5 bg-white hover:bg-purple-50/80 border border-purple-100 rounded-lg text-xs transition shadow-2xs"
            >
              <p class="font-bold text-slate-800">Cascade Subtasks to Done</p>
              <p class="text-[11px] text-slate-500">When parent is marked Done, all subtasks close automatically</p>
            </button>

            <button
              @click="applyTemplate('Reassign to Reporter when Done', 'STATUS_CHANGED', 'ASSIGN_TO_REPORTER')"
              class="text-left p-2.5 bg-white hover:bg-purple-50/80 border border-purple-100 rounded-lg text-xs transition shadow-2xs"
            >
              <p class="font-bold text-slate-800">Hand-off Back to Reporter</p>
              <p class="text-[11px] text-slate-500">When ticket is completed, reassign back to original creator</p>
            </button>
          </div>
        </div>

        <!-- Rule Builder Form (IF-THEN) -->
        <div v-if="isCreating" class="bg-slate-50 border border-blue-200 rounded-xl p-4 space-y-3.5 animate-slide-up">
          <div class="flex items-center justify-between">
            <h3 class="text-xs font-bold text-blue-600 uppercase tracking-wider flex items-center gap-1.5">
              <Zap class="w-3.5 h-3.5" /> Rule Builder (IF - THEN - THAT)
            </h3>
            <button @click="isCreating = false" class="text-xs font-medium text-slate-500 hover:text-slate-800">Cancel</button>
          </div>

          <div>
            <label class="block text-[11px] text-slate-600 font-medium mb-1">Rule Name</label>
            <input
              v-model="ruleName"
              placeholder="e.g. Close Subtasks When Epic Finishes"
              class="w-full bg-white border border-slate-300 rounded-lg px-3 py-1.5 text-xs text-slate-900 focus:outline-none focus:border-blue-600 shadow-2xs"
            />
          </div>

          <!-- Visual IF block -->
          <div class="p-3 bg-white rounded-lg border border-slate-200 flex items-center space-x-3 shadow-2xs">
            <span class="px-2 py-0.5 rounded font-bold text-xs bg-amber-50 text-amber-700 border border-amber-200">
              IF TRIGGER
            </span>
            <div class="flex-1">
              <select
                v-model="selectedTrigger"
                class="w-full bg-slate-50 border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-800 focus:outline-none focus:border-blue-600"
              >
                <option value="STATUS_CHANGED">When issue status changes to "Done"</option>
                <option value="DUE_DATE_NEAR">When Due Date is within 24 hours</option>
              </select>
            </div>
          </div>

          <!-- Arrow connector -->
          <div class="flex justify-center text-slate-400">
            <ArrowRight class="w-4 h-4 rotate-90" />
          </div>

          <!-- Visual THEN block -->
          <div class="p-3 bg-white rounded-lg border border-slate-200 flex items-center space-x-3 shadow-2xs">
            <span class="px-2 py-0.5 rounded font-bold text-xs bg-emerald-50 text-emerald-700 border border-emerald-200">
              THEN ACTION
            </span>
            <div class="flex-1">
              <select
                v-model="selectedAction"
                class="w-full bg-slate-50 border border-slate-300 rounded-lg px-2.5 py-1.5 text-xs text-slate-800 focus:outline-none focus:border-blue-600"
              >
                <option value="CASCADE_SUBTASKS_DONE">Automatically set all child subtasks to Done</option>
                <option value="ASSIGN_TO_REPORTER">Reassign ticket back to the original Reporter</option>
              </select>
            </div>
          </div>

          <div class="flex justify-end space-x-2 pt-2">
            <button
              @click="isCreating = false"
              class="px-3 py-1.5 rounded-lg text-xs font-medium text-slate-600 hover:text-slate-800 hover:bg-slate-200/50"
            >
              Cancel
            </button>
            <button
              @click="handleCreateRule"
              class="px-4 py-1.5 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-lg text-xs font-semibold shadow-xs"
            >
              Save Rule
            </button>
          </div>
        </div>

        <!-- Create Button -->
        <div v-else class="flex justify-end">
          <button
            @click="isCreating = true"
            class="flex items-center space-x-1.5 px-3 py-1.5 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-lg text-xs font-semibold shadow-xs transition"
          >
            <Plus class="w-3.5 h-3.5" />
            <span>Create Custom Rule</span>
          </button>
        </div>

        <!-- Existing Active Rules List -->
        <div class="space-y-2">
          <h3 class="text-xs font-bold text-slate-500 uppercase tracking-wider">Active Rules in this Project</h3>
          <div
            v-for="rule in autoStore.rules"
            :key="rule.id"
            class="bg-white border border-slate-200 rounded-xl p-3.5 flex items-center justify-between transition hover:border-slate-300 shadow-2xs"
          >
            <div class="flex items-start space-x-3">
              <button
                @click="handleToggle(rule)"
                class="mt-0.5 text-slate-400 hover:text-slate-700 transition"
                :title="rule.is_active ? 'Click to disable' : 'Click to enable'"
              >
                <ToggleRight v-if="rule.is_active" class="w-6 h-6 text-emerald-600" />
                <ToggleLeft v-else class="w-6 h-6 text-slate-400" />
              </button>

              <div>
                <p class="text-xs font-bold text-slate-800 flex items-center gap-2">
                  <span>{{ rule.name }}</span>
                  <span
                    class="text-[10px] px-1.5 py-0.2 rounded font-medium"
                    :class="rule.is_active ? 'bg-emerald-50 text-emerald-700 border border-emerald-200' : 'bg-slate-100 text-slate-500'"
                  >
                    {{ rule.is_active ? 'ACTIVE' : 'PAUSED' }}
                  </span>
                </p>
                <div class="flex items-center space-x-2 text-[11px] text-slate-500 mt-1">
                  <span class="text-amber-700 font-medium">IF: {{ rule.trigger_type }}</span>
                  <span>→</span>
                  <span class="text-emerald-700 font-medium">THEN: {{ rule.action_type }}</span>
                </div>
              </div>
            </div>

            <button
              @click="handleDelete(rule.id)"
              class="p-1.5 rounded-lg text-slate-400 hover:text-red-600 hover:bg-red-50 transition"
              title="Delete Rule"
            >
              <Trash2 class="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
