<template>
  <div class="card">
    <div class="card-body">
      <div class="container-xl" v-if="rootNode">
        <div class="text-center mb-2 org-chart-root">
          <div class="d-inline-block">
            <div class="avatar avatar bg-primary text-white mb-2 small">
              {{ getAcronym(rootNode.area) }}
            </div>
            <div class="fw-medium">{{ formatAreaName(rootNode.area) }}</div>
            <div class="text-muted small">{{ getAreaType(rootNode.area) }}</div>
          </div>
        </div>

        <div class="row g-4 justify-content-center org-chart-row" v-if="childNodes.length > 0">
          <div v-for="area in childNodes" :key="area.id" class="col-xl-3 col-lg-4 col-md-6 org-chart-node">
            <div class="card card-sm h-100" :class="{ 'card-active': activeNodeId === area.id }">
              <div class="card-body d-flex flex-column">
                <div class="d-flex align-items-start justify-content-between mb-3">
                  <div class="d-flex align-items-center">
                    <div class="avatar avatar-sm me-2 bg-primary-lt">
                      <component :is="getAreaIcon(area.area)" class="icon icon-sm" />
                    </div>
                    <div class="flex-fill">
                      <div class="fw-bold small">
                        {{ formatAreaName(area.area) }}
                      </div>
                      <div class="text-muted small">
                        {{ getAreaType(area.area) }}
                      </div>
                    </div>
                  </div>
                  <button v-if="area.subgerencias && area.subgerencias.length > 0" class="btn btn-sm btn-ghost-secondary p-1" @click="toggleExpand(area.id)">
                    <IconChevronDown class="icon icon-sm" :class="{ 'rotate-180': activeNodeId === area.id }" />
                  </button>
                </div>

                <div class="mb-3 flex-grow-1">
                  <div v-if="area.jefe" class="d-flex align-items-center">
                    <IconUser class="icon icon-sm me-1 text-muted" />
                    <span class="fs-5 fw-medium text-secondary">{{ formatName(area.jefe) }}</span>
                  </div>
                  <div v-else class="d-flex align-items-center text-muted">
                    <IconUserOff class="icon icon-sm me-1" />
                    <span class="small">Sin asignar</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="org-chart-expansion" v-if="activeSubgerencias && activeSubgerencias.length > 0">
          <div class="small text-muted mb-2">Subáreas ({{ activeSubgerencias.length }})</div>
          <div class="sub-area-container">
            <div class="d-flex flex-nowrap gap-2 pb-1">
              <div v-for="sub in activeSubgerencias" :key="sub.id" class="sub-area-card">
                <div class="d-flex align-items-center p-2 bg-light rounded">
                  <div class="avatar avatar-xs bg-gray-lt text-gray me-2">
                    <IconUsers class="icon icon-xs" />
                  </div>
                  <div class="flex-fill">
                    <div class="fw-medium small">
                      {{ formatAreaName(sub.area) }}
                    </div>
                    <div class="text-dark-emphasis small">
                      {{ sub.jefe ? formatName(sub.jefe) : 'Sin asignar' }}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="text-center text-muted py-5">Cargando datos del organigrama...</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import {
  IconUser,
  IconUserOff,
  IconUsers,
  IconChevronDown,
  IconScale,
  IconShield,
  IconBook,
  IconHeart,
  IconCoin,
  IconTool,
  IconFileText,
  IconSettings,
  IconBuilding
} from '@tabler/icons-vue'

import { api } from '@api/axios'

const activeNodeId = ref<number | null>(null)
const orgData = ref<any[]>([])

const rootNode = computed(() => orgData.value[0] || null)
const childNodes = computed(() => rootNode.value?.subgerencias || [])

const activeSubgerencias = computed(() => {
  if (!activeNodeId.value) return null
  const activeNode = childNodes.value.find((node: any) => node.id === activeNodeId.value)
  return activeNode ? activeNode.subgerencias : null
})

onMounted(async () => {
  try {
    orgData.value = await (await api.post('/dash/organigrama')).data
  } catch (error) {
    console.error('Error al cargar organigrama:', error)
  }
})

const toggleExpand = (nodeId: number) => {
  if (activeNodeId.value === nodeId) {
    activeNodeId.value = null
  } else {
    activeNodeId.value = nodeId
  }
}

