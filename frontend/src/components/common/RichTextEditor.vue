<script setup>
import { ref, computed, onMounted, onBeforeUnmount, watch, nextTick } from 'vue';
import {
  Bold,
  Italic,
  Underline,
  Strikethrough,
  Heading1,
  Heading2,
  Heading3,
  List,
  ListOrdered,
  Image as ImageIcon,
  Code,
  Table as TableIcon,
  Link as LinkIcon,
  Undo,
  Redo,
  Palette,
  Maximize2,
  Minimize2,
  Quote,
  AlignLeft,
  AlignCenter,
  AlignRight,
  Plus,
  Trash2,
  Check,
  Upload,
  AlertCircle,
  HelpCircle,
  FileText,
  Rows,
  Columns,
  Sparkles,
  ChevronDown
} from 'lucide-vue-next';
import { toast } from '@/utils/toast';

const props = defineProps({
  modelValue: {
    type: String,
    default: '',
  },
  placeholder: {
    type: String,
    default: 'Tambahkan deskripsi rinci, kriteria penerimaan, tabel spesifikasi, lampiran gambar...',
  },
});

const emit = defineEmits(['update:modelValue', 'save']);

const editorRef = ref(null);
const fileInputRef = ref(null);
const isFullscreen = ref(false);
const isInsideTable = ref(false);
const isRawMode = ref(false);
const rawHtml = ref('');

// Dropdowns & Popovers state
const isStyleMenuOpen = ref(false);
const isMoreMenuOpen = ref(false);
const isColorMenuOpen = ref(false);
const isTableMenuOpen = ref(false);
const isImageModalOpen = ref(false);
const isLinkModalOpen = ref(false);

const imageUrlInput = ref('');
const linkUrlInput = ref('');
const linkTextInput = ref('');

// Selection saving for modal dialogs
let savedSelectionRange = null;

// Word & Character count
const charCount = ref(0);
const wordCount = ref(0);
const hasUnsavedChanges = ref(false);
const isSaved = ref(true);
let autoSaveTimeout = null;

// Table size selector
const tableRows = ref(3);
const tableCols = ref(3);

// Colors for palette
const textColors = [
  { name: 'Default', value: '#0f172a' },
  { name: 'Muted', value: '#64748b' },
  { name: 'Merah', value: '#dc2626' },
  { name: 'Oranye', value: '#ea580c' },
  { name: 'Kuning', value: '#d97706' },
  { name: 'Hijau', value: '#16a34a' },
  { name: 'Biru', value: '#2563eb' },
  { name: 'Ungu', value: '#9333ea' },
];

const bgColors = [
  { name: 'Tanpa Highlight', value: 'transparent' },
  { name: 'Kuning', value: '#fef08a' },
  { name: 'Hijau', value: '#bbf7d0' },
  { name: 'Biru', value: '#bfdbfe' },
  { name: 'Merah', value: '#fecaca' },
  { name: 'Ungu', value: '#e9d5ff' },
  { name: 'Abu-abu', value: '#e2e8f0' },
];

// Initialize content
function updateTableContext() {
  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0 || !editorRef.value) {
    isInsideTable.value = false;
    return;
  }
  let node = sel.getRangeAt(0).startContainer;
  if (node.nodeType === Node.TEXT_NODE) node = node.parentElement;
  isInsideTable.value = !!node.closest?.('table') && editorRef.value.contains(node);
}

onMounted(() => {
  if (editorRef.value) {
    editorRef.value.innerHTML = props.modelValue || '';
    updateStats();
  }
  document.addEventListener('click', handleOutsideClick);
  document.addEventListener('selectionchange', updateTableContext);
});

onBeforeUnmount(() => {
  document.removeEventListener('click', handleOutsideClick);
  document.removeEventListener('selectionchange', updateTableContext);
  if (autoSaveTimeout) clearTimeout(autoSaveTimeout);
});

// Watch external value change
watch(
  () => props.modelValue,
  (newVal) => {
    if (editorRef.value && !isFocused.value && editorRef.value.innerHTML !== (newVal || '')) {
      editorRef.value.innerHTML = newVal || '';
      updateStats();
    }
  }
);

const isFocused = ref(false);

function handleFocus() {
  isFocused.value = true;
}

function handleBlur() {
  isFocused.value = false;
  saveSelection();
  emitContent(true);
}

function handleInput() {
  hasUnsavedChanges.value = true;
  isSaved.value = false;
  updateStats();

  if (autoSaveTimeout) clearTimeout(autoSaveTimeout);
  autoSaveTimeout = setTimeout(() => {
    emitContent(false);
  }, 1000);
}

function updateStats() {
  if (!editorRef.value) return;
  const text = editorRef.value.innerText || '';
  charCount.value = text.length;
  const words = text.trim().split(/\s+/).filter(Boolean);
  wordCount.value = words.length;
}

