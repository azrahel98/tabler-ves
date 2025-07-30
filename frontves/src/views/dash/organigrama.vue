<template>
  <div class="card">
    <div class="card-body">
      <div class="container-xl">
        <div class="text-center mb-2">
          <div class="d-inline-block">
            <div class="avatar avatar bg-primary text-white mb-2 small">GM</div>
            <div class="fw-medium">Gerencia Municipal</div>
            <div class="text-muted small">Nivel Superior</div>
          </div>
        </div>

        <div class="d-flex justify-content-center mb-5">
          <div style="width: 2px; height: 40px" class="bg-secondary-subtle" />
        </div>

        <div class="row g-4 justify-content-center">
          <div v-for="area in orgData[0]?.subgerencias" :key="area.id" class="col-xl-3 col-lg-4 col-md-6">
            <div class="card card-sm h-100">
              <div class="card-body">
                <div class="d-flex align-items-start justify-content-between mb-3">
                  <div class="d-flex align-items-center">
                    <div class="avatar avatar-sm me-2 bg-primary-lt">
                      <component :is="getAreaIcon(area.area)" class="icon icon-sm" />
                    </div>
                    <div class="flex-fill">
                      <div class="fw-bold small">{{ formatAreaName(area.area) }}</div>
                      <div class="text-muted small">{{ getAreaType(area.area) }}</div>
                    </div>
                  </div>
                  <button v-if="area.subgerencias && area.subgerencias.length > 0" class="btn btn-sm btn-ghost-secondary p-1" @click="toggleExpand(area.id)">
                    <IconChevronDown class="icon icon-sm" :class="{ 'rotate-180': expandedNodes.includes(area.id) }" />
                  </button>
                </div>

                <div class="mb-3">
                  <div v-if="area.jefe" class="d-flex align-items-center">
                    <IconUser class="icon icon-sm me-1" />
                    <span class="fs-5 fw-medium text-secondary">{{ formatName(area.jefe) }}</span>
                  </div>
                  <div v-else class="d-flex align-items-center text-muted">
                    <IconUserOff class="icon icon-sm me-1" />
                    <span class="small">Sin asignar</span>
                  </div>
                </div>

                <div v-if="area.subgerencias && area.subgerencias.length > 0 && expandedNodes.includes(area.id)" class="mt-3 pt-3 border-top">
                  <div class="small text-muted mb-2">Subáreas ({{ area.subgerencias.length }})</div>
                  <div class="d-flex flex-column gap-2">
                    <div v-for="sub in area.subgerencias" :key="sub.id" class="d-flex align-items-center p-2 bg-light rounded">
                      <div class="avatar avatar-xs bg-gray-lt text-gray me-2">
                        <IconUsers class="icon icon-xs" />
                      </div>
                      <div class="flex-fill">
                        <div class="fw-medium small">{{ formatAreaName(sub.area) }}</div>
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
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
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

const expandedNodes = ref<number[]>([])

const orgData = ref<any>([])

onMounted(async () => {
  try {
    orgData.value = await (await api.post('/dash/organigrama')).data
  } catch (error) {
    console.log(error)
  }
})

// const oficinasIndependientes = computed(() => orgData.value.slice(1))

const toggleExpand = (nodeId: number) => {
  const index = expandedNodes.value.indexOf(nodeId)
  if (index > -1) {
    expandedNodes.value.splice(index, 1)
  } else {
    expandedNodes.value.push(nodeId)
  }
}

const formatAreaName = (area: string) => {
  return area
    .replace(/GERENCIA DE |SUBGERENCIA DE |OFICINA DE |UNIDAD DE |SUB GERENCIA DE /g, '')
    .toLowerCase()
    .split(' ')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
}

const formatName = (name: string) => {
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

<style scoped>
.rotate-180 {
  transform: rotate(180deg);
  transition: transform 0.2s ease;
}

.card-sm {
  transition: all 0.2s ease;
}

.card-sm:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.badge-xs {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  padding: 0;
}
</style>
