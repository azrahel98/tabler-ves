<template>
  <div class="relative w-full" ref="containerRef">
    
    <div
      class="h-11 w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm text-gray-800 focus-within:border-brand-300 focus-within:ring-3 focus-within:ring-brand-500/10 dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 flex items-center justify-between cursor-pointer transition-colors"
      :class="{ 'ring-3 ring-brand-500/10 border-brand-300 dark:border-brand-700 dark:ring-brand-500/20': isOpen }"
      @click="toggleDropdown"
    >
      <span v-if="selectedOption" class="truncate">{{ selectedOption.nombre }}</span>
      <span v-else class="text-gray-500 dark:text-gray-400 truncate">{{ placeholder }}</span>
      
      <ChevronDown
        class="h-4 w-4 text-gray-400 transition-transform duration-200 shrink-0"
        :class="{ 'rotate-180': isOpen }"
      />
    </div>

    
    <Transition
      enter-active-class="transition duration-100 ease-out"
      enter-from-class="transform scale-95 opacity-0"
      enter-to-class="transform scale-100 opacity-100"
      leave-active-class="transition duration-75 ease-in"
      leave-from-class="transform scale-100 opacity-100"
      leave-to-class="transform scale-95 opacity-0"
    >
      <div
        v-if="isOpen"
        class="absolute z-50 mt-1.5 w-full rounded-lg border border-gray-200 bg-white py-1 shadow-xl dark:border-gray-700 dark:bg-gray-800"
      >
        <div class="px-2 py-2 border-b border-gray-100 dark:border-gray-700/50">
          <div class="relative">
            <Search class="absolute left-2.5 top-2.5 h-4 w-4 text-gray-400" />
            <input
              v-model="searchQuery"
              ref="searchInput"
              type="text"
              class="w-full rounded-md border border-gray-300 bg-gray-50 pl-9 pr-3 py-1.5 text-sm focus:border-brand-300 focus:ring-1 focus:ring-brand-500 focus:outline-hidden dark:border-gray-600 dark:bg-gray-900/50 dark:text-white/90 dark:focus:border-brand-500"
              :placeholder="`Buscar ${placeholder.toLowerCase()}...`"
              @click.stop
            />
          </div>
        </div>

        <ul class="max-h-60 overflow-auto py-1 text-sm text-gray-700 dark:text-gray-200 scrollbar-thin scrollbar-thumb-gray-300 dark:scrollbar-thumb-gray-600">
          <li
            v-if="filteredOptions.length === 0"
            class="px-3 py-4 text-center text-sm text-gray-500 dark:text-gray-400"
          >
            No se encontraron resultados
          </li>
          <li
            v-for="option in filteredOptions"
            :key="option.id"
            class="relative cursor-pointer select-none py-2.5 pl-3 pr-9 transition-colors hover:bg-gray-50 dark:hover:bg-gray-700/50"
            :class="{ 'bg-brand-50 text-brand-700 dark:bg-brand-500/10 dark:text-brand-300 font-medium': option.id === modelValue }"
            @click.stop="selectOption(option)"
          >
            <span class="block truncate">{{ option.nombre }}</span>
            <span
              v-if="option.id === modelValue"
              class="absolute inset-y-0 right-0 flex items-center pr-3 text-brand-600 dark:text-brand-400"
            >
              <Check class="h-4 w-4" />
            </span>
          </li>
        </ul>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'
import { onClickOutside } from '@vueuse/core'
import { ChevronDown, Search, Check } from 'lucide-vue-next'

const props = withDefaults(defineProps<{
  modelValue: number | string | null
  options: { id: number | string; nombre: string }[]
  placeholder?: string
}>(), {
  placeholder: 'Seleccionar'
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: number | string): void
  (e: 'change', value: number | string): void
}>()

const containerRef = ref<HTMLElement | null>(null)
const searchInput = ref<HTMLInputElement | null>(null)

const isOpen = ref(false)
const searchQuery = ref('')

const selectedOption = computed(() => {
  return props.options.find(opt => opt.id === props.modelValue) || null
})

const filteredOptions = computed(() => {
  if (!searchQuery.value) return props.options
  const query = searchQuery.value.toLowerCase()
  
  const normalize = (str: string) => str.normalize("NFD").replace(/[\u0300-\u036f]/g, "")
  const normalizedQuery = normalize(query)
  
  return props.options.filter(opt => 
    normalize(opt.nombre.toLowerCase()).includes(normalizedQuery)
  )
})

const toggleDropdown = () => {
  isOpen.value = !isOpen.value
  if (isOpen.value) {
    searchQuery.value = ''
    nextTick(() => {
      searchInput.value?.focus()
    })
  }
}

const selectOption = (option: { id: number | string; nombre: string }) => {
  emit('update:modelValue', option.id)
  emit('change', option.id)
  isOpen.value = false
}

onClickOutside(containerRef, () => {
  if (isOpen.value) isOpen.value = false
})

</script>
