<script setup>
import { computed } from 'vue';
import { useReportStore } from '@/stores/report';

const reportStore = useReportStore();

const maxVelocityPoints = computed(() => {
  if (!reportStore.velocity?.sprints?.length) return 100;
  const max = Math.max(
    ...reportStore.velocity.sprints.map((s) => Math.max(s.committed_points || 0, s.completed_points || 0))
  );
  return Math.ceil((max + 10) / 10) * 10;
});
</script>

<template>
  <div class="space-y-6">
    <div class="bg-white rounded-xl p-6 border border-slate-200/90 shadow-2xs">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-4 border-b border-slate-100 gap-3">
        <div>
          <div class="flex items-center space-x-2">
            <h2 class="text-base font-bold text-slate-900">Sprint Velocity Chart (Jan 2026 - Okt 2026)</h2>
            <span class="text-xs bg-emerald-100 text-emerald-800 px-2 py-0.5 rounded font-semibold">Agile Scrum Standard</span>
          </div>
          <p class="text-xs text-slate-500 mt-1">
            Membandingkan perkiraan Story Points yang direncanakan (Committed) dengan poin yang berhasil diselesaikan (Completed) per sprint/bulan.
          </p>
        </div>

        <!-- Legend -->
        <div class="flex items-center space-x-4 text-xs font-semibold">
          <div class="flex items-center space-x-1.5">
            <span class="w-3.5 h-3.5 bg-slate-300 rounded-sm inline-block"></span>
            <span class="text-slate-600">Committed Points</span>
          </div>
          <div class="flex items-center space-x-1.5">
            <span class="w-3.5 h-3.5 bg-emerald-500 rounded-sm inline-block"></span>
            <span class="text-emerald-700 font-bold">Completed Points</span>
          </div>
        </div>
      </div>

      <!-- SVG Velocity Bar Chart -->
      <div class="mt-6 w-full overflow-x-auto">
        <div class="min-w-[700px] h-72 flex flex-col justify-end">
          <!-- Bars Container -->
          <div class="flex-1 flex items-end justify-between px-6 pb-2 border-b border-slate-200 relative">
            <!-- Background Grid Lines -->
            <div class="absolute inset-0 flex flex-col justify-between pointer-events-none opacity-20">
              <div class="border-b border-slate-400 w-full"></div>
              <div class="border-b border-slate-400 w-full"></div>
              <div class="border-b border-slate-400 w-full"></div>
              <div class="border-b border-slate-400 w-full"></div>
            </div>

            <!-- Monthly Bars Group -->
            <div
              v-for="s in reportStore.velocity?.sprints"
              :key="s.month"
              class="flex flex-col items-center group relative z-10"
            >
              <!-- Tooltip Hover -->
              <div class="absolute -top-16 opacity-0 group-hover:opacity-100 bg-slate-900 text-white text-[11px] rounded-lg px-2.5 py-1.5 transition pointer-events-none shadow-lg whitespace-nowrap z-30">
                <p class="font-bold">{{ s.month }}</p>
                <p class="text-slate-300">Committed: {{ s.committed_points }} pts | Completed: {{ s.completed_points }} pts</p>
                <p class="text-emerald-400 font-semibold">Presisi: {{ s.predictability_pct }}%</p>
              </div>

              <!-- The Pair of Bars -->
              <div class="flex items-end space-x-1.5 h-56">
                <!-- Committed Bar -->
                <div
                  class="w-6 sm:w-8 bg-slate-300 group-hover:bg-slate-400 rounded-t transition-all duration-300 flex items-center justify-center text-[10px] font-bold text-slate-700 pb-1"
                  :style="{ height: `${((s.committed_points || 0) / maxVelocityPoints) * 100}%` }"
                >
                  <span v-if="s.committed_points > 15">{{ s.committed_points }}</span>
                </div>

                <!-- Completed Bar -->
                <div
                  class="w-6 sm:w-8 bg-emerald-500 group-hover:bg-emerald-600 rounded-t transition-all duration-300 flex items-center justify-center text-[10px] font-bold text-white pb-1 shadow-xs"
                  :style="{ height: `${((s.completed_points || 0) / maxVelocityPoints) * 100}%` }"
                >
                  <span v-if="s.completed_points > 15">{{ s.completed_points }}</span>
                </div>
              </div>

              <!-- Month Label -->
              <span class="mt-2 text-[11px] font-semibold text-slate-600 group-hover:text-blue-600 font-mono">
                {{ s.month.replace('2026-', 'M') }}
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Velocity Data Table -->
      <div class="mt-6 border-t border-slate-100 pt-4">
        <h3 class="text-xs font-bold text-slate-700 uppercase tracking-wider mb-2">Rincian Riwayat Sprint Velocity</h3>
        <div class="overflow-x-auto">
          <table class="w-full text-left text-xs">
            <thead class="bg-slate-50 text-slate-600 font-bold uppercase tracking-wider border-y border-slate-200">
              <tr>
                <th class="py-2.5 px-3">Periode / Sprint</th>
                <th class="py-2.5 px-3 text-center">Tiket Dibuat</th>
                <th class="py-2.5 px-3 text-center">Tiket Resolved</th>
                <th class="py-2.5 px-3 text-center">Committed Points</th>
                <th class="py-2.5 px-3 text-center">Completed Points</th>
                <th class="py-2.5 px-3 text-center">Tingkat Prediktabilitas</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-slate-100">
              <tr
                v-for="s in reportStore.velocity?.sprints"
                :key="s.month"
                class="hover:bg-slate-50 transition"
              >
                <td class="py-2.5 px-3 font-semibold text-slate-800 font-mono">
                  {{ s.month }} (Sprint)
                </td>
                <td class="py-2.5 px-3 text-center font-mono">{{ s.created_count }}</td>
                <td class="py-2.5 px-3 text-center font-mono text-emerald-600 font-bold">{{ s.resolved_count }}</td>
                <td class="py-2.5 px-3 text-center font-mono text-slate-600 font-semibold">{{ s.committed_points }} pts</td>
                <td class="py-2.5 px-3 text-center font-mono text-emerald-600 font-bold">{{ s.completed_points }} pts</td>
                <td class="py-2.5 px-3 text-center">
                  <span
                    class="px-2 py-0.5 rounded text-[11px] font-bold"
                    :class="s.predictability_pct >= 90 ? 'bg-emerald-100 text-emerald-800' : (s.predictability_pct >= 70 ? 'bg-amber-100 text-amber-800' : 'bg-blue-100 text-blue-800')"
                  >
                    {{ s.predictability_pct }}%
                  </span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

