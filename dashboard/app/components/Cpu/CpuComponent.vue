
<template>
	<div class="flex h-full w-full flex-col rounded-2xl border border-white/10 bg-slate-950/70 p-6 font-sans shadow-xl shadow-slate-950/30 backdrop-blur">
		<div class="mb-4 flex items-start justify-between gap-4">
			<p class="mt-1 text-lg font-semibold uppercase tracking-[0.08em] text-white">UTILISATION CPU</p>
			<button class="inline-flex items-center gap-2 rounded-lg border border-white/10 bg-white/5 px-3 py-2 text-xs text-slate-300 transition-colors hover:bg-white/10">
				<svg viewBox="0 0 24 24" class="h-4 w-4 text-slate-300" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
					<path d="M4 19V5" />
					<path d="M4 15l5-5 4 4 7-7" />
					<path d="M20 7v4h-4" />
				</svg>
				<span>Voir en graphique</span>
			</button>
		</div>
		<div class="grid flex-1 gap-6 lg:grid-cols-[20rem_minmax(0,1fr)]">
			<div class="flex flex-col items-start gap-5">
				<div class="mx-auto flex w-full max-w-[20rem] flex-col items-center rounded-2xl border border-white/10 bg-white/5 p-5 text-center">
					<p class="mb-1 text-center text-sm text-slate-300">Utilisation totale</p>
					<div class="relative mx-auto -mt-2 w-full max-w-68">
						<svg class="block h-40 w-full" viewBox="0 0 120 70" aria-label="Jauge CPU">
							<path d="M20 60 A40 40 0 0 1 100 60" class="stroke-slate-800" stroke-width="10" fill="none" stroke-linecap="round" />
							<path
								d="M20 60 A40 40 0 0 1 100 60"
								:stroke="ringColor"
								stroke-width="10"
								fill="none"
								stroke-linecap="round"
								:stroke-dasharray="arcLength"
								:stroke-dashoffset="arcDashoffset"
								class="transition-[stroke-dashoffset,stroke] duration-500 ease-out"
							/>
						</svg>

						<div class="absolute inset-x-0 bottom-0 flex flex-col items-center justify-center pb-5 text-center">
							<span :class="['text-[2.05rem] font-semibold tabular-nums leading-none', colorClass]">{{ clampedUsage }}%</span>
							<span class="mt-1 text-sm text-slate-400">Moyenne sur 16 coeurs</span>
						</div>
					</div>
				</div>

				<div class="w-full rounded-2xl border border-white/10 bg-white/5 p-4 text-slate-200">
					<div class="flex items-center gap-3">
						<div class="flex h-10 w-10 items-center justify-center rounded-xl border border-white/10 bg-slate-900/80 text-violet-400">
							<svg viewBox="0 0 24 24" class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
								<rect x="6" y="6" width="12" height="12" rx="2" />
								<path d="M9 3v3M15 3v3M9 18v3M15 18v3M3 9h3M3 15h3M18 9h3M18 15h3" />
							</svg>
						</div>
						<div>
							<p class="text-sm font-medium text-slate-100">AMD Ryzen 7 3700X</p>
							<p class="text-sm text-slate-300">8-Core Processor</p>
						</div>
					</div>

					<div class="mt-4 border-t border-white/10 pt-3">
						<p class="text-xs text-slate-400">Fréquence</p>
						<p class="mt-1 text-sm font-medium text-slate-100">3600 MHz</p>
					</div>
				</div>
			</div>

			<div class="grid gap-3 lg:grid-cols-2 lg:gap-0">
				<div
					v-for="(column, index) in cpuColumns"
					:key="column.id"
					class="space-y-3 lg:px-4"
					:class="index === 0
						? 'lg:border-r lg:border-white/10 lg:pl-0 lg:pr-4'
						: 'lg:pl-4 lg:pr-0'"
				>
					<div
						v-for="core in column.cores"
						:key="core.id"
						class="flex items-center gap-4 text-sm"
					>
						<span class="w-12 text-right text-slate-200">
							{{ core.nom_cpu }}
						</span>

						<div class="flex min-w-0 flex-1 items-center gap-3">
							<Progress
								:model-value="core.utilisation"
								class="h-2 flex-1 bg-slate-800/80"
								:indicator-class="progressColor(core.usage)"

							/>

							<span class="w-16 text-right tabular-nums text-slate-300">
								{{ core.usage.toFixed(2) }}%
							</span>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import { Progress } from '@/components/ui/progress'

type Core = {
	name: string
	usage: number
	label: string
	indicatorClass: string
	valueClass: string
}

const props = withDefaults(defineProps<{
	usage?: number,
	cpuHearts?: Array<Record<string, unknown>>,
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

const radius = 40
const arcLength = Math.PI * radius

const arcDashoffset = computed(() => {
	return arcLength - (clampedUsage.value / 100) * arcLength
})


function progressColor(usage: number): string {
  if (usage >= 80) return 'bg-[#ef4444]'
  if (usage >= 70) return 'bg-[#fb923c]'
  return 'bg-[#38bdf8]'
}
const cpuColumns = computed(() => {
  const cpus = (props.cpuHearts ?? []) as any[]

  const enriched = cpus.map((cpu) => ({
    id: cpu.id,
    nom_cpu: cpu.nom_cpu,
    usage: Number(cpu.usage ?? cpu.utilisation ?? 0),
    utilisation: Number(cpu.utilisation ?? cpu.usage ?? 0),
  }))

  const mid = Math.ceil(enriched.length / 2)

  return [
    { id: 'left', cores: enriched.slice(0, mid) },
    { id: 'right', cores: enriched.slice(mid) },
  ]
})
</script>
