import { createPinia } from "pinia";
import { createApp } from "vue";

import App from "./App.vue";
import i18n from "./i18n";
import "./styles.css";

const pinia = createPinia();
const app = createApp(App);

app.use(pinia);
app.use(i18n);
app.mount("#app");
