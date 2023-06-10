import { createI18n } from "vue-i18n";

import en from "@/i18n/locales/en.json";
import fr from "@/i18n/locales/fr.json";
import mg from "@/i18n/locales/mg.json";

export default createI18n({
  locale: "fr",
  fallbackLocale: "en",
  legacy: false,
  globalInjection: true,
  messages: {
    fr,
    en,
    mg,
  },
});
