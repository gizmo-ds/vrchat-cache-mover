import { createI18n, Locale } from "vue-i18n";
import { usePreferredLanguages } from "@vueuse/core";
import enUS from "./locales/en-US.json";
import zhCN from "./locales/zh-CN.json";
import jaJP from "./locales/ja-JP.json";
import koKR from "./locales/ko-KR.json";

export const FALLBACK_LOCALE = "en-US";
export const SUPPORT_LOCALES = [
  { locale: "zh-CN", name: "简体中文" },
  { locale: "en-US", name: "English" },
  { locale: "ja-JP", name: "日本語 (機械翻訳)" },
  { locale: "ko-KR", name: "한국어 (기계 번역)" },
];

export function broswerLocale() {
  const languages = usePreferredLanguages();
  const broswerLocale = languages.value.find((lang) =>
    SUPPORT_LOCALES.find((v) => v.locale === lang) ? lang : undefined
  );
  return localStorage.getItem("locale") ?? broswerLocale ?? FALLBACK_LOCALE;
}

export async function setI18nLanguage(locale: Locale) {
  i18n.global.locale = locale;
  localStorage.setItem("locale", locale);
  document.querySelector("html")!.setAttribute("lang", locale);
}

export const i18n = createI18n({
  locale: broswerLocale(),
  fallbackLocale: FALLBACK_LOCALE,
  messages: {
    "en-US": enUS,
    "zh-CN": zhCN,
    "ja-JP": jaJP,
    "ko-KR": koKR,
  },
});
