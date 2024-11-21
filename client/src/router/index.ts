import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/share",
      name: "share",
      component: () => import("../views/FormView.vue"),
    },
    {
      path: "/organize",
      name: "organize",
      component: () => import("../views/OrganizeView.vue"),
    },
  ],
});

export default router;
