// composables/useMonitor.js
import { ref } from 'vue'

export function useMonitor() {
  const data = ref(null)
  const loading = ref(false)
  const error = ref(null)
  const dataMachines = ref(null)
  const dataNetworks = ref([])
  const dataDisks = ref([])
  const dataMemories = ref(null)
  const dataCpus = ref([])

  const toNumber = (value, fallback = 0) => {
    const num = Number(value)
    return Number.isFinite(num) ? num : fallback
  }

  const clampPercent = value => Math.min(100, Math.max(0, toNumber(value, 0)))

  const toArray = value => (Array.isArray(value) ? value : [])

  const parseLatestMetrics = payload => {
    const latest = Array.isArray(payload) && payload.length > 0 ? payload[0] : null

    if (!latest) {
      dataMachines.value = null
      dataNetworks.value = []
      dataDisks.value = []
      dataMemories.value = null
      dataCpus.value = []
      return
    }

    dataMachines.value = {
      id: latest.machine?.id ?? null,
      hostname: latest.machine?.hostname ?? '',
      os: latest.machine?.os ?? '',
      os_version: latest.machine?.os_version ?? '',
      machine_created_at: latest.machine?.created_at ?? null,
      metrics_id: latest.metrics_id ?? null,
      metrics_created_at: latest.created_at ?? null,
      uptime: toNumber(latest.uptime, 0),
      nombre_processus: toNumber(latest.nombre_processus, 0),
      nombre_processeurs: toNumber(latest.nombre_processeurs, 0),
    }

    dataNetworks.value = toArray(latest.networks).map((network, index) => ({
      id: String(index),
      interface: network?.interface ?? `iface-${index + 1}`,
      received: toNumber(network?.received, 0),
      transmitted: toNumber(network?.transmitted, 0),
    }))

    dataDisks.value = toArray(latest.disks).map((disk, index) => {
      const total = toNumber(disk?.taille_totale, 0)
      const free = toNumber(disk?.espace_libre, 0)
      const used = toNumber(disk?.espace_utilise, Math.max(total - free, 0))

      return {
        id: String(index),
        nom_disque: disk?.nom_disque ?? `disk-${index + 1}`,
        file_system: disk?.file_system ?? 'unknown',
        taille_totale: total,
        espace_libre: free,
        espace_utilise: used,
        pourcentage_utilisation: clampPercent(disk?.pourcentage_utilisation ?? (total > 0 ? (used / total) * 100 : 0)),
      }
    })

    const memoryUsed = toNumber(latest.memory_used, 0)
    const memoryTotal = toNumber(latest.memory_total, 0)
    const memoryFree = toNumber(latest.memory_free, 0)
    const swapTotal = toNumber(latest.total_swap, 0)
    const swapUsed = toNumber(latest.used_swap, 0)

    dataMemories.value = {
      used: memoryUsed,
      total: memoryTotal,
      free: memoryFree,
      used_swap: swapUsed,
      total_swap: swapTotal,
      usage_percent: clampPercent(memoryTotal > 0 ? (memoryUsed / memoryTotal) * 100 : 0),
      swap_usage_percent: clampPercent(swapTotal > 0 ? (swapUsed / swapTotal) * 100 : 0),
    }

    dataCpus.value = toArray(latest.cpus).map((cpu, index) => ({
      id: toNumber(cpu?.id, index),
      nom_cpu: cpu?.nom_cpu ?? `CPU ${index + 1}`,
      brand: cpu?.brand ?? 'Unknown',
      frequence: toNumber(cpu?.frequence, 0),
      usage: clampPercent(cpu?.usage),
      utilisation: clampPercent(cpu?.utilisation ?? cpu?.usage),
      total_usage: clampPercent(cpu?.total_usage),
    }))
  }

  const fetchLatest = async () => {
    loading.value = true
    error.value = null

    try {
      const response = await fetch('http://localhost:8000/metrics/latest')

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`)
      }

      data.value = await response.json()
      parseLatestMetrics(data.value)
      return data.value
    } catch (err) {
      error.value = err
      console.error(err)
      dataMachines.value = null
      dataNetworks.value = []
      dataDisks.value = []
      dataMemories.value = null
      dataCpus.value = []
    } finally {
      loading.value = false
    }
  }

return {
    data,
    loading,
    error,
    fetchLatest,
    dataMachines,
    dataNetworks,
    dataDisks,
    dataMemories,
    dataCpus
  }
}