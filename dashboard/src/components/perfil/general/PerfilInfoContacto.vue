<script setup lang="ts">
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
  IconHome,
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
  <div class="space-y-4 text-micro">
    <section class="space-y-3">
      <div class="flex items-center justify-between">
        <h3 class="font-semibold text-foreground text-meta">Acerca del servidor</h3>
        <button type="button" class="text-primary hover:underline font-medium inline-flex items-center gap-1 cursor-pointer"
          @click="emit('openEditModal')">
          <IconEdit class="size-3.5" /> Editar
        </button>
      </div>

      <ul class="space-y-2">
        <li class="flex items-center gap-3" title="DNI">
          <IconId class="size-4 text-muted-foreground shrink-0" />
          <span class="font-mono text-foreground">{{ perfil?.dni || '-' }}</span>
          <button type="button" class="text-muted-foreground hover:text-primary transition cursor-pointer"
            title="Copiar número de DNI al portapapeles" aria-label="Copiar número de DNI al portapapeles"
            @click="emit('copyToClipboard', perfil?.dni || '', 'dni-info')">
            <IconCheck v-if="copiedField === 'dni-info'" class="size-3.5 text-emerald-600 dark:text-emerald-400" />
            <IconCopy v-else class="size-3.5" />
          </button>
        </li>
        <li class="flex items-center gap-3" title="RUC">
          <IconShieldCheck class="size-4 text-muted-foreground shrink-0" />
          <span class="font-mono text-foreground">{{ perfil?.ruc || 'RUC no registrado' }}</span>
        </li>
        <li class="flex items-center gap-3" title="Fecha de nacimiento">
          <IconCalendar class="size-4 text-muted-foreground shrink-0" />
          <span class="text-foreground">
            {{ formatDate(perfil?.nacimiento) }}
            <span v-if="calcularEdad(perfil?.nacimiento)" class="text-muted-foreground">({{ calcularEdad(perfil?.nacimiento) }} años)</span>
          </span>
        </li>
        <li class="flex items-center gap-3" title="Sexo">
          <IconUser class="size-4 text-muted-foreground shrink-0" />
          <span class="text-foreground">{{ perfil?.sexo === 'F' ? 'Femenino' : perfil?.sexo === 'M' ? 'Masculino' : '-' }}</span>
        </li>
        <li class="flex items-center gap-3" title="Teléfono">
          <IconPhone class="size-4 text-muted-foreground shrink-0" />
          <a v-if="perfil?.telf" :href="'tel:' + perfil.telf" class="text-primary hover:underline">{{ perfil.telf }}</a>
          <button v-else type="button" class="text-primary hover:underline cursor-pointer" @click="emit('openEditModal')">Registrar teléfono</button>
        </li>
        <li class="flex items-center gap-3 min-w-0" title="Correo electrónico">
          <IconMail class="size-4 text-muted-foreground shrink-0" />
          <a v-if="perfil?.email" :href="'mailto:' + perfil.email" class="text-primary hover:underline truncate">{{ perfil.email }}</a>
          <button v-else type="button" class="text-primary hover:underline cursor-pointer" @click="emit('openEditModal')">Registrar correo</button>
        </li>
        <li class="flex items-center gap-3" title="Distrito">
          <IconMapPin class="size-4 text-muted-foreground shrink-0" />
          <span class="text-foreground">
            <router-link v-if="perfil?.distrito" :to="{ path: '/personal/distrito', query: { distrito: perfil.distrito } }"
              class="text-primary hover:underline" title="Ver trabajadores del distrito">{{ perfil.distrito }}</router-link>
            <template v-else>-</template>
            <span class="text-muted-foreground"> ({{ perfil?.region || 'LIMA' }})</span>
          </span>
        </li>
        <li class="flex items-start gap-3" title="Dirección">
          <IconHome class="size-4 text-muted-foreground shrink-0 mt-0.5" />
          <span class="text-foreground break-words">{{ perfil?.direccion || 'Sin dirección domiciliaria registrada' }}</span>
        </li>
      </ul>
    </section>

    <div class="border-t border-border"></div>

    <section class="space-y-3">
      <div class="flex items-center justify-between">
        <h3 class="font-semibold text-foreground text-meta">Contacto de emergencia</h3>
        <Badge v-if="contacto" variant="outline" size="sm">Registrado</Badge>
      </div>

      <ul v-if="contacto" class="space-y-2.5">
        <li class="flex items-center gap-3" title="Nombre">
          <IconHeartHandshake class="size-4 text-rose-500 shrink-0" />
          <span class="font-semibold text-foreground">{{ contacto.nombre }}</span>
        </li>
        <li class="flex items-center gap-3" title="Parentesco">
          <IconUser class="size-4 text-muted-foreground shrink-0" />
          <span class="px-2 py-0.5 rounded bg-muted text-foreground font-medium">{{ contacto.relacion }}</span>
        </li>
        <li class="flex items-center gap-3" title="Teléfono">
          <IconPhone class="size-4 text-muted-foreground shrink-0" />
          <a :href="'tel:' + contacto.telefono" class="font-semibold text-primary hover:underline">{{ contacto.telefono }}</a>
        </li>
      </ul>
      <p v-else class="text-muted-foreground">No se encontró contacto de emergencia registrado para este servidor.</p>
    </section>
  </div>
</template>
