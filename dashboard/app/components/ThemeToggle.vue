<script setup lang="ts">
import { ref, onMounted } from 'vue'

const theme = ref<'light' | 'dark'>('light')

function applyTheme(t: 'light' | 'dark') {
  const el = document.documentElement
  if (t === 'dark') {
    el.classList.add('dark')
  } else {
    el.classList.remove('dark')
  }
}

onMounted(() => {
  // load from localStorage or system preference
  const saved = localStorage.getItem('theme') as 'light' | 'dark' | null
  if (saved) {
    theme.value = saved
  } else {
    const prefersDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches
    theme.value = prefersDark ? 'dark' : 'light'
  }
  applyTheme(theme.value)
})

function toggle() {
  theme.value = theme.value === 'dark' ? 'light' : 'dark'
  localStorage.setItem('theme', theme.value)
  applyTheme(theme.value)
}
</script>

<template>
  <button @click="toggle" class="p-2 rounded-md hover:bg-muted/20">
    <span v-if="theme === 'dark'">
      <!-- sun icon -->
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
        <path d="M10 3.5a.75.75 0 01.75-.75h0a.75.75 0 010 1.5H10.75A.75.75 0 0110 3.5zM10 16.25a.75.75 0 01.75-.75h0a.75.75 0 010 1.5H10.75a.75.75 0 01-.75-.75zM3.5 10a.75.75 0 01-.75-.75v0a.75.75 0 011.5 0V9.25A.75.75 0 013.5 10zM16.25 10a.75.75 0 01-.75-.75v0a.75.75 0 011.5 0V9.25a.75.75 0 01-.75.75zM5.636 5.636a.75.75 0 01-1.06-1.06l0 0a.75.75 0 011.06 1.06zM14.424 14.424a.75.75 0 01-1.06-1.06 0 011.06 1.06zM5.636 14.364a.75.75 0 01-1.06 1.06 0 011.06-1.06zM14.424 5.636a.75.75 0 01-1.06 1.06 0 011.06-1.06zM10 6.5a3.5 3.5 0 100 7 3.5 3.5 0 000-7z" />
      </svg>
    </span>
    <span v-else>
      <!-- moon icon -->
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
        <path d="M17.293 13.293A8 8 0 116.707 2.707a7 7 0 0010.586 10.586z" />
      </svg>
    </span>
  </button>
</template>
