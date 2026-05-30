<template>
  <div class="flex h-fit flex-col rounded-2xl border border-white/10 bg-slate-950/70 p-3  font-sans shadow-xl shadow-slate-950/30 backdrop-blur">
    <div class="mb-2">
      <p class="text-lg font-semibold uppercase tracking-[0.08em] text-white">INFORMATIONS RAPIDES</p>
    </div>

    <div class="grid gap-0.5">
      <div
        v-for="item in systemRows"
        :key="item.label"
        class="flex items-center justify-between gap-3 border-b border-white/10 px-1 py-2 last:border-b-0"
      >
        <div class="flex min-w-0 items-center gap-3">
          <span class="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg border border-violet-500/20 bg-violet-500/10 text-violet-400">
            <svg viewBox="0 0 24 24" class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
              <path :d="item.icon" />
            </svg>
          </span>
          <span class="truncate text-sm text-slate-300">{{ item.label }}</span>
        </div>
        <span class="text-right text-sm font-medium text-slate-100">{{ item.value }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
type SystemRow = {
  label: string
  value: string
  icon: string
}

const globalData = {
  nom_machine: 'DESKTOP-90OIQRQ',
  nombre_processeurs: 16,
  nombre_processus: 298,
  systeme_exploitation: 'Windows',
  uptime: 11748,
  version_systeme: '11 (26200)',
}

function formatUptime(totalSeconds: number) {
  const hours = Math.floor(totalSeconds / 3600)
  const minutes = Math.floor((totalSeconds % 3600) / 60)
  const seconds = totalSeconds % 60

  if (hours > 0) {
    return `${hours}h ${minutes}m ${seconds}s`
  }

  if (minutes > 0) {
    return `${minutes}m ${seconds}s`
  }

  return `${seconds}s`
}

const systemRows: SystemRow[] = [
  {
    label: "Nom de la machine",
    value: globalData.nom_machine,
    icon: 'M4 7h16M4 12h16M4 17h16',
  },
  {
    label: "Système d'exploitation",
    value: `${globalData.systeme_exploitation} ${globalData.version_systeme}`,
    icon: 'M4 7h16M4 12h16M4 17h16',
  },
  {
    label: 'Architecture',
    value: 'x64',
    icon: 'M9 3h6M9 21h6M3 9v6M21 9v6M6 6h12v12H6z',
  },
  {
    label: 'Nombre de processeurs logiques',
    value: String(globalData.nombre_processeurs),
    icon: 'M9 9h6v6H9zM3 3h6v6H3zM15 3h6v6h-6zM3 15h6v6H3zM15 15h6v6h-6z',
  },
  {
    label: 'Temps de démarrage',
    value: '11/05/2024 10:35:12',
    icon: 'M12 6v6l4 2M12 3a9 9 0 1 0 9 9',
  },
]
</script>