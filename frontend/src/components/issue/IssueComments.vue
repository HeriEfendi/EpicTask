<script setup>
import { ref } from 'vue';
import { Send } from 'lucide-vue-next';

const props = defineProps({
  comments: {
    type: Array,
    default: () => []
  },
  currentUserAvatar: {
    type: String,
    default: 'https://api.dicebear.com/7.x/avataaars/svg?seed=user'
  }
});

const emit = defineEmits(['add-comment']);

const newCommentText = ref('');

function handleSend() {
  if (!newCommentText.value.trim()) return;
  emit('add-comment', newCommentText.value.trim());
  newCommentText.value = '';
}
</script>

<template>
  <div class="space-y-3.5">
    <!-- Add Comment Box -->
    <div class="flex items-start space-x-3">
      <img
        :src="currentUserAvatar"
        class="w-8 h-8 rounded-full bg-slate-200 ring-1 ring-slate-300 shrink-0 mt-1"
        alt="Avatar"
      />
      <div class="flex-1 space-y-2">
        <textarea
          v-model="newCommentText"
          rows="2"
          placeholder="Tulis komentar... (Ketik @nama untuk me-mention anggota tim)"
          class="w-full bg-white border border-slate-300 rounded-xl p-3 text-sm text-slate-800 placeholder-slate-400 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 shadow-2xs"
        ></textarea>
        <div class="flex justify-between items-center">
          <span class="text-xs text-slate-500">Tip: @mention mengirimkan notifikasi instan</span>
          <button
            @click="handleSend"
            class="px-4 py-1.5 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-lg text-sm font-semibold flex items-center space-x-1.5 shadow-xs transition"
          >
            <Send class="w-3.5 h-3.5" />
            <span>Kirim</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Comments List -->
    <div class="space-y-2.5 max-h-56 overflow-y-auto pt-2">
      <div
        v-for="comment in comments"
        :key="comment.id"
        class="p-3.5 bg-slate-50 border border-slate-200 rounded-xl flex items-start space-x-3"
      >
        <img
          :src="comment.user_avatar || 'https://api.dicebear.com/7.x/avataaars/svg?seed=' + comment.id"
          class="w-7 h-7 rounded-full bg-slate-200 shrink-0 mt-0.5"
          alt="User"
        />
        <div class="flex-1">
          <div class="flex items-center justify-between mb-1">
            <span class="text-sm font-bold text-slate-800">{{ comment.user_name || 'Anggota' }}</span>
            <span class="text-xs text-slate-500">{{ new Date(comment.created_at).toLocaleString() }}</span>
          </div>
          <p class="text-sm text-slate-700 whitespace-pre-wrap leading-relaxed">{{ comment.body }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

