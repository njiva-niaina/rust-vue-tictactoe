import i18n from "@/i18n";

export const useTranslation = () => {
  const supportedLocales = ["en", "fr", "mg"];

  function setCurrentLocale(newLocale: "fr" | "en" | "mg") {
    i18n.global.locale.value = newLocale;
  }

  async function switchLanguage(newLocale: "fr" | "en" | "mg") {
    setCurrentLocale(newLocale);
    document.querySelector("html")?.setAttribute("lang", newLocale);
  }

  return {
    supportedLocales,
    switchLanguage,
  };
};
