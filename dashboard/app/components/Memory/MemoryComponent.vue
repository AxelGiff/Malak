
<template>
    <div class="rounded-2xl border border-white/10 bg-slate-950/70 px-4 pt-4 pb-10 font-sans shadow-xl shadow-slate-950/30 backdrop-blur">
        <div class="mb-3">
            <p class="mt-1 text-base font-semibold tracking-[0.08em] text-white">MÉMOIRE</p>
        </div>

        <div class="flex items-center gap-5">
            <div class="relative flex shrink-0 items-center justify-center">
            <svg class="h-36 w-36 -rotate-90" viewBox="0 0 120 120" aria-label="Jauge Mémoire">
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
                <span :class="['text-[1.8rem] font-bold tabular-nums leading-none', colorClass]">{{ clampedUsage }}%</span>
                <span class="mt-1 text-[0.72rem] tracking-wide text-slate-400">Utilisée</span>
            </div>
            </div>
            
            <div class="min-w-0 flex-1">
            
                    <SubMemoryComponent  :data="props.data" />
            </div>
        </div>
                <div class="mt-7">
                        <div class="mt-4 flex items-center gap-3 text-sm text-slate-300"> 
                <h1 class="mb-0.5 text-xs font-medium text-slate-200">Swap</h1>
        <p class="h-px w-full bg-slate-200/10 rounded-lg"> </p>
                <p class="mb-0.5 text-xs text-slate-200">{{props.data.swap_usage_percent.toFixed(2)}}%</p>
        </div>
          <Progress :model-value="progress" class="w-full" />
        <p class="mt-1 text-xs text-slate-400">{{props.data.used_swap.toFixed(2)}} / {{props.data.total_swap.toFixed(2)}} Go</p>
</div>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { onMounted, ref } from 'vue'
import { Progress } from '@/components/ui/progress'

import SubMemoryComponent from  '@/components/Memory/SubMemoryComponent.vue'

type MemoryData = {
    total: number
    used: number
    free: number
    used_swap:number
    total_swap:number
    swap_usage_percent:number
    available: number
}


const progress = ref(13)
onMounted(() => {
  const timer = setTimeout(() => {
    progress.value = props.data.swap_usage_percent
  }, 500)
  return () => clearTimeout(timer)
})


const props = withDefaults(defineProps<{
    usage?: number,
    data: MemoryData,
}>(), {
    usage: 0,
})

const clampedUsage = computed(() => Math.min(100, Math.max(0, props.usage)))
console.log(props.data);
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