function emitContent(triggerSave = false) {
  if (!editorRef.value) return;
  const content = editorRef.value.innerHTML;
  emit('update:modelValue', content);
  hasUnsavedChanges.value = false;
  isSaved.value = true;
  if (triggerSave) {
    emit('save', content);
  }
}

function handleManualSave() {
  emitContent(true);
}

// Selection helpers
function saveSelection() {
  const sel = window.getSelection();
  if (sel && sel.rangeCount > 0) {
    savedSelectionRange = sel.getRangeAt(0).cloneRange();
  }
}

function restoreSelection() {
  if (savedSelectionRange) {
    const sel = window.getSelection();
    sel.removeAllRanges();
    sel.addRange(savedSelectionRange);
  } else if (editorRef.value) {
    editorRef.value.focus();
  }
}

// Formatting commands
function execCmd(command, value = null) {
  if (isRawMode.value) return;
  editorRef.value?.focus();
  document.execCommand(command, false, value);
  handleInput();
}

function setBlock(tag) {
  isStyleMenuOpen.value = false;
  if (tag === 'p') {
    execCmd('formatBlock', '<p>');
  } else if (tag === 'pre') {
    execCmd('formatBlock', '<pre>');
  } else if (tag === 'blockquote') {
    execCmd('formatBlock', '<blockquote>');
  } else {
    execCmd('formatBlock', `<${tag}>`);
  }
}

function applyTextColor(color) {
  isColorMenuOpen.value = false;
  restoreSelection();
  execCmd('foreColor', color);
}

function applyBgColor(color) {
  isColorMenuOpen.value = false;
  restoreSelection();
  execCmd('hiliteColor', color);
}

// Table insertion & manipulation
function insertTable() {
  isTableMenuOpen.value = false;
  isMoreMenuOpen.value = false;
  restoreSelection();

  const rows = Math.max(1, tableRows.value);
  const cols = Math.max(1, tableCols.value);

  let html = '<table class="rich-table" style="width:100%; border-collapse:collapse; margin:1rem 0;">';
  // Header
  html += '<thead><tr>';
  for (let c = 0; c < cols; c++) {
    html += `<th style="border:1px solid #cbd5e1; background:#f1f5f9; padding:8px 12px; font-weight:600; text-align:left;">Kolom ${c + 1}</th>`;
  }
  html += '</tr></thead>';

  // Body
  html += '<tbody>';
  for (let r = 0; r < rows - 1; r++) {
    html += '<tr>';
    for (let c = 0; c < cols; c++) {
      html += `<td style="border:1px solid #cbd5e1; padding:8px 12px;">Data ${r + 1}-${c + 1}</td>`;
    }
    html += '</tr>';
  }
  html += '</tbody></table><p><br></p>';

  execCmd('insertHTML', html);
}

// Quick Table Modifiers
function getActiveCell() {
  const sel = window.getSelection();
  if (!sel || !sel.anchorNode) return null;
  let node = sel.anchorNode;
  while (node && node !== editorRef.value) {
    if (node.tagName === 'TD' || node.tagName === 'TH') return node;
    node = node.parentNode;
  }
  return null;
}

function addTableRow() {
  const cell = getActiveCell();
  if (!cell) {
    toast.warning('Klik di dalam tabel terlebih dahulu untuk menambah baris.');
    return;
  }
  const row = cell.parentElement;
  const colCount = row.children.length;
  const newRow = document.createElement('tr');
  for (let i = 0; i < colCount; i++) {
    const td = document.createElement('td');
    td.style.border = '1px solid #cbd5e1';
    td.style.padding = '8px 12px';
    td.innerHTML = '&nbsp;';
    newRow.appendChild(td);
  }
  row.parentElement.insertBefore(newRow, row.nextSibling);
  handleInput();
}

function addTableColumn() {
  const cell = getActiveCell();
  if (!cell) {
    toast.warning('Klik di dalam tabel terlebih dahulu untuk menambah kolom.');
    return;
  }
  const table = cell.closest('table');
  if (!table) return;

  const rows = table.querySelectorAll('tr');
  rows.forEach((tr, idx) => {
    const isHead = tr.parentElement.tagName === 'THEAD' || idx === 0;
    const newCell = document.createElement(isHead ? 'th' : 'td');
    newCell.style.border = '1px solid #cbd5e1';
    newCell.style.padding = '8px 12px';
    if (isHead) {
      newCell.style.background = '#f1f5f9';
      newCell.style.fontWeight = '600';
      newCell.innerHTML = 'Kolom Baru';
    } else {
      newCell.innerHTML = '&nbsp;';
    }
    tr.appendChild(newCell);
  });
  handleInput();
}

function deleteCurrentRow() {
  const cell = getActiveCell();
  if (!cell) return;
  const row = cell.parentElement;
  const table = cell.closest('table');
  row.remove();
  if (table && table.querySelectorAll('tr').length === 0) {
    table.remove();
  }
  handleInput();
}

