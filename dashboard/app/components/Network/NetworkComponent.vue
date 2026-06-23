<template>
  <DefineTemplate v-slot="{ payment }">
    <DropdownMenu>
      <DropdownMenuTrigger as-child>
        <Button variant="ghost" class="h-8 w-8 p-0">
          <span class="sr-only">Open menu</span>
          <MoreHorizontal class="h-4 w-4" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuLabel>Actions</DropdownMenuLabel>
        <DropdownMenuItem @click="copy(payment.id)">
          Copy payment ID
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem>View customer</DropdownMenuItem>
        <DropdownMenuItem>View payment details</DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  </DefineTemplate>
    <div class="rounded-2xl border border-white/10 bg-slate-950/70 p-4 font-sans shadow-xl shadow-slate-950/30 backdrop-blur">
        <div class="mb-3">
      <p class="mt-1 text-base font-semibold tracking-[0.08em] text-white">RÉSEAUX</p>
        </div>

    <div class="rounded-md ">
      <Table>
        <TableHeader>
          <TableRow v-for="headerGroup in table.getHeaderGroups()" :key="headerGroup.id">
            <TableHead v-for="header in headerGroup.headers" :key="header.id" class="text-center text-[0.7rem] uppercase tracking-[0.08em] text-slate-400">
              <FlexRender v-if="!header.isPlaceholder" :render="header.column.columnDef.header" :props="header.getContext()" />
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <template v-if="table.getRowModel().rows?.length">
            <template v-for="row in table.getRowModel().rows" :key="row.id">
              <TableRow :data-state="row.getIsSelected() && 'selected'">
                <TableCell v-for="cell in row.getVisibleCells()" :key="cell.id" class="text-center text-xs text-slate-100">
                  <FlexRender :render="cell.column.columnDef.cell" :props="cell.getContext()" />
                </TableCell>
              </TableRow>
              <TableRow v-if="row.getIsExpanded()">
                <TableCell :colspan="row.getAllCells().length" class="text-center">
                  {{ JSON.stringify(row.original) }}
                </TableCell>
              </TableRow>
            </template>
          </template>

          <TableRow v-else>
            <TableCell
              :colspan="columns.length"
              class="h-20 text-center text-xs text-slate-400"
            >
              Pas d'interfaces trouvées.
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>

    <div class="mt-auto flex items-end pt-3">
      <Button class="p-0 w-full cursor-pointer" variant="secondary" as-child>
        <a href="/monitor/networks" class="flex w-full items-center justify-start gap-2 rounded-xl bg-slate-900/80 px-4 py-2 text-sm font-medium text-slate-100 hover:bg-slate-900">
          <span>Voir toutes les interfaces</span>
                    <MoveRight class="h-4 w-4" />

        </a>
      </Button>
    </div>
  </div>
</template>


<script setup lang="ts">
import type {
  ColumnDef,
  ColumnFiltersState,
  ExpandedState,
  SortingState,
  VisibilityState,
} from '@tanstack/vue-table'
import { ArrowUpDown, MoveRight, ChevronDown, CircleArrowDown, CircleArrowUp, MoreHorizontal } from '@lucide/vue'
import {
  FlexRender,
  getCoreRowModel,
  getExpandedRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useVueTable,
} from '@tanstack/vue-table'
import { createReusableTemplate } from '@vueuse/core'
import { h, ref } from 'vue'

import { valueUpdater } from '@/components/ui/table/utils'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Input } from '@/components/ui/input'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

export interface Network {
  id: string
  interface: string
  recu: string
  transmis: string
}

type BackendNetwork = {
  received: number
  interface: string
  transmitted: number
}

const props = defineProps<{
  data: BackendNetwork[]
}>()

const formatBytes = (bytes: number) => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

const networkData = computed<Network[]>(() => {
  return props.data.map((net, index) => ({
    id: `net-${index}`,
    interface: net.interface,
    recu: formatBytes(net.received),
    transmis: formatBytes(net.transmitted),
  }))
})

const [DefineTemplate, ReuseTemplate] = createReusableTemplate<{
  payment: {
    id: string
  }
  onExpand: () => void
}>()

const columns: ColumnDef<Network>[] = [
  {
    accessorKey: 'interface',
     header: ({ column }) => {
      return h(Button, {
        variant: 'ghost',
        class: 'w-full justify-center',
        onClick: () => column.toggleSorting(column.getIsSorted() === 'asc'),
      }, () => ['Interface', h(ArrowUpDown, { class: 'ml-2 h-4 w-4' })])
    },
    cell: ({ row }) => h('div', { class: 'grid w-full grid-cols-[1rem_minmax(0,1fr)] items-center justify-center gap-2 text-center capitalize' }, [
      h('span', { class: 'mx-auto h-3 w-3 rounded-full bg-emerald-400 shadow-[0_0_10px_rgba(52,211,153,0.75)]' }),
      h('span', { class: 'truncate' }, row.getValue('interface')),
    ]),
  },
  {     

    accessorKey: 'recu',
    header: ({ column }) => {
      return h(Button, {
        variant: 'ghost',
        class: 'w-full justify-center',
        onClick: () => column.toggleSorting(column.getIsSorted() === 'asc'),
      }, () => ['Reçu', h(ArrowUpDown, { class: 'ml-2 h-4 w-4' })])
    },
    cell: ({ row }) => h('div', { class: 'grid w-full grid-cols-[minmax(0,1fr)_1.25rem] items-center justify-center gap-2 text-center' }, [
      h('span', { class: 'truncate' }, row.getValue('recu')),
      h(CircleArrowDown, {
        size: 20,
        color: '#10c9ea',
        absoluteStrokeWidth: true,
      }),
    ]),
  },
  {
    accessorKey: 'transmis',
    header: ({ column }) => {
      return h(Button, {
        variant: 'ghost',
        class: 'w-full justify-center',
        onClick: () => column.toggleSorting(column.getIsSorted() === 'asc'),
      }, () => ['Transmis', h(ArrowUpDown, { class: 'ml-2 h-4 w-4' })])
    },
    cell: ({ row }) => h('div', { class: 'grid w-full grid-cols-[minmax(0,1fr)_1.25rem] items-center justify-center gap-2 text-center' }, [
      h('span', { class: 'truncate' }, row.getValue('transmis')),
      h(CircleArrowUp, {
        size: 20,
        color: '#9534e5',
        absoluteStrokeWidth: true,
      }),
    ]),
  },
     
 
]

const sorting = ref<SortingState>([])
const columnFilters = ref<ColumnFiltersState>([])
const columnVisibility = ref<VisibilityState>({})
const expanded = ref<ExpandedState>({})

const table = useVueTable({
  get data() { return networkData.value },
  columns,
  getCoreRowModel: getCoreRowModel(),
  getPaginationRowModel: getPaginationRowModel(),
  getSortedRowModel: getSortedRowModel(),
  getFilteredRowModel: getFilteredRowModel(),
  getExpandedRowModel: getExpandedRowModel(),
  onSortingChange: updaterOrValue => valueUpdater(updaterOrValue, sorting),
  onColumnFiltersChange: updaterOrValue => valueUpdater(updaterOrValue, columnFilters),
  onColumnVisibilityChange: updaterOrValue => valueUpdater(updaterOrValue, columnVisibility),
  onExpandedChange: updaterOrValue => valueUpdater(updaterOrValue, expanded),
  state: {
    get sorting() { return sorting.value },
    get columnFilters() { return columnFilters.value },
    get columnVisibility() { return columnVisibility.value },
    get expanded() { return expanded.value },
  },
})

function copy(id: string) {
  navigator.clipboard.writeText(id)
}
</script>