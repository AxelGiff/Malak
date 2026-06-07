
<template>
	<div class="grid grid-cols-1 gap-4">
		<div class="flex flex-col gap-4">
			<h3 class="flex w-full items-center justify-between text-sm font-semibold text-white">
				<span class="flex items-center gap-2">
					<span class="h-2.5 w-2.5 rounded-full bg-blue-400"></span>
					<span>Utilisée</span>
				</span>
				<span class="tabular-nums text-slate-100">{{ data?.used.toFixed(2) }} Go</span>
			</h3>
		</div>
		<div class="flex flex-col gap-4">
			<h3 class="flex w-full items-center justify-between text-sm font-semibold text-white">
				<span class="flex items-center gap-2">
					<span class="h-2.5 w-2.5 rounded-full bg-green-400"></span>
					<span>Libre</span>
				</span>
				<span class="tabular-nums text-slate-100">{{ data?.free.toFixed(2) }} Go</span>
			</h3>
		</div>
		<div class="flex flex-col gap-4">
			<h3 class="flex w-full items-center justify-between text-sm font-semibold text-white">
				<span class="flex items-center gap-2">
					<span class="h-2.5 w-2.5 rounded-full bg-gray-400"></span>
					<span>Total</span>
				</span>
				<span class="tabular-nums text-slate-100">{{ data?.total.toFixed(2) }} Go</span>
			</h3>
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

type MemoryData = {
    total: number
    used: number
    free: number
    available: number
}

const props = withDefaults(defineProps<{
    data: MemoryData,
}>(), {
})
</script>
