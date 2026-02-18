import { defineStore } from "pinia";
import api from "../services/api";

export const useAutenticacionStore = defineStore("autenticacion", {
  state: () => ({
    token: localStorage.getItem("token") || (null as string | null),
    usuario: null as any,
  }),
  getters: {
    estaAutenticado: (state) => !!state.token,
  },
  actions: {
    async iniciarSesion(usuario: string, contrasena: string) {
      try {
        const response = await api.post("/login/", {
          username: usuario,
          password: contrasena,
        });
        this.token = response.data.token;
        if (this.token) {
          localStorage.setItem("token", this.token);
        }
      } catch (error) {
        console.error("Login failed:", error);
        throw error;
      }
    },
    cerrarSesion() {
      this.token = null;
      this.usuario = null;
      localStorage.removeItem("token");
    },
  },
});
