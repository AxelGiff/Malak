<template>
  <div class="flex h-screen w-full items-center justify-center bg-background text-foreground dark">
    <div class="w-full h-full flex items-center justify-center bg-neutral-950">
      <Card class="w-[350px] bg-neutral-900 border-neutral-800">
        <CardHeader class="flex flex-row items-center gap-2">
          <View class="h-8 w-8 text-purple-500" />
          <CardTitle class="text-2xl font-semibold text-white">ARGOS</CardTitle>
        </CardHeader>
        <CardContent>
          <form @submit.prevent>
            <div class="grid w-full items-center gap-4">
              <div class="flex flex-col space-y-1.5">
                <Label for="email" class="text-neutral-300">Email</Label>
                <Input id="email" v-model="email" placeholder="Email" class="bg-neutral-800 border-neutral-700 text-white" />
              </div>
              <div class="flex flex-col space-y-1.5">
                <Label for="password" class="text-neutral-300">Mot de passe</Label>
                <Input id="password" type="password" v-model="password" placeholder="Mot de passe" class="bg-neutral-800 border-neutral-700 text-white" />
              </div>
            </div>
          </form>
        </CardContent>
        <CardFooter class="flex justify-between">
          <Button @click="handleRegister" variant="outline" class="border-neutral-700 text-neutral-300 hover:bg-neutral-800">S'authentifier</Button>
          <Button @click="handleLogin" class="bg-purple-600 hover:bg-purple-700 text-white">Se connecter</Button>
        </CardFooter>
      </Card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Button } from '@/components/ui/button'
import { View } from '@lucide/vue'

const email = ref('')
const password = ref('')
const router = useRouter()

const handleRegister = async () => {
  try {
    const response = await fetch('http://localhost:8000/api/auth/register', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email: email.value, password: password.value })
    })
    if (response.ok) {
      alert('Inscription réussie')
      router.push('/monitor')
    } else {
      alert('Erreur lors de l\'inscription')
    }
  } catch (error) {
    console.error('Error:', error)
  }
}

const handleLogin = async () => {
  try {
    const response = await fetch('http://localhost:8000/api/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email: email.value, password: password.value })
    })
    if (response.ok) {
      alert('Connexion réussie')
      router.push('/monitor')
    } else {
      alert('Erreur lors de la connexion')
    }
  } catch (error) {
    console.error('Error:', error)
  }
}
</script>
