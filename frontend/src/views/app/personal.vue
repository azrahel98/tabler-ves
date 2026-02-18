<script setup lang="ts">
import { router } from '@router/router'
import { onMounted, watch } from 'vue'
import { useProfileStore } from '@store/perfil'
import Vinculos from '@comp/personal/vinculos.vue'
import Info from '@comp/personal/info.vue'
import Banco from '@comp/personal/banco.vue'
import Historial from '@comp/personal/historial.vue'

const perfil = useProfileStore()

const cargarPerfil = () => {
  const dni = router.currentRoute.value.params.dni?.toString() || ''
  perfil.fetchData(dni)
}

onMounted(cargarPerfil)

watch(
  () => router.currentRoute.value.params.dni,
  () => {
    cargarPerfil()
  }
)
</script>

<template>
  <div class="perfil-container">
    <aside class="sidebar-section">
      <Info :user="perfil.perfil" :vinculo="perfil.vinculos" :Emergencia="perfil.contacto" />
    </aside>

    <main class="main-layout">
      <section class="vinculos-wrapper">
        <Vinculos :vinculos="perfil.vinculos" />
      </section>

      <section class="widgets-column">
        <div class="widget-item" v-if="perfil.banco">
          <Banco />
        </div>

        <div class="widget-item historial-scroll-container" v-if="perfil.historial.length > 0">
          <Historial :lista="perfil.historial" />
        </div>
      </section>
    </main>
  </div>
</template>

<style lang="scss" scoped>
$gap-size: 3vh;
$sidebar-base: 230px;
$widget-max-width: 350px;
.perfil-container {
  display: flex;
  flex-wrap: wrap;
  gap: $gap-size;
  width: 100%;
  justify-content: center;

  .sidebar-section {
    flex: 1 1 $sidebar-base;
    max-width: $sidebar-base;
  }

  .main-layout {
    flex: 1 1 0; // Ocupa el resto del espacio
    min-width: 300px; // Evita que se colapse demasiado
    display: flex;
    flex-direction: column;
    gap: $gap-size;

    .vinculos-wrapper {
      width: 100%; // Siempre ancho total del main-layout
    }

    .widgets-column {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(19vw, 1fr));
      align-items: start;
      justify-items: end;
      gap: 1.5rem;
      width: 100%;

      .widget-item {
        width: 100%;
      }

      .historial-scroll-container {
        max-width: 25vw;
        // overflow-y: auto;
      }
    }
  }
}
</style>
