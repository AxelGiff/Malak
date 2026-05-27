
<template>
    <div class="rounded-2xl border border-white/10 bg-slate-950/70 p-6 shadow-xl shadow-slate-950/30 backdrop-blur">
        <div class="mb-6">
			<p class="mt-1 text-2xl font-semibold text-white">MÉMOIRE</p>
        </div>

        <div class="flex items-center gap-6">
            <div class="relative flex shrink-0 items-center justify-center">
            <svg class="h-48 w-48 -rotate-90" viewBox="0 0 120 120" aria-label="Jauge Mémoire">
                <circle
                    cx="60"
                    cy="60"
                    r="54"
                    class="stroke-slate-800"
                    stroke-width="10"
                    fill="none"
                />
                <circle
                    cx="60"
                    cy="60"
                    r="54"
                    :stroke="ringColor"
                    stroke-width="10"
                    stroke-linecap="round"
                    fill="none"
                    :stroke-dasharray="circumference"
                    :stroke-dashoffset="strokeDashoffset"
                    class="transition-[stroke-dashoffset,stroke] duration-500 ease-out"
                />
            </svg>

            <div class="absolute inset-0 flex flex-col items-center justify-center text-center">
                <span :class="['text-5xl font-bold tabular-nums', colorClass]">{{ clampedUsage }}%</span>
                <span class="mt-2 text-sm text-slate-400">Utilisée</span>
            </div>
            </div>
            
            <div class="min-w-0 flex-1">
            
                    <SubMemoryComponent />
            </div>
        </div>
        <div> 
            <div class="flex justify-between items-center gap-4 mt-6"> 
        <h1 class="mb-1">Swap</h1>
        <p class="border w-full bg-slate-200/10 rounded-lg"> </p>
        <p class="mb-1"> 0% </p>
        </div>
          <Progress :model-value="progress" class="w-[100%]" />
        <p>0 o / 2.00 Go</p>
</div>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { onMounted, ref } from 'vue'
import { Progress } from '@/components/ui/progress'

import SubMemoryComponent from  '@/components/Memory/SubMemoryComponent.vue'


const progress = ref(13)
onMounted(() => {
  const timer = setTimeout(() => {
    progress.value = 66
  }, 500)
  return () => clearTimeout(timer)
})


const props = withDefaults(defineProps<{
    usage?: number
}>(), {
    usage: 0,
})

const clampedUsage = computed(() => Math.min(100, Math.max(0, props.usage)))

const colorClass = computed(() => {
    if (clampedUsage.value >= 80) return 'text-red-500'
    if (clampedUsage.value >= 70) return 'text-orange-400'
    return 'text-sky-500'
})

const ringColor = computed(() => {
    if (clampedUsage.value >= 80) return '#ef4444'
    if (clampedUsage.value >= 70) return '#fb923c'
    return '#38bdf8'
})

const radius = 54
const circumference = 2 * Math.PI * radius

const strokeDashoffset = computed(() => {
    return circumference - (clampedUsage.value / 100) * circumference
})
</script>
