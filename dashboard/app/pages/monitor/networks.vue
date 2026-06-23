<template>
  <SidebarProvider>
    <Sidebar class="text-lg">
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" >
              <View class="h-8 w-8" color="purple"  />
              <div class="grid flex-1 text-left text-2xl leading-tight">
                <span class="truncate font-semibold text-white">ARGOS</span>
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
                  <a href="/monitor/networks">
                     <Monitor class="text-white" /> <span class="text-white"> Réseaux</span>
                  </a>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter />
      <SidebarRail />
    </Sidebar>
    <SidebarInset>
      <header class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12">
        <div class="flex items-center gap-2 px-4">
          <SidebarTrigger class="-ml-1 text-white" />
        </div>
         <div class="text-white">
          <h1 class="text-2xl font-semibold ">Réseaux</h1>
        </div>
        <div class="ml-auto flex items-center gap-2 px-4">
           <Button class="cursor-pointer text-white" variant="outline" @click="refresh" size="sm" :disabled="disabledBtn">
            <RefreshCw />
            Actualiser
          </Button>
        </div>
      </header>
      <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
        <NetworksList :networks="dataNetworks" />
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
import NetworksList from '@/components/NetworksList.vue'
import { RefreshCw, LayoutDashboard, Monitor, View } from '@lucide/vue'
import { Button } from '@/components/ui/button'
import { onMounted, ref } from 'vue'

const { fetchLatest, fetchMachines, dataNetworks } = useMonitor()
const disabledBtn=ref(false);

onMounted(async () => {
  const machines = await fetchMachines();
  if (machines.length > 0) {
      await fetchLatest(machines[0].hostname);
  }
})

const refresh = async () => {
disabledBtn.value=true;

  await fetchLatest();
  setTimeout(() => {
   disabledBtn.value=false;
  }, 5000)}

</script>
