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
import ThemeToggle from '@/components/ThemeToggle.vue'
import { Home, LayoutDashboard, Monitor, View } from '@lucide/vue'
</script>

<template>
  <SidebarProvider>
    <Sidebar class="text-lg">
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" >
                    <View color="orange"  />
             <div class="grid flex-1 text-left text-2xl leading-tight">
                <span class="truncate font-semibold text-amber-400">Argus</span>
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
                  
                  <a href="#">
              <LayoutDashboard />    <span> Dashboard</span>
                  </a>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
        <hr class="w-3/4 mx-4 border border-gray-200" />
        <SidebarGroup>
          <SidebarGroupLabel>Machines</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton as-child>
                  <a href="#">
                    <Home />
                   <Monitor /> <span>DESKTOP-UID</span>
                  </a>
                </SidebarMenuButton>
                 <SidebarMenuButton as-child>
                  <a href="#">
                    <Home />
                   <Monitor /> <span>DESKTOP-d</span>
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
          <SidebarTrigger class="-ml-1" />
        </div>
        <div class="ml-auto flex items-center gap-2 px-4">
          <ThemeToggle />
        </div>
      </header>
      <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
        <div class="grid flex-1 gap-4 xl:grid-cols-[minmax(0,1.15fr)_minmax(0,0.85fr)]">
          <div class="flex flex-col gap-4">
            <GlobalDataComponent class="w-full" />
            <CpuComponent :usage="53" class="h-full w-full" />
          </div>
          <div class="flex flex-col gap-4">
            <MemoryComponent :usage="68" class="w-full" />
            <DiskComponent class="w-full" />
            <NetworkComponent class="w-full" />
          </div>
        </div>
      </div>
    </SidebarInset>
  </SidebarProvider>
</template>
