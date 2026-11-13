<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
import { useLabelStore } from '@/stores/label';
import { Tag, X, Plus } from 'lucide-vue-next';

const props = defineProps({
  modelValue: {
    type: Array,
    default: () => [],
  },
  disabled: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['update:modelValue', 'change']);

const labelStore = useLabelStore();
const inputVal = ref('');
const isDropdownOpen = ref(false);
const containerRef = ref(null);

const tagColors = [
  'bg-indigo-50 text-indigo-700 border-indigo-200',
  'bg-emerald-50 text-emerald-700 border-emerald-200',
  'bg-amber-50 text-amber-800 border-amber-200',
  'bg-cyan-50 text-cyan-800 border-cyan-200',
  'bg-rose-50 text-rose-700 border-rose-200',
  'bg-purple-50 text-purple-700 border-purple-200',
  'bg-blue-50 text-blue-700 border-blue-200',
  'bg-teal-50 text-teal-800 border-teal-200',
];

function getTagColorClass(name) {
  let hash = 0;
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash);
  }
  const index = Math.abs(hash) % tagColors.length;
  return tagColors[index];
}

// Suggestions matching input that are not yet added
const filteredSuggestions = computed(() => {
  const query = inputVal.value.trim().toLowerCase();
  const current = (props.modelValue || []).map((l) => l.toLowerCase());
  return labelStore.labels.filter(
    (l) =>
      !current.includes(l.name.toLowerCase()) &&
      (!query || l.name.toLowerCase().includes(query))
  );
});

// Check if current input matches an existing label exactly
const hasExactMatch = computed(() => {
  const query = inputVal.value.trim().toLowerCase();
  if (!query) return true;
  return (
    (props.modelValue || []).some((l) => l.toLowerCase() === query) ||
    labelStore.labels.some((l) => l.name.toLowerCase() === query)
  );
});

function addLabel(name) {
  const clean = name.trim().toLowerCase();
  if (!clean) return;

  const current = [...(props.modelValue || [])];
  if (!current.some((l) => l.toLowerCase() === clean)) {
    current.push(clean);
    emit('update:modelValue', current);
    emit('change', current);
  }
  inputVal.value = '';
  isDropdownOpen.value = false;
}

function removeLabel(index) {
  if (props.disabled) return;
  const current = [...(props.modelValue || [])];
  current.splice(index, 1);
  emit('update:modelValue', current);
  emit('change', current);
}

function handleKeyDown(e) {
  if (e.key === 'Enter' || e.key === ',') {
    e.preventDefault();
    if (inputVal.value.trim()) {
      addLabel(inputVal.value);
    }
  } else if (e.key === 'Backspace' && !inputVal.value && (props.modelValue || []).length > 0) {
    removeLabel(props.modelValue.length - 1);
  }
}

function handleClickOutside(e) {
  if (containerRef.value && !containerRef.value.contains(e.target)) {
    isDropdownOpen.value = false;
  }
}

onMounted(() => document.addEventListener('mousedown', handleClickOutside));
onBeforeUnmount(() => document.removeEventListener('mousedown', handleClickOutside));
</script>

<template>
  <div class="relative space-y-1.5" ref="containerRef">
    <div
      class="w-full bg-white border border-slate-300 rounded-lg p-2 text-sm text-slate-800 shadow-2xs focus-within:border-blue-600 focus-within:ring-1 focus-within:ring-blue-600 flex flex-wrap items-center gap-1.5 transition min-h-[38px]"
      :class="{ 'opacity-60 pointer-events-none': disabled }"
      @click="isDropdownOpen = true"
    >
      <!-- Active Label Chips -->
      <span
        v-for="(label, idx) in modelValue"
        :key="idx"
        class="inline-flex items-center gap-1 text-xs font-semibold px-2.5 py-0.5 rounded-md border select-none transition-all shadow-2xs"
        :class="getTagColorClass(label)"
      >
        <span>🏷️ {{ label }}</span>
        <button
          v-if="!disabled"
          type="button"
          @click.stop="removeLabel(idx)"
          class="hover:opacity-75 rounded-xs p-0.5 ml-0.5"
          title="Hapus label"
        >
          <X class="w-3 h-3" />
        </button>
      </span>

      <!-- Inline Input -->
      <input
        v-if="!disabled"
        v-model="inputVal"
        type="text"
        placeholder="+ Tambah label..."
        @focus="isDropdownOpen = true"
        @keydown="handleKeyDown"
        class="flex-1 min-w-[100px] border-none outline-none text-xs text-slate-800 placeholder-slate-400 bg-transparent py-0.5"
      />
    </div>

    <!-- Autocomplete & Suggestions Dropdown -->
    <div
      v-if="isDropdownOpen && !disabled && (filteredSuggestions.length > 0 || (!hasExactMatch && inputVal.trim()))"
      class="absolute left-0 right-0 top-full mt-1 z-30 bg-white border border-slate-200 rounded-xl shadow-xl max-h-48 overflow-y-auto p-1.5 space-y-0.5 select-none"
    >
      <!-- Option to create new label -->
      <button
        v-if="!hasExactMatch && inputVal.trim()"
        type="button"
        @click="addLabel(inputVal)"
        class="w-full text-left px-3 py-1.5 rounded-lg text-xs font-semibold text-blue-700 bg-blue-50/70 hover:bg-blue-100 flex items-center gap-1.5 transition"
      >
        <Plus class="w-3.5 h-3.5 text-blue-600" />
        <span>Buat label baru: "<strong>{{ inputVal.trim().toLowerCase() }}</strong>"</span>
      </button>

      <!-- Existing matching labels -->
      <button
        v-for="sug in filteredSuggestions"
        :key="sug.id"
        type="button"
        @click="addLabel(sug.name)"
        class="w-full text-left px-3 py-1.5 rounded-lg text-xs text-slate-700 hover:bg-slate-100 flex items-center justify-between font-medium transition"
      >
        <div class="flex items-center gap-2">
          <span class="w-2 h-2 rounded-full" :style="{ backgroundColor: sug.color || '#6366f1' }"></span>
          <span>{{ sug.name }}</span>
        </div>
        <span class="text-[10px] text-slate-400 font-mono">{{ sug.issue_count || 0 }} tiket</span>
      </button>
    </div>
  </div>
</template>
