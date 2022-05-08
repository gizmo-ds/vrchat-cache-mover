<script lang="ts" setup>
import { useI18n } from "vue-i18n";
import { setI18nLanguage, SUPPORT_LOCALES } from "../i18n";

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
.app-header {
  width: 100%;
  height: 75px;
  .logo {
    user-select: none;
    div {
      width: 330px;
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
      width: 330px;
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
  }
}
</style>
