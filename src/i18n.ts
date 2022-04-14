import { nextTick } from "vue";
import { createI18n, Locale } from "vue-i18n";
import enUS from "./locales/en-US.json";

export const SUPPORT_LOCALES = ["zh-CN", "en-US"];

export const i18n = createI18n({
  locale: "en-US",
  fallbackLocale: "en-US",
  messages: { "en-US": enUS },
});

export async function setI18nLanguage(locale: Locale) {
  if (!(locale in i18n.global.messages)) {
    await loadLocaleMessages(locale);
  }
  i18n.global.locale = locale;
  document.querySelector("html")!.setAttribute("lang", locale);
}

const getResourceMessages = (r: any) => r.default || r;

async function loadLocaleMessages(locale: Locale) {
  const messages = await import(
    /* @vite-ignore */ `./locales/${locale}.json`
  ).then(getResourceMessages);
  i18n.global.setLocaleMessage(locale, messages);
  return nextTick();
}
