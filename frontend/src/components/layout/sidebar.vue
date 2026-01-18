<template>
  <aside
    class="fixed inset-y-0 left-0 z-50 w-60 bg-white border-r border-gray-100 flex flex-col transition-transform duration-300 md:relative md:translate-x-0"
    :class="isOpen ? 'translate-x-0' : '-translate-x-full'"
  >
    <div class="p-6 border-b border-gray-100">
      <div class="flex items-center gap-3">
        <a href="/" class="flex items-center gap-2">
          <div class="w-8 h-8 rounded-lg bg-linear-to-br from-[#5347ce] to-[#887cfd] flex items-center justify-center">
            <Zap :size="18" class="text-white" />
          </div>
          <span class="text-xl font-bold text-gray-900">Munives</span>
        </a>
        <div class="ml-auto flex items-center gap-2">
          <kbd class="hidden md:inline-block px-2 py-1 text-xs text-gray-500 bg-gray-50 rounded border border-gray-200">⌘K</kbd>
          <button class="md:hidden p-1 text-gray-500 hover:text-primary hover:cursor-pointer rounded" @click="$emit('close')">
            <X :size="20" />
          </button>
        </div>
      </div>
    </div>

    <nav class="flex-1 p-4 overflow-y-auto">
      <div class="mb-6">
        <p class="px-3 mb-2 text-xs font-semibold text-gray-400 uppercase tracking-wider">Dashboard</p>
        <ul class="space-y-1">
          <RouterLink :to="{ name: 'dashboard' }" class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm transition-colors">
            <LayoutDashboard :size="18" />
            <span>Dashboard</span>
          </RouterLink>
          <RouterLink to="/nuevo" v-if="store.id == '1'" class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm transition-colors">
            <LucideUserPlus2 :size="18" />
            <span>Nuevo </span>
          </RouterLink>

          <RouterLink to="/chat" class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm transition-colors">
            <MessageSquare :size="18" />
            <span>Chat</span>
          </RouterLink>
          <RouterLink to="/organigrama" class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm transition-colors">
            <Building2 :size="18" />
            <span>Organigrama</span>
          </RouterLink>
        </ul>
      </div>
    </nav>
  </aside>
</template>

<script setup lang="ts">
import { userStore } from '@store/user'
import { LayoutDashboard, MessageSquare, Zap, X, LucideUserPlus2, Building2 } from 'lucide-vue-next'

const store = userStore()

defineProps({
  isOpen: {
    type: Boolean,
    default: false
  }
})

defineEmits(['close'])
</script>
