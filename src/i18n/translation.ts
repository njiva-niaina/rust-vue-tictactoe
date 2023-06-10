import i18n from ".";

export const useTranslation = () => {
  const supportedLocales = import.meta.env.VITE_SUPPORTED_LOCALES.split(",");

  function setCurrentLocale(newLocale: string) {
    i18n.global.locale.value = newLocale;
  }

  async function switchLanguage(newLocale: string) {
    setCurrentLocale(newLocale);
    document.querySelector("html")?.setAttribute("lang", newLocale);
  }

  return {
    supportedLocales,
    switchLanguage,
  };
};
