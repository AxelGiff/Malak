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
      <header class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12">
        <div class="flex items-center gap-2 px-4">
          <SidebarTrigger class="-ml-1" />
        </div>
        <div class="ml-auto flex items-center gap-2 px-4">
          <ThemeToggle />
        </div>
      </header>
      <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
        <div class="grid auto-rows-min gap-4 md:grid-cols-3">
          <div class="aspect-video rounded-xl bg-muted/50" ><CpuComponent :usage="53" /></div>
          <div class="aspect-video rounded-xl bg-muted/50" ><MemoryComponent :usage="68" /> </div>
           <div class="aspect-video rounded-xl bg-muted/50" > <NetworkComponent /> </div>
        </div>
        <div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min" />
      </div>
    </SidebarInset>
  </SidebarProvider>
</template>