function deleteCurrentTable() {
  const cell = getActiveCell();
  if (!cell) return;
  const table = cell.closest('table');
  if (table) {
    table.remove();
    handleInput();
  }
}

// Callout / Info Panel
function insertCallout(type = 'info') {
  isMoreMenuOpen.value = false;
  restoreSelection();

  const configs = {
    info: {
      bg: '#eff6ff',
      border: '#3b82f6',
      icon: 'ℹ️',
      title: 'Info',
      color: '#1e40af',
    },
    warning: {
      bg: '#fffbeb',
      border: '#f59e0b',
      icon: '⚠️',
      title: 'Perhatian',
      color: '#92400e',
    },
    success: {
      bg: '#f0fdf4',
      border: '#22c55e',
      icon: '✅',
      title: 'Catatan Selesai',
      color: '#166534',
    },
  };

  const cfg = configs[type] || configs.info;
  const html = `
    <div style="background:${cfg.bg}; border-left:4px solid ${cfg.border}; border-radius:8px; padding:12px 16px; margin:1rem 0; color:#1e293b;">
      <strong style="color:${cfg.color}; display:flex; align-items:center; gap:6px; margin-bottom:4px;">
        <span>${cfg.icon}</span> ${cfg.title}
      </strong>
      <p style="margin:0; font-size:14px;">Tulis catatan penting atau detail peringatan di sini...</p>
    </div><p><br></p>
  `;
  execCmd('insertHTML', html);
}

// Horizontal divider
function insertDivider() {
  isMoreMenuOpen.value = false;
  restoreSelection();
  execCmd('insertHorizontalRule');
}

// Image Handling
function openImageModal() {
  saveSelection();
  imageUrlInput.value = '';
  isImageModalOpen.value = true;
}

function handleFileInput(e) {
  const files = e.target.files;
  if (!files || files.length === 0) return;
  const file = files[0];
  insertImageFile(file);
  e.target.value = '';
  isImageModalOpen.value = false;
}

function insertImageFile(file) {
  if (!file.type.startsWith('image/')) {
    toast.error('Hanya file gambar (PNG, JPG, WebP, GIF) yang didukung.');
    return;
  }
  const reader = new FileReader();
  reader.onload = (event) => {
    restoreSelection();
    const dataUrl = event.target.result;
    const html = `<div style="margin:1rem 0;"><img src="${dataUrl}" style="max-width:100%; border-radius:8px; border:1px solid #e2e8f0; box-shadow:0 4px 6px -1px rgba(0,0,0,0.1);" alt="Lampiran gambar" /><p style="font-size:11px; color:#64748b; margin-top:4px;">📸 ${file.name}</p></div><p><br></p>`;
    execCmd('insertHTML', html);
  };
  reader.readAsDataURL(file);
}

function insertImageByUrl() {
  if (!imageUrlInput.value.trim()) return;
  restoreSelection();
  const url = imageUrlInput.value.trim();
  const html = `<div style="margin:1rem 0;"><img src="${url}" style="max-width:100%; border-radius:8px; border:1px solid #e2e8f0; box-shadow:0 4px 6px -1px rgba(0,0,0,0.1);" alt="Lampiran gambar" /></div><p><br></p>`;
  execCmd('insertHTML', html);
  isImageModalOpen.value = false;
}

// Clipboard Paste Handler (e.g. screenshot Win+Shift+S / PrtScn)
function handlePaste(e) {
  const clipboardData = e.clipboardData;
  if (!clipboardData) return;

  // Check for image items
  const items = clipboardData.items;
  if (items) {
    for (let i = 0; i < items.length; i++) {
      if (items[i].type.indexOf('image') !== -1) {
        const file = items[i].getAsFile();
        if (file) {
          e.preventDefault();
          insertImageFile(file);
          return;
        }
      }
    }
  }
}

// Drag & drop image files
function handleDrop(e) {
  e.preventDefault();
  if (e.dataTransfer && e.dataTransfer.files && e.dataTransfer.files.length > 0) {
    const file = e.dataTransfer.files[0];
    if (file.type.startsWith('image/')) {
      insertImageFile(file);
    }
  }
}

// Link Handling
function openLinkModal() {
  saveSelection();
  const sel = window.getSelection();
  linkTextInput.value = sel ? sel.toString() : '';
  linkUrlInput.value = 'https://';
  isLinkModalOpen.value = true;
}

function insertLink() {
  if (!linkUrlInput.value.trim()) return;
  restoreSelection();
  const url = linkUrlInput.value.trim();
  const text = linkTextInput.value.trim() || url;
  const html = `<a href="${url}" target="_blank" rel="noopener noreferrer" style="color:#2563eb; text-decoration:underline; font-weight:500;">${text}</a>`;
  execCmd('insertHTML', html);
  isLinkModalOpen.value = false;
}

