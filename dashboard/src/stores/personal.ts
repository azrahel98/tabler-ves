import { defineStore } from "pinia";
import api from "../services/api";
import { ref } from "vue";

export const usePersonalStore = defineStore("personal", () => {
  const resultadosBusqueda = ref<any[]>([]);
  const perfilActual = ref<any>(null);
  const vinculos = ref<any[]>([]); // Vinculos is already spanish
  const infoBancaria = ref<any>(null);
  const grados = ref<any>(null);
  const contactoEmergencia = ref<any>(null);
  const cargando = ref(false);

  async function buscar(nombre: string) {
    cargando.value = true;
    try {
      const res = await api.post("/personal/buscar", { nombre });
      resultadosBusqueda.value = res.data;
    } finally {
      cargando.value = false;
    }
  }

  async function buscarGlobal(nombre: string) {
    // No set loading state to avoid global spinner if not needed, or create local loading state in component
    // But for now, we just fetch and return.
    const res = await api.post("/personal/buscar", { nombre });
    return res.data;
  }

  async function obtenerPerfil(dni: string) {
    cargando.value = true;
    try {
      const res = await api.post("/personal/por_dni", { dni });
      perfilActual.value = res.data;

      // Fetch related data
      await Promise.all([
        obtenerVinculos(dni),
        obtenerInfoBancaria(dni),
        obtenerGrados(dni),
        obtenerContacto(dni),
      ]);
    } finally {
      cargando.value = false;
    }
  }

  async function obtenerVinculos(dni: string) {
    const res = await api.post("/personal/vinculos_por_dni", { dni });
    vinculos.value = res.data;
  }

  async function obtenerInfoBancaria(dni: string) {
    const res = await api.post("/personal/banco_por_dni", { dni });
    infoBancaria.value = res.data;
  }

  async function obtenerGrados(dni: string) {
    const res = await api.post("/personal/grado_por_dni", { dni });
    grados.value = res.data;
  }

  async function obtenerContacto(dni: string) {
    const res = await api.post("/personal/contacto_dni", { dni });
    contactoEmergencia.value = res.data;
  }

  async function actualizarPerfil(data: any) {
    await api.post("/personal/editar_por_dni", data);
    await obtenerPerfil(data.dni);
  }

  return {
    resultadosBusqueda,
    perfilActual,
    vinculos,
    infoBancaria,
    grados,
    contactoEmergencia,
    cargando,
    buscar,
    buscarGlobal,
    obtenerPerfil,
    updateProfile: actualizarPerfil, // Alias for compatibility during transition or just rename? Plan said updateProfile -> actualizarPerfil. I will rename.
    obtenerVinculos,
    obtenerInfoBancaria,
    obtenerGrados,
    obtenerContacto,
    actualizarPerfil,
  };
});
