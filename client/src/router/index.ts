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
    {
      path: "/view",
      name: "view",
      component: () => import("../views/ResponsesView.vue"),
    },
    {
      path: "/:path(.*)*",
      name: "not-found",
      component: () => import("../views/NotFoundView.vue"),
    },
  ],
});

export default router;
