<template>
  <div class="mx-auto flex w-full max-w-full flex-col rounded-2xl border border-white/10 bg-slate-950/70 p-3 font-sans shadow-xl shadow-slate-950/30 backdrop-blur">
     <div class="mb-3">
            <p class="mt-1 text-base font-semibold tracking-[0.08em] text-white">STOCKAGE</p>
        </div>

    <div class="grid grid-cols-[minmax(9rem,1.05fr)_minmax(7.5rem,0.8fr)_minmax(9.5rem,0.95fr)] gap-3 px-2 pb-1 text-[0.7rem] font-medium uppercase tracking-[0.12em] text-slate-400">
      <div>Disque</div>
      <div>Utilisation</div>
      <div>Espace utilisé / total</div>
    </div>

    <div class="space-y-1">
      <div
        v-for="disk in disks"
        :key="disk.id"
        class="grid grid-cols-[minmax(9rem,1.05fr)_minmax(7.5rem,0.8fr)_minmax(9.5rem,0.95fr)] items-center gap-3 rounded-xl px-2 py-2.5 transition-colors hover:bg-white/5"
      >
        <div class="flex min-w-0 items-start gap-3">
          <div class="mt-0.5 flex h-10 w-10 shrink-0 items-center justify-center rounded-xl border border-white/10 bg-slate-900/85 shadow-inner shadow-black/30">
            <div class="relative h-5 w-5 rounded-sm border border-slate-300/70 bg-slate-700/90">
              <div class="absolute inset-x-0 bottom-0 h-1 rounded-b-sm bg-slate-300/90"></div>
              <div class="absolute right-0 top-0 grid h-2.5 w-2.5 grid-cols-2 gap-px p-0.5">
                <span class="rounded-[1px] bg-sky-400"></span>
                <span class="rounded-[1px] bg-sky-400"></span>
                <span class="rounded-[1px] bg-sky-400"></span>
                <span class="rounded-[1px] bg-sky-400"></span>
              </div>
            </div>
          </div>

          <div class="min-w-0">
            <div class="truncate text-sm font-medium text-slate-100">{{ disk.name }}</div>
            <div class="text-xs uppercase tracking-[0.16em] text-slate-500">{{ disk.fileSystem }}</div>
          </div>
        </div>

        <div class="flex min-w-0 items-center gap-2 pr-1">
          <Progress
            :model-value="disk.utilisation"
            class="h-1.5 min-w-0 flex-1 bg-slate-800/80"
            :indicator-class="disk.progressClass"
          />
          <span class="min-w-14 text-right text-xs font-semibold tabular-nums" :class="disk.percentClass">
            {{ disk.percentLabel }}
          </span>
        </div>

        <div class="text-center text-xs leading-5 text-slate-200">
          <div class="font-medium">{{ disk.used }} / {{ disk.total }}</div>
          <div class="text-slate-400">{{ disk.free }}</div>
        </div>
      </div>
    </div>

    <div class="mt-auto pt-4">
      <Button class="flex w-full p-0 cursor-pointer" variant="secondary" as-child>
        <a href="/monitor/disks" class="flex w-full items-center justify-start gap-2 rounded-xl bg-slate-900/80 px-4 py-2 text-sm font-medium text-slate-100 hover:bg-slate-900">
          <span>Voir tous les disques</span>
                    <MoveRight class="h-4 w-4" />

        </a>
      </Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { MoveRight } from '@lucide/vue'

import { Button } from '@/components/ui/button'
import { Progress } from '@/components/ui/progress'

type BackendDisk = {
  nom_disque: string
  file_system: string
  espace_libre: number
  taille_totale: number
  espace_utilise: number
  pourcentage_utilisation: number
}

const props = defineProps<{
  data: BackendDisk[]
}>()

const formatBytes = (bytes: number) => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

const getProgressClass = (utilisation: number) => {
  if (utilisation >= 90) return 'bg-red-500';
  if (utilisation >= 70) return 'bg-orange-500';
  return 'bg-sky-500';
}

const getPercentClass = (utilisation: number) => {
  if (utilisation >= 90) return 'text-red-500';
  if (utilisation >= 70) return 'text-orange-400';
  return 'text-sky-500';
}

const disks = computed<DiskRow[]>(() => {
  return props.data.map((disk, index) => ({
    id: `disk-${index}`,
    name: disk.nom_disque || 'Disque local',
    fileSystem: disk.file_system,
    utilisation: disk.pourcentage_utilisation,
    percentLabel: `${disk.pourcentage_utilisation.toFixed(2)}%`,
    used: formatBytes(disk.espace_utilise),
    total: formatBytes(disk.taille_totale),
    free: `${formatBytes(disk.espace_libre)} libres`,
    progressClass: getProgressClass(disk.pourcentage_utilisation),
    percentClass: getPercentClass(disk.pourcentage_utilisation),
  }))
})
</script>