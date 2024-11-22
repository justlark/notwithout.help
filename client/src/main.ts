import "./assets/main.css";
import "primeicons/primeicons.css";

import { createApp } from "vue";
import ToastService from "primevue/toastservice";
import PrimeVue from "primevue/config";
import Aura from "@primevue/themes/aura";

import App from "./App.vue";
import router from "./router";

const app = createApp(App);

app.use(router);
app.use(PrimeVue, {
  theme: {
    preset: Aura,
  },
});
app.use(ToastService);

app.mount("#app");
