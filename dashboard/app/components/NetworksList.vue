<template>
  <div class="mx-auto flex w-full max-w-full flex-col rounded-2xl border border-white/10 bg-slate-950/70 p-5 font-sans shadow-xl shadow-slate-950/30 backdrop-blur">
    <div class="mb-4">
      <p class="text-base font-semibold tracking-[0.08em] text-white">RÉSEAUX</p>
    </div>

    <Table class="text-white">
      <TableHeader>
        <TableRow class="border-white/10 hover:bg-transparent">
          <TableHead class="text-slate-400">Interface</TableHead>
          <TableHead class="text-slate-400">Reçu</TableHead>
          <TableHead class="text-slate-400">Transmis</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow v-for="network in formattedNetworks" :key="network.id" class="border-white/5 hover:bg-white/5">
          <TableCell class="font-medium text-slate-100">{{ network.interface }}</TableCell>
          <TableCell class="text-slate-300">{{ network.received }}</TableCell>
          <TableCell class="text-slate-300">{{ network.transmitted }}</TableCell>
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
import { computed } from 'vue'

const props = defineProps<{
  networks: Array<{
    id: string
    interface: string
    received: number
    transmitted: number
  }>
}>()

const formatBytes = (bytes: number) => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

const formattedNetworks = computed(() => props.networks.map(net => ({
  ...net,
  received: formatBytes(net.received),
  transmitted: formatBytes(net.transmitted),
})))
</script>