const formatAreaName = (area: string) => {
  if (!area) return 'Área sin nombre'
  return area
    .replace(/GERENCIA DE |SUBGERENCIA DE |OFICINA DE |UNIDAD DE /gi, '')
    .toLowerCase()
    .split(' ')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

const getAcronym = (area: string) => {
  if (!area) return '?'
  return area
    .replace(/GERENCIA DE |SUBGERENCIA DE |OFICINA DE |UNIDAD DE /gi, '')
    .split(' ')
    .map((word) => word.charAt(0))
    .join('')
    .toUpperCase()
}

const formatName = (name: string) => {
  if (!name) return ''
  return name
    .toLowerCase()
    .split(' ')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

const getAreaIcon = (area: string) => {
  if (area.includes('JURIDICA') || area.includes('PROCURADURIA')) return IconScale
  if (area.includes('SEGURIDAD') || area.includes('SERENAZGO')) return IconShield
  if (area.includes('EDUCACION') || area.includes('CULTURA')) return IconBook
  if (area.includes('SALUD') || area.includes('SOCIAL')) return IconHeart
  if (area.includes('RENTAS') || area.includes('TRIBUTARIA')) return IconCoin
  if (area.includes('OBRAS') || area.includes('DESARROLLO')) return IconTool
  if (area.includes('SECRETARIA') || area.includes('DOCUMENTARIA')) return IconFileText
  if (area.includes('ADMINISTRACION') || area.includes('RECURSOS HUMANOS')) return IconSettings
  return IconBuilding
}

const getAreaType = (area: string) => {
  if (area.includes('GERENCIA') && !area.includes('SUBGERENCIA')) return 'Gerencia'
  if (area.includes('SUBGERENCIA')) return 'Subgerencia'
  if (area.includes('OFICINA')) return 'Oficina'
  if (area.includes('UNIDAD')) return 'Unidad'
  if (area.includes('ORGANO')) return 'Órgano'
  return 'Área'
}
</script>

<style lang="scss" scoped>
.rotate-180 {
  transform: rotate(180deg);
  transition: transform 0.2s ease;
}

.card-sm {
  transition: all 0.2s ease;
  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  }
}

$border-color: var(--tblr-border-color, #dee2e6);
$line-space: 1.5rem; // 24px
$breakpoint-md: 768px;

.org-chart-root {
  position: relative;

  @media (min-width: $breakpoint-md) {
    margin-bottom: $line-space;
    &::after {
      content: '';
      position: absolute;
      bottom: -$line-space;
      left: 50%;
      transform: translateX(-50%);
      width: 2px;
      height: $line-space;
      background-color: $border-color;
    }
  }
}

.org-chart-row {
  position: relative;

  @media (min-width: $breakpoint-md) {
    padding-top: $line-space;
    &::before {
      content: '';
      position: absolute;
      top: $line-space;
      left: 10%;
      right: 10%;
      height: 2px;
      background-color: $border-color;
    }
  }
}

.org-chart-node {
  position: relative;

  @media (min-width: $breakpoint-md) {
    &::before {
      content: '';
      position: absolute;
      top: 0;
      left: 50%;
      transform: translateX(-50%);
      width: 2px;
      height: $line-space;
      background-color: $border-color;
    }
  }
}

.card-active {
  border-color: var(--tblr-primary) !important;
  box-shadow: 0 4px 12px rgba(var(--tblr-primary-rgb), 0.15);
}

.org-chart-expansion {
  position: relative;

  @media (min-width: $breakpoint-md) {
    padding-top: $line-space;
    margin-top: $line-space;

    &::before {
      content: '';
      position: absolute;
      top: -$line-space; // Sube hasta el 'gap'
      left: 50%;
      transform: translateX(-50%);
      width: 2px;
      height: $line-space;
      background-color: $border-color;
    }

    &::after {
      content: '';
      position: absolute;
      top: 0;
      left: 10%;
      right: 10%;
      height: 2px;
      background-color: $border-color;
    }
  }

  @media (max-width: ($breakpoint-md - 1px)) {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid $border-color;
  }
}

.sub-area-container {
  overflow-x: auto;
  margin: 0 -0.5rem;
  padding: 0.5rem;

  &::-webkit-scrollbar {
    height: 6px;
  }
  &::-webkit-scrollbar-thumb {
    background: $border-color;
    border-radius: 3px;
  }
  &::-webkit-scrollbar-track {
    background: transparent;
  }
}

.sub-area-card {
  width: 260px;
  flex-shrink: 1;
}
</style>
