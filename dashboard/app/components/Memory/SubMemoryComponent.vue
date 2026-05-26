
<template>
	<div class="grid grid-cols-1 gap-4">
		<div class="flex flex-col gap-4">
			<h3 class="flex w-full items-center justify-between text-lg font-semibold text-white">
				<span class="flex items-center gap-3">
					<span class="h-3 w-3 rounded-sm bg-blue-400"></span>
					<span>Utilisée</span>
				</span>
				<span>16.43 Go</span>
			</h3>
		</div>
		<div class="flex flex-col gap-4">
			<h3 class="flex w-full items-center justify-between text-lg font-semibold text-white">
				<span class="flex items-center gap-3">
					<span class="h-3 w-3 rounded-sm bg-green-400"></span>
					<span>Libre</span>
				</span>
				<span>15.57 Go</span>
			</h3>
		</div>
		<div class="flex flex-col gap-4">
			<h3 class="flex w-full items-center justify-between text-lg font-semibold text-white">
				<span class="flex items-center gap-3">
					<span class="h-3 w-3 rounded-sm bg-gray-400"></span>
					<span>Total</span>
				</span>
				<span>32.00 Go</span>
			</h3>
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
</script>
