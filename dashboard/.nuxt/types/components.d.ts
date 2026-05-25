
import type { DefineComponent, SlotsType } from 'vue'
type IslandComponent<T> = DefineComponent<{}, {refresh: () => Promise<void>}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, SlotsType<{ fallback: { error: unknown } }>> & T

type HydrationStrategies = {
  hydrateOnVisible?: IntersectionObserverInit | true
  hydrateOnIdle?: number | true
  hydrateOnInteraction?: keyof HTMLElementEventMap | Array<keyof HTMLElementEventMap> | true
  hydrateOnMediaQuery?: string
  hydrateAfter?: number
  hydrateWhen?: boolean
  hydrateNever?: true
}
type LazyComponent<T> = DefineComponent<HydrationStrategies, {}, {}, {}, {}, {}, {}, { hydrated: () => void }> & T

interface _GlobalComponents {
  CpuComponent: typeof import("../../app/components/Cpu/CpuComponent.vue")['default']
  CpuSubCpuComponent: typeof import("../../app/components/Cpu/SubCpuComponent.vue")['default']
  DiskComponent: typeof import("../../app/components/DiskComponent.vue")['default']
  MemoryComponent: typeof import("../../app/components/MemoryComponent.vue")['default']
  Sidebar: typeof import("../../app/components/Sidebar.vue")['default']
  ThemeToggle: typeof import("../../app/components/ThemeToggle.vue")['default']
  NuxtWelcome: typeof import("../../node_modules/nuxt/dist/app/components/welcome.vue")['default']
  NuxtLayout: typeof import("../../node_modules/nuxt/dist/app/components/nuxt-layout")['default']
  NuxtErrorBoundary: typeof import("../../node_modules/nuxt/dist/app/components/nuxt-error-boundary.vue")['default']
  ClientOnly: typeof import("../../node_modules/nuxt/dist/app/components/client-only")['default']
  DevOnly: typeof import("../../node_modules/nuxt/dist/app/components/dev-only")['default']
  ServerPlaceholder: typeof import("../../node_modules/nuxt/dist/app/components/server-placeholder")['default']
  NuxtLink: typeof import("../../node_modules/nuxt/dist/app/components/nuxt-link")['default']
  NuxtLoadingIndicator: typeof import("../../node_modules/nuxt/dist/app/components/nuxt-loading-indicator")['default']
  NuxtTime: typeof import("../../node_modules/nuxt/dist/app/components/nuxt-time.vue")['default']
  NuxtRouteAnnouncer: typeof import("../../node_modules/nuxt/dist/app/components/nuxt-route-announcer")['default']
  NuxtAnnouncer: typeof import("../../node_modules/nuxt/dist/app/components/nuxt-announcer")['default']
  NuxtImg: typeof import("../../node_modules/nuxt/dist/app/components/nuxt-stubs")['NuxtImg']
  NuxtPicture: typeof import("../../node_modules/nuxt/dist/app/components/nuxt-stubs")['NuxtPicture']
  AlertDialog: typeof import("../../app/components/ui/alert-dialog/index")['AlertDialog']
  AlertDialogAction: typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogAction']
  AlertDialogCancel: typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogCancel']
  AlertDialogContent: typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogContent']
  AlertDialogDescription: typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogDescription']
  AlertDialogFooter: typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogFooter']
  AlertDialogHeader: typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogHeader']
  AlertDialogTitle: typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogTitle']
  AlertDialogTrigger: typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogTrigger']
  Button: typeof import("../../app/components/ui/button/index")['Button']
  NuxtPage: typeof import("../../node_modules/nuxt/dist/pages/runtime/page")['default']
  NoScript: typeof import("../../node_modules/nuxt/dist/head/runtime/components")['NoScript']
  Link: typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Link']
  Base: typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Base']
  Title: typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Title']
  Meta: typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Meta']
  Style: typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Style']
  Head: typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Head']
  Html: typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Html']
  Body: typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Body']
  NuxtIsland: typeof import("../../node_modules/nuxt/dist/app/components/nuxt-island")['default']
  LazyCpuComponent: LazyComponent<typeof import("../../app/components/Cpu/CpuComponent.vue")['default']>
  LazyCpuSubCpuComponent: LazyComponent<typeof import("../../app/components/Cpu/SubCpuComponent.vue")['default']>
  LazyDiskComponent: LazyComponent<typeof import("../../app/components/DiskComponent.vue")['default']>
  LazyMemoryComponent: LazyComponent<typeof import("../../app/components/MemoryComponent.vue")['default']>
  LazySidebar: LazyComponent<typeof import("../../app/components/Sidebar.vue")['default']>
  LazyThemeToggle: LazyComponent<typeof import("../../app/components/ThemeToggle.vue")['default']>
  LazyNuxtWelcome: LazyComponent<typeof import("../../node_modules/nuxt/dist/app/components/welcome.vue")['default']>
  LazyNuxtLayout: LazyComponent<typeof import("../../node_modules/nuxt/dist/app/components/nuxt-layout")['default']>
  LazyNuxtErrorBoundary: LazyComponent<typeof import("../../node_modules/nuxt/dist/app/components/nuxt-error-boundary.vue")['default']>
  LazyClientOnly: LazyComponent<typeof import("../../node_modules/nuxt/dist/app/components/client-only")['default']>
  LazyDevOnly: LazyComponent<typeof import("../../node_modules/nuxt/dist/app/components/dev-only")['default']>
  LazyServerPlaceholder: LazyComponent<typeof import("../../node_modules/nuxt/dist/app/components/server-placeholder")['default']>
  LazyNuxtLink: LazyComponent<typeof import("../../node_modules/nuxt/dist/app/components/nuxt-link")['default']>
  LazyNuxtLoadingIndicator: LazyComponent<typeof import("../../node_modules/nuxt/dist/app/components/nuxt-loading-indicator")['default']>
  LazyNuxtTime: LazyComponent<typeof import("../../node_modules/nuxt/dist/app/components/nuxt-time.vue")['default']>
  LazyNuxtRouteAnnouncer: LazyComponent<typeof import("../../node_modules/nuxt/dist/app/components/nuxt-route-announcer")['default']>
  LazyNuxtAnnouncer: LazyComponent<typeof import("../../node_modules/nuxt/dist/app/components/nuxt-announcer")['default']>
  LazyNuxtImg: LazyComponent<typeof import("../../node_modules/nuxt/dist/app/components/nuxt-stubs")['NuxtImg']>
  LazyNuxtPicture: LazyComponent<typeof import("../../node_modules/nuxt/dist/app/components/nuxt-stubs")['NuxtPicture']>
  LazyAlertDialog: LazyComponent<typeof import("../../app/components/ui/alert-dialog/index")['AlertDialog']>
  LazyAlertDialogAction: LazyComponent<typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogAction']>
  LazyAlertDialogCancel: LazyComponent<typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogCancel']>
  LazyAlertDialogContent: LazyComponent<typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogContent']>
  LazyAlertDialogDescription: LazyComponent<typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogDescription']>
  LazyAlertDialogFooter: LazyComponent<typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogFooter']>
  LazyAlertDialogHeader: LazyComponent<typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogHeader']>
  LazyAlertDialogTitle: LazyComponent<typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogTitle']>
  LazyAlertDialogTrigger: LazyComponent<typeof import("../../app/components/ui/alert-dialog/index")['AlertDialogTrigger']>
  LazyButton: LazyComponent<typeof import("../../app/components/ui/button/index")['Button']>
  LazyNuxtPage: LazyComponent<typeof import("../../node_modules/nuxt/dist/pages/runtime/page")['default']>
  LazyNoScript: LazyComponent<typeof import("../../node_modules/nuxt/dist/head/runtime/components")['NoScript']>
  LazyLink: LazyComponent<typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Link']>
  LazyBase: LazyComponent<typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Base']>
  LazyTitle: LazyComponent<typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Title']>
  LazyMeta: LazyComponent<typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Meta']>
  LazyStyle: LazyComponent<typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Style']>
  LazyHead: LazyComponent<typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Head']>
  LazyHtml: LazyComponent<typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Html']>
  LazyBody: LazyComponent<typeof import("../../node_modules/nuxt/dist/head/runtime/components")['Body']>
  LazyNuxtIsland: LazyComponent<typeof import("../../node_modules/nuxt/dist/app/components/nuxt-island")['default']>
}

declare module 'vue' {
  export interface GlobalComponents extends _GlobalComponents { }
}

export {}
