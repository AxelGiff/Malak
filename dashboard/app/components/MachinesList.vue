<template>
  <div class="mx-auto flex w-full max-w-full flex-col rounded-2xl border border-white/10 bg-slate-950/70 p-5 font-sans shadow-xl shadow-slate-950/30 backdrop-blur">
    <div class="mb-4">
      <p class="text-base font-semibold tracking-[0.08em] text-white">Liste des machines</p>
    </div>

    <Table class="text-white">
      <TableHeader>
        <TableRow class="border-white/10 hover:bg-transparent">
          <TableHead class="text-slate-400">Hostname</TableHead>
          <TableHead class="text-slate-400">OS</TableHead>
          <TableHead class="text-slate-400">Version</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow v-for="machine in machines" :key="machine.id" class="border-white/5 hover:bg-white/5">
          <TableCell class="font-medium text-slate-100">{{ machine.hostname }}</TableCell>
          <TableCell class="text-slate-300">{{ machine.os }}</TableCell>
          <TableCell class="text-slate-300">{{ machine.os_version }}</TableCell>
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

defineProps<{
  machines: Array<{
    id: string | number
    hostname: string
    os: string
    os_version: string
  }>
}>()
</script>
