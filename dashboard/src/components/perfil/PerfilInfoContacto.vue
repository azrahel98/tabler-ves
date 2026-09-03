<script setup lang="ts">
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import type { PersonalPerfil, PersonalContacto } from '@/services/personal'
import { formatDate, calculateAge } from '@/utils/date'
import {
  IconUser,
  IconId,
  IconMail,
  IconPhone,
  IconMapPin,
  IconCalendar,
  IconShieldCheck,
  IconHeartHandshake,
  IconEdit,
  IconCheck,
  IconCopy,
} from '@tabler/icons-vue'

interface Props {
  perfil: PersonalPerfil | null
  contacto: PersonalContacto | null
  copiedField: string | null
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'openEditModal'): void
  (e: 'copyToClipboard', text: string, fieldId: string): void
}>()

const calcularEdad = calculateAge
</script>

<template>
  <div class="space-y-6">
    <Card class="space-y-3">
      <div class="flex items-center justify-between border-b border-border pb-3">
        <span class="text-sm font-bold text-foreground tracking-wider flex items-center gap-2">
          <IconUser class="size-3.5 text-primary" />
          <h3 class="font-semibold text-foreground tracking-tight text-sm">Información de Contacto</h3>
        </span>
        <button
          type="button"
          class="text-xs text-primary hover:underline font-medium inline-flex items-center gap-1 cursor-pointer"
          @click="emit('openEditModal')"
        >
          <IconEdit class="size-3.5" /> Editar
        </button>
      </div>

      <div class="space-y-3.5 text-xs">
        <div class="flex items-start justify-between gap-3">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconId class="size-4 text-muted-foreground" /> DNI:
          </span>
          <div class="flex items-center gap-1.5 font-mono font-medium text-foreground">
            <span>{{ perfil?.dni || '-' }}</span>
            <button
              type="button"
              class="text-muted-foreground hover:text-primary transition cursor-pointer"
              title="Copiar número de DNI al portapapeles"
              aria-label="Copiar número de DNI al portapapeles"
              @click="emit('copyToClipboard', perfil?.dni || '', 'dni-info')"
            >
              <IconCheck v-if="copiedField === 'dni-info'" class="size-3.5 text-emerald-600 dark:text-emerald-400" />
              <IconCopy v-else class="size-3.5" />
            </button>
          </div>
        </div>

        <div class="flex items-start justify-between gap-3">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconShieldCheck class="size-4 text-muted-foreground" /> RUC:
          </span>
          <span class="font-mono font-medium text-foreground">
            {{ perfil?.ruc || 'No registrado' }}
          </span>
        </div>

        <div class="flex items-start justify-between gap-3">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconCalendar class="size-4 text-muted-foreground" /> Nacimiento:
          </span>
          <span class="font-medium text-foreground text-right">
            {{ formatDate(perfil?.nacimiento) }}
            <span v-if="calcularEdad(perfil?.nacimiento)" class="text-muted-foreground">
              ({{ calcularEdad(perfil?.nacimiento) }} años)
            </span>
          </span>
        </div>

        <div class="flex items-start justify-between gap-3">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconUser class="size-4 text-muted-foreground" /> Sexo:
          </span>
          <span class="font-medium text-foreground">
            {{ perfil?.sexo === 'F' ? 'Femenino' : perfil?.sexo === 'M' ? 'Masculino' : '-' }}
          </span>
        </div>

        <div class="border-t border-border/60 my-2 pt-2"></div>

        <div class="flex items-start justify-between gap-3">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconPhone class="size-4 text-muted-foreground" /> Teléfono:
          </span>
          <a v-if="perfil?.telf" :href="'tel:' + perfil.telf" class="font-medium text-primary hover:underline">
            {{ perfil.telf }}
          </a>
          <button v-else type="button" class="text-primary hover:underline text-xs" @click="emit('openEditModal')">
            Registrar teléfono
          </button>
        </div>

        <div class="flex items-start justify-between gap-3">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconMail class="size-4 text-muted-foreground" /> Correo:
          </span>
          <a
            v-if="perfil?.email"
            :href="'mailto:' + perfil.email"
            class="font-medium text-primary hover:underline truncate max-w-[180px]"
            :title="perfil.email"
          >
            {{ perfil.email }}
          </a>
          <button v-else type="button" class="text-primary hover:underline text-xs" @click="emit('openEditModal')">
            Registrar correo
          </button>
        </div>

        <div class="flex items-start justify-between gap-3">
          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
            <IconMapPin class="size-4 text-muted-foreground" /> Distrito:
          </span>
          <span class="font-medium text-[11.4px] text-foreground text-right">
            {{ perfil?.distrito || '-' }} ({{ perfil?.region || 'LIMA' }})
          </span>
        </div>

        <div class="flex flex-col gap-1">
          <span class="text-muted-foreground flex items-center gap-1.5">
            <IconMapPin class="size-4 text-muted-foreground" /> Dirección:
          </span>
          <span class="font-medium text-foreground bg-muted/40 p-2 rounded-lg border border-border/50 text-[11.4px]">
            {{ perfil?.direccion || 'Sin dirección domiciliaria registrada' }}
          </span>
        </div>
      </div>
    </Card>

    <Card class="p-5 space-y-4">
      <div class="border-b border-border pb-3 flex items-center justify-between">
        <h2 class="font-semibold text-foreground tracking-tight text-sm flex gap-2">
          <IconHeartHandshake class="size-4 text-rose-500" />
          <span>Contacto de Emergencia</span>
        </h2>
        <Badge v-if="contacto" variant="outline" size="sm">Registrado</Badge>
      </div>

      <div v-if="contacto" class="space-y-3 text-xs">
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">Nombre:</span>
          <span class="font-semibold text-foreground">{{ contacto.nombre }}</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">Parentesco:</span>
          <span class="px-2 py-0.5 rounded bg-muted text-foreground font-medium">{{ contacto.relacion }}</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">Teléfono:</span>
          <a
            :href="'tel:' + contacto.telefono"
            class="font-semibold text-primary hover:underline flex items-center gap-1"
          >
            <IconPhone class="size-3.5" /> {{ contacto.telefono }}
          </a>
        </div>
      </div>
      <div v-else class="text-xs text-muted-foreground py-2 text-center">
        No se encontró contacto de emergencia registrado para este servidor.
      </div>
    </Card>
  </div>
</template>
