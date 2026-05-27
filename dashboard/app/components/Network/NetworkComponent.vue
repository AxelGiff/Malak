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
    <div class="rounded-2xl border border-white/10 bg-slate-950/70 p-6 shadow-xl shadow-slate-950/30 backdrop-blur">
        <div class="mb-6">
			<p class="mt-1 text-2xl font-semibold text-white">RÉSEAUX</p>
        </div>

    <div class="rounded-md ">
      <Table>
        <TableHeader>
          <TableRow v-for="headerGroup in table.getHeaderGroups()" :key="headerGroup.id">
            <TableHead v-for="header in headerGroup.headers" :key="header.id" class="text-center">
              <FlexRender v-if="!header.isPlaceholder" :render="header.column.columnDef.header" :props="header.getContext()" />
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <template v-if="table.getRowModel().rows?.length">
            <template v-for="row in table.getRowModel().rows" :key="row.id">
              <TableRow :data-state="row.getIsSelected() && 'selected'">
                <TableCell v-for="cell in row.getVisibleCells()" :key="cell.id" class="text-center">
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
              class="h-24 text-center"
            >
              Pas d'interfaces trouvées.
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>

    <div class="flex items-center space-x-2 mt-13 justify-end">
      <Button class="w-full flex cursor-pointer justify-between" variant="secondary">
    Voir toutes les interfaces
    <MoveRight /> 
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
  recu: number
  unit: string
  transmis: number
}

const data: Network[] = [
  {
    id: 'm5gr84i9',
    interface: 'eth0',
    recu: 1024,
    unit: "Go",
    transmis: 2048,
  },
  {
    id: '3u1reuv4',
    interface: 'eth1',
    recu: 2048,
    unit: "Go",
    transmis: 4096,
  },
  {
    id: 'derv1ws0',
    interface: 'wlan0',
    recu: 512,
    unit: "Go",
    transmis: 1024,
  },
  {
    id: '5kma53ae',
    interface: 'lo',
    recu: 0,
    unit: "Go",
    transmis: 0,
  },
]

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
      h('span', { class: 'truncate' }, `${row.getValue('recu')} ${row.original.unit}`),
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
      h('span', { class: 'truncate' }, `${row.getValue('transmis')} ${row.original.unit}`),
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
  data,
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