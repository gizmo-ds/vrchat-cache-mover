<script setup lang="ts">
import AppHeader from "./components/Header.vue";
import { invoke, dialog, clipboard, app } from "@tauri-apps/api";
import { useI18n } from "vue-i18n";
import {
  FolderOpened,
  Switch,
  Delete,
  CopyDocument,
} from "@element-plus/icons-vue";
import { BrandGithub, BuildingStore } from "@vicons/tabler";

interface cache_directory {
  cache_directory: string;
}

const { t } = useI18n();

let appVersion = $ref("");
app.getVersion().then((version) => (appVersion = version));
let totalCacheSize = $ref("0 B");
let vrchatConfig: cache_directory = $ref({ cache_directory: "" });

const notification = (message: string, type: any, duration: number = 4500) =>
  ElNotification({
    message,
    type,
    duration,
  });
const totalCache = () =>
  invoke("total_cache").then(
    (result) => result && (totalCacheSize = result as string)
  );
totalCache();
const moveCache = () => {
  invoke("move_cache", { newPath: vrchatConfig.cache_directory })
    .then(() => {
      notification(t("messages.success"), "success", 1500);
      totalCacheSize = "0 B";
    })
    .catch((err) => notification(t(err), "error"));
};
const removeCache = () => {
  invoke("remove_cache")
    .then(() => {
      notification(t("messages.success"), "success", 1500);
      totalCacheSize = "0 B";
    })
    .catch((err) => notification(t(err), "error"));
};
const getVRChatConfig = async () =>
  invoke("vrchat_config")
    .catch((err) => notification(t(err), "error", 0))
    .then((result) => (vrchatConfig = JSON.parse(result as string)));
getVRChatConfig();
const openVRChatPath = () =>
  invoke("open_vrchat_path").catch((err) => notification(t(err), "error"));
const selectDirectory = () =>
  dialog
    .open({ directory: true })
    .then((path) => path && (vrchatConfig.cache_directory = path as string));
const copyConfig = () =>
  clipboard
    .writeText(JSON.stringify(vrchatConfig))
    .catch((err) => notification(t(err), "error"))
    .then(() => notification(t("messages.success"), "success", 1500));
const saveConfig = () =>
  invoke("save_config", { config: JSON.stringify(vrchatConfig) })
    .then(() => notification(t("messages.success"), "success", 1500))
    .catch((err) => notification(t(err), "error"));
</script>

<template>
  <app-header />

  <div
    :style="{ position: 'fixed', top: '8px', right: '8px', userSelect: 'none' }"
  >
    <el-tag @click="totalCache" :style="{ cursor: 'pointer' }">
      {{ t("cache", [totalCacheSize]) }}
    </el-tag>
    <el-tag :style="{ marginLeft: '8px' }">
      {{ t("version", [appVersion]) }}
    </el-tag>
  </div>

  <el-input
    v-model="vrchatConfig.cache_directory"
    :placeholder="t('cache-placeholder')"
    clearable
  >
    <template #prepend>{{ t("cache-directory") }}</template>
    <template #append>
      <el-button @click="selectDirectory">
        {{ t("select-directory-button") }}
      </el-button>
    </template>
  </el-input>

  <div style="margin-top: 10px">
    <el-button @click="openVRChatPath" :icon="FolderOpened">
      {{ t("open-config-path-button") }}
    </el-button>

    <el-button
      @click="moveCache"
      :disabled="!vrchatConfig.cache_directory"
      color="#eebe77"
      style="color: var(--el-color-white)"
      :icon="Switch"
    >
      {{ t("move-cache-button") }}
    </el-button>
    <el-button @click="removeCache" type="danger" :icon="Delete">
      {{ t("delete-cache-button") }}
    </el-button>

    <el-button @click="saveConfig" type="primary" style="float: right">
      {{ t("apply-button") }}
    </el-button>
    <el-button @click="copyConfig" :icon="CopyDocument" style="float: right">
      {{ t("copy-config-content-button") }}
    </el-button>
  </div>

  <div class="links">
    <el-link
      href="https://github.com/GizmoOAO/vrchat-cache-mover"
      target="_blank"
      type="primary"
    >
      <brand-github /> GitHub
    </el-link>
    <el-link href="https://gizmooooo.booth.pm/" target="_blank" type="primary">
      <building-store /> BOOTH
    </el-link>
  </div>
</template>

<style lang="scss" scoped>
.links {
  position: absolute;
  bottom: 0.8rem;
  left: 1rem;
  > * {
    margin-right: 0.8em;
  }
  .el-link {
    svg {
      width: 1.5em;
      height: 1.5em;
      margin-right: 0.3em;
    }
  }
}
</style>

<style lang="scss">
.el-input-group__prepend {
  user-select: none;
}
</style>
