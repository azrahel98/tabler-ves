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

        <div class="widget-item historial-scroll-container">
          <Historial :lista="perfil.historial" />
        </div>

        <!-- <button class="btn-ver-mas">Ver más</button> -->
      </section>
    </main>
  </div>
</template>

<style lang="scss" scoped>
$gap-size: 3vh;
$sidebar-base: 250px;
$widget-max-width: 350px; // Definimos un ancho máximo para los widgets

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
    flex: 999 1 500px;
    display: flex;
    flex-wrap: wrap;
    gap: $gap-size;
    align-items: flex-start;

    .vinculos-wrapper {
      flex: 2 1 400px;
      min-width: 300px;
    }

    .widgets-column {
      // 1. Evitamos que la columna crezca infinitamente a lo ancho
      flex: 1 1 300px;
      max-width: $widget-max-width;

      display: flex;
      flex-direction: column;
      gap: 1.5rem;

      .historial-scroll-container {
        // 2. Control de altura y scroll
        max-height: 400px; // Ajusta este valor según tu diseño
        overflow-y: auto;
        overflow-x: hidden;

        // Estilo opcional para la barra de scroll (más estética)
        &::-webkit-scrollbar {
          width: 6px;
        }
        &::-webkit-scrollbar-thumb {
          background: #ccc;
          border-radius: 10px;
        }
      }

      .btn-ver-mas {
        width: fit-content;
        align-self: flex-start;
        padding: 0.6rem 1.2rem;
        background: #3b82f6;
        color: white;
        border-radius: 6px;
        border: none;
        cursor: pointer;
      }
    }
  }
}
</style>