// Raw HTML Mode toggle
function toggleRawMode() {
  if (!isRawMode.value) {
    rawHtml.value = editorRef.value ? editorRef.value.innerHTML : '';
    isRawMode.value = true;
  } else {
    if (editorRef.value) {
      editorRef.value.innerHTML = rawHtml.value;
    }
    isRawMode.value = false;
    handleInput();
  }
}

function handleRawInput(e) {
  rawHtml.value = e.target.value;
  emit('update:modelValue', rawHtml.value);
  hasUnsavedChanges.value = true;
  isSaved.value = false;
}

// Fullscreen toggle
function toggleFullscreen() {
  isFullscreen.value = !isFullscreen.value;
}

function handleOutsideClick(e) {
  if (!e.target.closest('.dropdown-container')) {
    isStyleMenuOpen.value = false;
    isMoreMenuOpen.value = false;
    isColorMenuOpen.value = false;
    isTableMenuOpen.value = false;
  }
}
</script>

<template>
  <div
    class="flex flex-col bg-white border border-slate-300 rounded-xl shadow-xs transition-all duration-200"
    :class="{
      'fixed inset-4 z-50 shadow-2xl rounded-2xl border-slate-400': isFullscreen,
      'hover:border-slate-400': !isFullscreen,
      'ring-2 ring-blue-500/20 border-blue-500': isFocused,
    }"
  >
    <!-- ── EDITOR TOP TOOLBAR ── -->
    <div
      class="flex items-center flex-wrap gap-1 px-3 py-2 bg-slate-50 border-b border-slate-200 rounded-t-xl select-none text-slate-700"
    >
      <!-- ① Style Dropdown (Heading / Paragraph / Code) -->
      <div class="relative dropdown-container">
        <button
          type="button"
          @click="isStyleMenuOpen = !isStyleMenuOpen"
          class="flex items-center gap-1 px-2.5 py-1.5 rounded-lg text-xs font-bold text-slate-700 hover:bg-slate-200/70 border border-slate-200 bg-white transition shadow-2xs"
          title="Gaya Teks (Heading / Paragraph)"
        >
          <span class="font-serif text-sm font-bold">T</span>
          <ChevronDown class="w-3.5 h-3.5 text-slate-500" />
        </button>

        <div
          v-if="isStyleMenuOpen"
          class="absolute left-0 top-full mt-1 w-44 bg-white rounded-xl border border-slate-200 shadow-xl py-1 z-50 animate-slide-up"
        >
          <button
            type="button"
            @click="setBlock('p')"
            class="w-full text-left px-3 py-1.5 text-xs text-slate-700 hover:bg-slate-100 flex items-center justify-between"
          >
            <span>Normal text</span>
            <span class="text-[10px] text-slate-400">Ctrl+Alt+0</span>
          </button>
          <button
            type="button"
            @click="setBlock('h1')"
            class="w-full text-left px-3 py-1.5 text-sm font-bold text-slate-900 hover:bg-slate-100 flex items-center justify-between"
          >
            <span>Heading 1</span>
            <span class="text-[10px] text-slate-400">Ctrl+Alt+1</span>
          </button>
          <button
            type="button"
            @click="setBlock('h2')"
            class="w-full text-left px-3 py-1.5 text-xs font-bold text-slate-800 hover:bg-slate-100 flex items-center justify-between"
          >
            <span>Heading 2</span>
            <span class="text-[10px] text-slate-400">Ctrl+Alt+2</span>
          </button>
          <button
            type="button"
            @click="setBlock('h3')"
            class="w-full text-left px-3 py-1.5 text-xs font-semibold text-slate-700 hover:bg-slate-100 flex items-center justify-between"
          >
            <span>Heading 3</span>
            <span class="text-[10px] text-slate-400">Ctrl+Alt+3</span>
          </button>
          <div class="h-px bg-slate-100 my-1"></div>
          <button
            type="button"
            @click="setBlock('pre')"
            class="w-full text-left px-3 py-1.5 text-xs font-mono text-slate-700 hover:bg-slate-100"
          >
            &lt;/&gt; Code block
          </button>
          <button
            type="button"
            @click="setBlock('blockquote')"
            class="w-full text-left px-3 py-1.5 text-xs italic text-slate-600 hover:bg-slate-100"
          >
            “ Quote block
          </button>
        </div>
      </div>

      <!-- Divider -->
      <div class="h-4 w-px bg-slate-300 mx-0.5"></div>

      <!-- ② Bold, Italic, Underline, Strike -->
      <button
        type="button"
        @click="execCmd('bold')"
        class="p-1.5 rounded-lg text-slate-700 hover:bg-slate-200 hover:text-slate-900 transition font-bold"
        title="Tebal (Ctrl+B)"
      >
        <Bold class="w-4 h-4" />
      </button>

      <button
        type="button"
        @click="execCmd('italic')"
        class="p-1.5 rounded-lg text-slate-700 hover:bg-slate-200 hover:text-slate-900 transition"
        title="Miring (Ctrl+I)"
      >
        <Italic class="w-4 h-4" />
      </button>

      <button
        type="button"
        @click="execCmd('underline')"
        class="p-1.5 rounded-lg text-slate-700 hover:bg-slate-200 hover:text-slate-900 transition"
        title="Garis Bawah (Ctrl+U)"
      >
        <Underline class="w-4 h-4" />
      </button>

      <button
        type="button"
        @click="execCmd('strikeThrough')"
        class="p-1.5 rounded-lg text-slate-700 hover:bg-slate-200 hover:text-slate-900 transition"
        title="Coretan (Strikethrough)"
      >
        <Strikethrough class="w-4 h-4" />
      </button>

      <!-- Divider -->
      <div class="h-4 w-px bg-slate-300 mx-0.5"></div>

      <!-- ③ Lists: Bullet & Numbered -->
      <button
        type="button"
        @click="execCmd('insertUnorderedList')"
        class="p-1.5 rounded-lg text-slate-700 hover:bg-slate-200 hover:text-slate-900 transition"
        title="Daftar Bullet"
      >
        <List class="w-4 h-4" />
      </button>

      <button
        type="button"
        @click="execCmd('insertOrderedList')"
        class="p-1.5 rounded-lg text-slate-700 hover:bg-slate-200 hover:text-slate-900 transition"
        title="Daftar Nomor"
      >
        <ListOrdered class="w-4 h-4" />
      </button>

      <!-- Divider -->
      <div class="h-4 w-px bg-slate-300 mx-0.5"></div>

      <!-- ④ Color Picker Popover (Text & Highlight Color) -->
      <div class="relative dropdown-container">
        <button
          type="button"
          @click="isColorMenuOpen = !isColorMenuOpen"
          class="flex items-center gap-1 p-1.5 rounded-lg text-slate-700 hover:bg-slate-200 transition"
          title="Warna Teks & Sorotan"
        >
          <span class="font-bold underline text-sm leading-none text-blue-600">A</span>
          <ChevronDown class="w-3 h-3 text-slate-400" />
        </button>

        <div
          v-if="isColorMenuOpen"
          class="absolute left-0 top-full mt-1 w-56 bg-white rounded-xl border border-slate-200 shadow-xl p-3 z-50 animate-slide-up"
        >
          <!-- Text Color -->
          <div class="mb-3">
            <span class="text-[11px] font-bold text-slate-600 uppercase tracking-wider block mb-1.5">
              Warna Teks
            </span>
            <div class="grid grid-cols-4 gap-1.5">
              <button
                v-for="c in textColors"
                :key="c.value"
                type="button"
                @click="applyTextColor(c.value)"
                class="w-7 h-7 rounded-lg border border-slate-200 hover:scale-110 transition flex items-center justify-center shadow-2xs"
                :style="{ backgroundColor: c.value }"
                :title="c.name"
              ></button>
            </div>
          </div>

          <!-- Highlight Color -->
          <div>
            <span class="text-[11px] font-bold text-slate-600 uppercase tracking-wider block mb-1.5">
              Sorotan Latar (Highlight)
            </span>
            <div class="grid grid-cols-4 gap-1.5">
              <button
                v-for="bg in bgColors"
                :key="bg.value"
                type="button"
                @click="applyBgColor(bg.value)"
                class="w-7 h-7 rounded-lg border border-slate-300 hover:scale-110 transition flex items-center justify-center shadow-2xs"
                :style="{ backgroundColor: bg.value }"
                :title="bg.name"
              >
                <span v-if="bg.value === 'transparent'" class="text-[9px] text-slate-400">∅</span>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Divider -->
      <div class="h-4 w-px bg-slate-300 mx-0.5"></div>

      <!-- ⑤ Image Attachment (Image Attachment) -->
      <button
        type="button"
        @click="openImageModal"
        class="flex items-center gap-1 px-2 py-1.5 rounded-lg text-slate-700 hover:bg-blue-50 hover:text-blue-700 transition"
        title="Lampirkan Gambar (Bisa Paste Gambar Langsung Ctrl+V!)"
      >
        <ImageIcon class="w-4 h-4 text-emerald-600" />
        <span class="text-xs font-semibold hidden sm:inline">Gambar</span>
      </button>

      <!-- ⑥ Code Snippet -->
      <button
        type="button"
        @click="execCmd('insertHTML', '<code>kode_disini</code>')"
        class="p-1.5 rounded-lg text-slate-700 hover:bg-slate-200 hover:text-slate-900 transition font-mono text-xs font-bold"
        title="Inline Code `code`"
      >
        <Code class="w-4 h-4" />
      </button>

      <!-- ⑦ Hyperlink -->
      <button
        type="button"
        @click="openLinkModal"
        class="p-1.5 rounded-lg text-slate-700 hover:bg-slate-200 hover:text-slate-900 transition"
        title="Sisipkan Tautan Link (Ctrl+K)"
      >
        <LinkIcon class="w-4 h-4" />
      </button>

      <!-- ⑧ Insert Table Dropdown -->
      <div class="relative dropdown-container">
        <button
          type="button"
          @click="isTableMenuOpen = !isTableMenuOpen"
          class="flex items-center gap-1 px-2 py-1.5 rounded-lg text-slate-700 hover:bg-blue-50 hover:text-blue-700 transition"
          title="Sisipkan Tabel Spesifikasi"
        >
          <TableIcon class="w-4 h-4 text-blue-600" />
          <span class="text-xs font-semibold hidden sm:inline">Tabel</span>
          <ChevronDown class="w-3 h-3 text-slate-400" />
        </button>

        <div
          v-if="isTableMenuOpen"
          class="absolute left-0 top-full mt-1 w-64 bg-white rounded-xl border border-slate-200 shadow-xl p-3 z-50 animate-slide-up"
        >
          <span class="text-xs font-bold text-slate-700 block mb-2">Buat Tabel Baru</span>
          <div class="grid grid-cols-2 gap-2 mb-3">
            <div>
              <label class="text-[11px] text-slate-500 font-semibold block mb-1">Baris</label>
              <input
                v-model.number="tableRows"
                type="number"
                min="1"
                max="20"
                class="w-full bg-slate-50 border border-slate-300 rounded-lg px-2.5 py-1 text-xs text-slate-800"
              />
            </div>
            <div>
              <label class="text-[11px] text-slate-500 font-semibold block mb-1">Kolom</label>
              <input
                v-model.number="tableCols"
                type="number"
                min="1"
                max="10"
                class="w-full bg-slate-50 border border-slate-300 rounded-lg px-2.5 py-1 text-xs text-slate-800"
              />
            </div>
          </div>
          <button
            type="button"
            @click="insertTable"
            class="w-full py-1.5 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-xs font-semibold transition"
          >
            Sisipkan Tabel {{ tableRows }}x{{ tableCols }}
          </button>
        </div>
      </div>

      <!-- ⑨ Insert More (+) Menu (Panels, Divider, etc) -->
      <div class="relative dropdown-container">
        <button
          type="button"
          @click="isMoreMenuOpen = !isMoreMenuOpen"
          class="flex items-center gap-1 p-1.5 rounded-lg text-slate-700 hover:bg-slate-200 transition"
          title="Elemen Tambahan (+)"
        >
          <Plus class="w-4 h-4 text-purple-600" />
          <ChevronDown class="w-3 h-3 text-slate-400" />
        </button>

        <div
          v-if="isMoreMenuOpen"
          class="absolute left-0 top-full mt-1 w-52 bg-white rounded-xl border border-slate-200 shadow-xl py-1 z-50 animate-slide-up"
        >
          <button
            type="button"
            @click="insertCallout('info')"
            class="w-full text-left px-3 py-1.5 text-xs text-slate-700 hover:bg-blue-50 flex items-center gap-2"
          >
            <span>ℹ️</span> <span>Info Callout</span>
          </button>
          <button
            type="button"
            @click="insertCallout('warning')"
            class="w-full text-left px-3 py-1.5 text-xs text-slate-700 hover:bg-amber-50 flex items-center gap-2"
          >
            <span>⚠️</span> <span>Warning Callout</span>
          </button>
          <button
            type="button"
            @click="insertCallout('success')"
            class="w-full text-left px-3 py-1.5 text-xs text-slate-700 hover:bg-emerald-50 flex items-center gap-2"
          >
            <span>✅</span> <span>Success Box</span>
          </button>
          <div class="h-px bg-slate-100 my-1"></div>
          <button
            type="button"
            @click="insertDivider"
            class="w-full text-left px-3 py-1.5 text-xs text-slate-700 hover:bg-slate-100 flex items-center gap-2"
          >
            <span>➖</span> <span>Garis Pembatas (Divider)</span>
          </button>
        </div>
      </div>

      <!-- Right tools: Undo, Redo, Raw HTML, Fullscreen -->
      <div class="flex items-center gap-1 ml-auto">
        <button
          type="button"
          @click="execCmd('undo')"
          class="p-1.5 rounded-lg text-slate-600 hover:bg-slate-200 transition"
          title="Undo (Ctrl+Z)"
        >
          <Undo class="w-3.5 h-3.5" />
        </button>

        <button
          type="button"
          @click="execCmd('redo')"
          class="p-1.5 rounded-lg text-slate-600 hover:bg-slate-200 transition"
          title="Redo (Ctrl+Y)"
        >
          <Redo class="w-3.5 h-3.5" />
        </button>

        <div class="h-4 w-px bg-slate-300 mx-0.5"></div>

        <!-- Raw HTML view toggle -->
        <button
          type="button"
          @click="toggleRawMode"
          class="px-2 py-1 rounded text-xs font-mono transition"
          :class="isRawMode ? 'bg-blue-600 text-white font-bold' : 'text-slate-600 hover:bg-slate-200'"
          title="Lihat Kode HTML"
        >
          &lt;/&gt; HTML
        </button>

        <!-- Fullscreen expand button -->
        <button
          type="button"
          @click="toggleFullscreen"
          class="p-1.5 rounded-lg text-slate-600 hover:bg-slate-200 transition"
          :title="isFullscreen ? 'Kecilkan' : 'Perbesar Layar Penuh (Word Mode)'"
        >
          <Minimize2 v-if="isFullscreen" class="w-3.5 h-3.5 text-blue-600" />
          <Maximize2 v-else class="w-3.5 h-3.5" />
        </button>
      </div>
    </div>

    <!-- ── TABLE CONTEXTUAL QUICK BAR (Shown only when cursor is inside a table) ── -->
    <div
      v-show="isInsideTable"
      class="bg-blue-50/70 border-b border-blue-200 px-3 py-1 flex items-center gap-2 text-xs text-blue-900"
    >
      <span class="font-semibold flex items-center gap-1 text-[11px] text-blue-700">
        <TableIcon class="w-3 h-3 text-blue-600" /> Kontrol Tabel:
      </span>
      <button
        type="button"
        @click="addTableRow"
        class="px-2 py-0.5 bg-white hover:bg-blue-100 border border-blue-200 rounded text-[11px] font-semibold text-blue-800 transition"
      >
        + Baris
      </button>
      <button
        type="button"
        @click="addTableColumn"
        class="px-2 py-0.5 bg-white hover:bg-blue-100 border border-blue-200 rounded text-[11px] font-semibold text-blue-800 transition"
      >
        + Kolom
      </button>
      <button
        type="button"
        @click="deleteCurrentRow"
        class="px-2 py-0.5 bg-white hover:bg-red-50 border border-red-200 rounded text-[11px] font-semibold text-red-600 transition"
      >
        - Baris
      </button>
      <button
        type="button"
        @click="deleteCurrentTable"
        class="px-2 py-0.5 bg-white hover:bg-red-50 border border-red-200 rounded text-[11px] font-semibold text-red-600 transition"
      >
        Hapus Tabel
      </button>
    </div>

    <!-- ── MAIN EDITOR AREA ── -->
    <div
      class="relative flex-1 p-4 overflow-y-auto"
      :class="isFullscreen ? 'min-h-[500px]' : 'min-h-[280px] max-h-[500px]'"
      @drop="handleDrop"
      @dragover.prevent
    >
      <!-- Visual ContentEditable Editor -->
      <div
        v-show="!isRawMode"
        ref="editorRef"
        contenteditable="true"
        class="editor-content w-full h-full text-sm text-slate-900 leading-relaxed outline-none min-h-[240px]"
        @input="handleInput"
        @focus="handleFocus"
        @blur="handleBlur"
        @paste="handlePaste"
        :data-placeholder="placeholder"
      ></div>

      <!-- Raw HTML Code Editor -->
      <textarea
        v-if="isRawMode"
        :value="rawHtml"
        @input="handleRawInput"
        class="w-full h-full min-h-[240px] font-mono text-xs text-slate-800 bg-slate-50 border border-slate-300 rounded-lg p-3 outline-none focus:border-blue-500"
      ></textarea>
    </div>

    <!-- ── BOTTOM STATUS BAR ── -->
    <div
      class="flex items-center justify-between px-3 py-2 bg-slate-50/80 border-t border-slate-200 rounded-b-xl text-xs text-slate-500"
    >
      <div class="flex items-center gap-3">
        <span>{{ wordCount }} kata</span>
        <span>•</span>
        <span>{{ charCount }} karakter</span>
        <span>•</span>
        <span class="flex items-center gap-1 text-[11px]">
          <span
            class="w-2 h-2 rounded-full"
            :class="isSaved ? 'bg-emerald-500' : 'bg-amber-500 animate-pulse'"
          ></span>
          {{ isSaved ? 'Tersimpan otomatis' : 'Menyimpan...' }}
        </span>
      </div>

      <div class="flex items-center gap-2">
        <button
          v-if="hasUnsavedChanges"
          type="button"
          @click="handleManualSave"
          class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-xs font-semibold flex items-center gap-1 shadow-xs transition"
        >
          <Check class="w-3.5 h-3.5" />
          <span>Simpan Perubahan</span>
        </button>
      </div>
    </div>

    <!-- ── MODAL: IMAGE ATTACHMENT ── -->
    <div
      v-if="isImageModalOpen"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/40 backdrop-blur-xs"
    >
      <div class="bg-white rounded-2xl border border-slate-200 shadow-2xl p-5 w-full max-w-md animate-slide-up space-y-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <ImageIcon class="w-5 h-5 text-emerald-600" />
            <h3 class="text-sm font-bold text-slate-800">Lampirkan Gambar</h3>
          </div>
          <button @click="isImageModalOpen = false" class="text-slate-400 hover:text-slate-600">✕</button>
        </div>

        <!-- Option 1: Upload from device -->
        <div>
          <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider mb-1.5">
            Unggah dari Komputer
          </label>
          <div
            @click="fileInputRef.click()"
            class="border-2 border-dashed border-slate-300 hover:border-blue-500 rounded-xl p-4 text-center cursor-pointer bg-slate-50 hover:bg-blue-50/50 transition group"
          >
            <Upload class="w-8 h-8 text-slate-400 group-hover:text-blue-600 mx-auto mb-1 transition" />
            <p class="text-xs font-semibold text-slate-700">Klik untuk memilih file gambar</p>
            <p class="text-[11px] text-slate-400">PNG, JPG, WebP, GIF (Max 10MB)</p>
          </div>
          <input
            ref="fileInputRef"
            type="file"
            accept="image/*"
            class="hidden"
            @change="handleFileInput"
          />
        </div>

        <!-- Option 2: Paste from clipboard notice -->
        <div class="p-3 bg-blue-50 border border-blue-200 rounded-xl text-xs text-blue-800 flex items-start gap-2">
          <Sparkles class="w-4 h-4 text-blue-600 shrink-0 mt-0.5" />
          <p>
            <strong>Tips Pintar:</strong> Anda juga bisa menempelkan (<strong>Ctrl+V</strong>) tangkapan layar (screenshot) langsung di dalam editor teks tanpa perlu mengunggah file!
          </p>
        </div>

        <!-- Option 3: Image URL -->
        <div>
          <label class="block text-xs font-bold text-slate-600 uppercase tracking-wider mb-1.5">
            Atau masukkan URL Gambar
          </label>
          <div class="flex gap-2">
            <input
              v-model="imageUrlInput"
              placeholder="https://example.com/image.png"
              class="flex-1 bg-slate-50 border border-slate-300 rounded-lg px-3 py-1.5 text-xs text-slate-800 focus:outline-none focus:border-blue-500"
              @keyup.enter="insertImageByUrl"
            />
            <button
              @click="insertImageByUrl"
              class="px-3 py-1.5 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-xs font-semibold transition"
            >
              Sisipkan
            </button>
          </div>
        </div>

        <div class="flex justify-end pt-2">
          <button
            @click="isImageModalOpen = false"
            class="px-4 py-1.5 text-xs font-semibold text-slate-600 hover:bg-slate-100 rounded-lg"
          >
            Tutup
          </button>
        </div>
      </div>
    </div>

    <!-- ── MODAL: HYPERLINK ── -->
    <div
      v-if="isLinkModalOpen"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/40 backdrop-blur-xs"
    >
      <div class="bg-white rounded-2xl border border-slate-200 shadow-2xl p-5 w-full max-w-sm animate-slide-up space-y-3">
        <div class="flex items-center justify-between">
          <h3 class="text-sm font-bold text-slate-800">Sisipkan Tautan (Link)</h3>
          <button @click="isLinkModalOpen = false" class="text-slate-400 hover:text-slate-600">✕</button>
        </div>

        <div>
          <label class="block text-xs font-semibold text-slate-600 mb-1">Teks Tautan</label>
          <input
            v-model="linkTextInput"
            placeholder="Teks yang akan ditampilkan..."
            class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-1.5 text-xs text-slate-800 focus:outline-none focus:border-blue-500"
          />
        </div>

        <div>
          <label class="block text-xs font-semibold text-slate-600 mb-1">URL / Link Web</label>
          <input
            v-model="linkUrlInput"
            placeholder="https://..."
            class="w-full bg-slate-50 border border-slate-300 rounded-lg px-3 py-1.5 text-xs text-slate-800 focus:outline-none focus:border-blue-500"
            @keyup.enter="insertLink"
          />
        </div>

        <div class="flex justify-end gap-2 pt-2">
          <button
            @click="isLinkModalOpen = false"
            class="px-3 py-1.5 text-xs text-slate-600 hover:bg-slate-100 rounded-lg"
          >
            Batal
          </button>
          <button
            @click="insertLink"
            class="px-4 py-1.5 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-xs font-semibold shadow-xs"
          >
            Simpan Link
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Placeholder styling for ContentEditable */
[contenteditable="true"]:empty:before {
  content: attr(data-placeholder);
  color: #94a3b8;
  pointer-events: none;
  display: block;
}
</style>
