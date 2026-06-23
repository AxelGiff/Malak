<template>
  <div class="mx-auto flex w-full max-w-full flex-col rounded-2xl border border-white/10 bg-slate-950/70 p-5 font-sans shadow-xl shadow-slate-950/30 backdrop-blur">
    <div class="mb-4">
      <p class="text-base font-semibold tracking-[0.08em] text-white">DISQUES</p>
    </div>

    <Table class="text-white">
      <TableHeader>
        <TableRow class="border-white/10 hover:bg-transparent">
          <TableHead class="text-slate-400">Nom</TableHead>
          <TableHead class="text-slate-400">Système de fichiers</TableHead>
          <TableHead class="text-slate-400">Utilisation</TableHead>
          <TableHead class="text-slate-400">Total / Utilisé</TableHead>
          <TableHead class="text-slate-400">Libre</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow v-for="disk in formattedDisks" :key="disk.id" class="border-white/5 hover:bg-white/5">
          <TableCell class="font-medium text-slate-100">{{ disk.nom_disque || 'Disque C:' }}</TableCell>
          <TableCell class="text-slate-300">{{ disk.file_system }}</TableCell>
          <TableCell class="text-slate-300">
            <div class="flex items-center gap-2">
              <Progress :model-value="disk.pourcentage_utilisation" class="h-2 w-24" />
              <span>{{ disk.pourcentage_utilisation.toFixed(2) }}%</span>
            </div>
          </TableCell>
          <TableCell class="text-slate-300">{{ disk.taille_totale }} / {{ disk.espace_utilise }}</TableCell>
          <TableCell class="text-slate-300">{{ disk.espace_libre }}</TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>
</template>

<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { Progress } from '@/components/ui/progress'
import { computed } from 'vue'

const props = defineProps<{
  disks: Array<{
    id: string
    nom_disque: string
    file_system: string
    taille_totale: number
    espace_utilise: number
    espace_libre: number
    pourcentage_utilisation: number
  }>
}>()

const formatBytes = (bytes: number) => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

const formattedDisks = computed(() => props.disks.map(disk => ({
  ...disk,
  taille_totale: formatBytes(disk.taille_totale),
  espace_utilise: formatBytes(disk.espace_utilise),
  espace_libre: formatBytes(disk.espace_libre),
})))
</script>
