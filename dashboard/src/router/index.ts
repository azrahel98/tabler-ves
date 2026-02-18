import { createRouter, createWebHistory } from "vue-router";
import LoginView from "../views/LoginView.vue";
import MainLayout from "../layouts/MainLayout.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  linkActiveClass: "menu-item-active",
  linkExactActiveClass: "menu-item-active",
  routes: [
    {
      path: "/login",
      name: "login",
      component: LoginView,
    },
    {
      path: "/",
      component: MainLayout,
      meta: { requiresAuth: true },
      children: [
        {
          path: "",
          redirect: "/dash",
        },
        {
          path: "dash",
          name: "dashboard",
          component: () => import("../views/DashboardView.vue"),
        },
        {
          path: "personal",
          name: "personal",
          component: () => import("../views/Personal/PersonalSearch.vue"),
        },
        {
          path: "personal/:dni",
          name: "personal-profile",
          component: () => import("../views/Personal/PersonalProfile.vue"),
        },
        {
          path: "change-password",
          name: "change-password",
          component: () => import("../views/ChangePassword.vue"),
        },
      ],
    },
  ],
});

export default router;
