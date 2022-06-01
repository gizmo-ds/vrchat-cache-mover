<script lang="ts" setup>
import { useI18n } from "vue-i18n";
import { setI18nLanguage, SUPPORT_LOCALES } from "../i18n";
import { Language as LanguageIcon } from "@vicons/tabler";

const { t, locale } = useI18n();
let language = $ref(locale.value);

const options = SUPPORT_LOCALES.map((v) => ({
  label: v.name,
  value: v.locale,
}));
</script>

<template>
  <div class="app-header">
    <div class="logo">
      <div>{{ t("name") }}</div>
    </div>

    <el-icon class="language-icon" :size="24">
      <language-icon />
    </el-icon>

    <el-select
      v-model="language"
      class="language"
      size="small"
      @change="(v) => setI18nLanguage(v)"
    >
      <el-option
        v-for="item in options"
        :key="item.value"
        :label="item.label"
        :value="item.value"
      />
    </el-select>
  </div>
</template>

<style lang="scss" scoped>
$title-width: 330px;
.app-header {
  width: 100%;
  height: 75px;
  .logo {
    user-select: none;
    div {
      width: $title-width;
      height: 60px;
      line-height: 60px;
      font-size: 32px;
      cursor: pointer;
      color: var(--text-color);
      text-align: center;
      transition: color 0.5s;
      transform: translate(0%);
    }

    div::before {
      content: "";
      position: absolute;
      left: 0%;
      bottom: 0;
      width: $title-width;
      transform: scaleX(0);
      height: 2px;
      background: #ff6266;
      z-index: -1;
      transition: transform 0.5s;
      transform-origin: 100% 0;
    }

    div:hover::before {
      transform: scaleX(1);
      transform-origin: 0 0;
    }
  }

  .language {
    position: fixed;
    top: 45px;
    right: 8px;
    width: 180px;
  }
  .language-icon {
    position: fixed;
    top: 45px;
    right: 190px;
  }
}
</style>
