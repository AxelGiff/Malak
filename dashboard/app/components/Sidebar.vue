<template>
  <SidebarProvider>
    <Sidebar class="text-lg">
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" >
                    <View  color="purple"  />
             <div class="grid flex-1 text-left text-2xl leading-tight">
                <span class="truncate font-semibold">ARGOS</span>
                <span class="truncate text-xs">Monitoring</span>
              </div>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Vue d'ensemble</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton as-child>
                  <a href="/monitor">
                    <LayoutDashboard /> <span> Dashboard</span>
                  </a>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
        <hr class="w-3/4 mx-4 border border-gray-200" />
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton as-child>
                  <a href="/monitor/machines">
                     <Monitor class="text-white" /> <span class="text-white"> Machines</span>
                  </a>
                </SidebarMenuButton>
              </SidebarMenuItem>
             
            
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        <div class="flex items-center gap-2 p-2">
          <div class="h-8 w-8 rounded-full bg-gray-500 flex items-center justify-center text-white">U</div>
          <div class="flex flex-col">
            <span class="text-sm font-medium">Utilisateur</span>
            <span class="text-xs text-gray-500">user@example.com</span>
          </div>
        </div>
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
    <SidebarInset>
      <header class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12">
         
        <div class="flex items-center gap-2 px-4">
          <SidebarTrigger class="-ml-1" />
        </div>
         <div >
          <h1 class="text-2xl font-semibold ">Tableau de bord</h1>
        </div>
        <div class=" flex items-center gap-2 px-4">
<Select v-model="selectedMachine" @update:model-value="onMachineChange">
    <SelectTrigger>
      <SelectValue placeholder="Sélectionner une machine" />
    </SelectTrigger>
    <SelectContent>
      <SelectItem v-for="machine in machines" :key="machine.id" :value="machine.hostname">
        {{ machine.hostname }}
      </SelectItem>
    </SelectContent>
  </Select>
</div>
<div class="flex  gap-2">
  <span :class="['h-3 w-3 rounded-full transition-colors duration-300', statusDotClass]"></span>
  <p class="text-sm font-medium text-emerald-400">{{ statusLabel }}</p>
</div>

        <div class="ml-auto flex items-center gap-2 px-4">
           <Button class="cursor-pointer" variant="outline" @click="refresh" size="sm" :disabled="disabledBtn">
            <RefreshCw />
            Actualiser
          </Button>
          <ThemeToggle />
        </div>
      </header>
      <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
        <div class="grid flex-1 gap-4 xl:grid-cols-[minmax(0,1.15fr)_minmax(0,0.85fr)]">
          <div class="flex flex-col gap-4">
            <GlobalDataComponent  v-if="dataMachines" class="w-full" :data="dataMachines" />
            <CpuComponent :usage="cpuUsage" :cpuHearts="dataCpus" class="h-full w-full" />
          </div>
          <div class="flex flex-col gap-4">
            <MemoryComponent :usage="memoryUsage" class="w-full" v-if="dataMemories" :data="dataMemories" />
<DiskComponent v-if="dataDisks" class="w-full" :data="dataDisks" />
<NetworkComponent v-if="dataNetworks" class="w-full" :data="dataNetworks" />

          </div>
        </div>
      </div>
    </SidebarInset>
  </SidebarProvider>
</template>

<script setup lang="ts">
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarRail,
  SidebarTrigger,
} from '@/components/ui/sidebar'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { RefreshCw } from '@lucide/vue'
import { Button } from '@/components/ui/button'
import ThemeToggle from '@/components/ThemeToggle.vue'
import { Home, LayoutDashboard, Monitor, View } from '@lucide/vue'
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
  import { useMonitor } from '@/composables/useMonitor'

const isOnline = ref(true)
const syncOnlineStatus = () => {
  if (typeof navigator !== 'undefined') {
    isOnline.value = navigator.onLine
  }
}
const disabledBtn=ref(false);
const machines = ref([]);
const selectedMachine = ref('');
const { data, loading, error, fetchLatest, fetchMachines, dataMachines, dataNetworks, dataDisks, dataMemories, dataCpus } = useMonitor()

onMounted(async () => {
  syncOnlineStatus()
  window.addEventListener('online', syncOnlineStatus)
  window.addEventListener('offline', syncOnlineStatus)

  machines.value = await fetchMachines();
  if (machines.value.length > 0) {
      selectedMachine.value = machines.value[0].hostname;
      await fetchLatest(selectedMachine.value);
  }
})

const onMachineChange = async (hostname) => {
    await fetchLatest(hostname);
}
onBeforeUnmount(() => {
  window.removeEventListener('online', syncOnlineStatus)
  window.removeEventListener('offline', syncOnlineStatus)
})

const statusLabel = computed(() => (isOnline.value ? 'En ligne' : 'Hors ligne'))

const statusDotClass = computed(() =>
  isOnline.value
    ? 'bg-emerald-400 shadow-[0_0_10px_rgba(52,211,153,0.75)]'
    : 'bg-red-500 shadow-[0_0_10px_rgba(239,68,68,0.75)]',
)

const cpuHearts = computed(() => {
  const cpus = (dataCpus.value ?? []) as Array<Record<string, unknown>>

  const enriched = cpus.map((cpu) => {
    const usage = Number(cpu?.usage)

    return {
      id: cpu.id,
      nom_cpu: cpu.nom_cpu,
      usage: Number.isFinite(usage)
        ? Number(Math.min(100, Math.max(0, usage)).toFixed(2))
        : 0,
      utilisation: cpu.utilisation,
      total_usage: cpu.total_usage,
    }
  })

  const mid = Math.ceil(enriched.length / 2)

  return [
    enriched.slice(0, mid),
    enriched.slice(mid),
  ]
})

const cpuUsage = computed(() => {
  const cpus = (dataCpus.value ?? []) as Array<Record<string, unknown>>

  if (cpus.length === 0) {
    return 0
  }

 

  const average = cpus.reduce((sum, cpu) => sum + Number(cpu?.usage || 0), 0) / cpus.length
  return Number(average.toFixed(2));
})

const memoryUsage = computed(() => {
  const memory = (dataMemories.value ?? null) as Record<string, unknown> | null
  const value = Number(memory?.usage_percent)
  return Number.isFinite(value) ? Math.min(100, Math.max(0, value)).toFixed(2) : 0
})

const refresh = () => {
  fetchLatest(selectedMachine.value)
  disabledBtn.value=true;
  setTimeout(() => {
   disabledBtn.value=false;
  }, 5000)
}

</script>