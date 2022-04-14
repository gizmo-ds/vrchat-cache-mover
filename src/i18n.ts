import { nextTick } from "vue";
import { createI18n, Locale } from "vue-i18n";

export const SUPPORT_LOCALES = ["zh-CN", "en-US"];

const getResourceMessages = (r: any) => r.default || r;

export const i18n = createI18n({
  locale: "en-US",
  fallbackLocale: "en-US",
  messages: {
    "en-US": await import("./locales/en-US.json").then(getResourceMessages),
  },
});

export async function setI18nLanguage(locale: Locale) {
  if (!(locale in i18n.global.messages)) {
    await loadLocaleMessages(locale);
  }
  i18n.global.locale = locale;
  document.querySelector("html")!.setAttribute("lang", locale);
}

async function loadLocaleMessages(locale: Locale) {
  const messages = await import(
    /* @vite-ignore */ `./locales/${locale}.json`
  ).then(getResourceMessages);
  i18n.global.setLocaleMessage(locale, messages);
  return nextTick();
}
