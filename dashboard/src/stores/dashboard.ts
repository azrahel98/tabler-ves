import { defineStore } from "pinia";
import api from "../services/api";
import { ref } from "vue";

export const useTableroStore = defineStore("tablero", () => {
  const resumen = ref<any>(null);
  const cumpleanos = ref<any[]>([]);
  const reporteAreas = ref<any[]>([]);
  const personalActivo = ref<any[]>([]);
  const historial = ref<any[]>([]);
  const organigrama = ref<any[]>([]);
  const listaRenuncias = ref<any[]>([]);
  const documentos = ref<any[]>([]);
  const bancos = ref<any[]>([]);

  const cargando = ref(false);

  async function obtenerResumen() {
    const res = await api.post("/dash/info");
    resumen.value = res.data;
  }

  async function obtenerCumpleanos() {
    const res = await api.post("/dash/cumpleaños");
    cumpleanos.value = res.data;
  }

  async function obtenerReporteAreas() {
    const res = await api.post("/dash/area_report");
    reporteAreas.value = res.data;
  }

  async function obtenerPersonalActivo() {
    const res = await api.post("/dash/personal_activo");
    personalActivo.value = res.data;
  }

  async function obtenerHistorial(dni?: string) {
    const res = await api.post("/dash/reporte_historia", { dni });
    historial.value = res.data;
  }

  async function obtenerOrganigrama() {
    const res = await api.post("/dash/organigrama");
    organigrama.value = res.data;
  }

  async function obtenerListaRenuncias() {
    const res = await api.post("/dash/reporte_renuncias");
    listaRenuncias.value = res.data;
  }

  async function obtenerDocumentos() {
    const res = await api.post("/dash/reporte_documentos");
    documentos.value = res.data;
  }

  async function obtenerBancos() {
    const res = await api.post("/dash/banco_report");
    bancos.value = res.data;
  }

  async function obtenerTodo() {
    cargando.value = true;
    try {
      await Promise.all([
        obtenerResumen(),
        obtenerCumpleanos(),
        obtenerReporteAreas(),

        // obtenerPersonalActivo(), // Might be too heavy to load initially?
        // obtenerHistorial(), // Needs DNI?
        // obtenerOrganigrama(),
        obtenerListaRenuncias(),
      ]);
    } catch (e) {
      console.error("Error fetching dashboard data", e);
    } finally {
      cargando.value = false;
    }
  }

  return {
    resumen,
    cumpleanos,
    reporteAreas,
    personalActivo,
    historial,
    organigrama,
    listaRenuncias,
    documentos,
    bancos,
    cargando,
    obtenerResumen,
    obtenerCumpleanos,
    obtenerReporteAreas,
    obtenerPersonalActivo,
    obtenerHistorial,
    obtenerOrganigrama,
    obtenerListaRenuncias,
    obtenerDocumentos,
    obtenerBancos,
    obtenerTodo,
  };
});
