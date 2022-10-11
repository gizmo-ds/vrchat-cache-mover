<script setup lang="ts">
import AppHeader from "./components/Header.vue";
import DropHover from "./components/DropHover.vue";
import { invoke, dialog, clipboard, app } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { useI18n } from "vue-i18n";
import {
  BrandGithub,
  BuildingStore,
  Sun,
  Moon,
  Folder,
  SwitchHorizontal,
  TrashX,
  Copy,
} from "@vicons/tabler";
import { UseDark } from "@vueuse/components";
import pkg from "../package.json";

interface cache_directory {
  cache_directory: string;
  cache_expiry_delay?: number;
  cache_size?: number;
}

const { t } = useI18n();

let appVersion = $ref("");
app.getVersion().then((version) => (appVersion = version));
let totalCacheSize = $ref("0 B");
let vrchatConfig: cache_directory = $ref({ cache_directory: "" });
let showAdvanced = $ref(false);

// 监听 tauri 的文件拖拽事件
listen<string[]>("tauri://file-drop", (event) => {
  const files = event.payload;
  if (!files || files.length === 0) return;
  invoke("check_new_path", { newPath: files[0] })
    .then(() => (vrchatConfig.cache_directory = files[0]))
    .catch((err) => notification(t(`messages.${err}`), "error"));
});

// 等待主題切換后显示GUI
setTimeout(() => invoke("ui_created"), 200);

const notification = (message: string, type: any, duration: number = 4500) =>
  ElNotification({ message, type, duration, position: "bottom-right" });

let disabled = $ref(true);

invoke("check_vrchat_path")
  .catch((err) => notification(t(`messages.${err}`), "error"))
  .then(() => (disabled = false));

const totalCache = () =>
  invoke("total_cache").then(
    (result) => result && (totalCacheSize = result as string)
  );
totalCache();
const moveCache = () => {
  invoke("move_cache", { newPath: vrchatConfig.cache_directory })
    .then(() => {
      notification(t("messages.success"), "success", 1500);
      totalCache();
    })
    .catch((err) => notification(t(`messages.${err}`), "error"));
};
const removeCache = () => {
  invoke("remove_cache")
    .then(() => {
      notification(t("messages.success"), "success", 1500);
      totalCache();
    })
    .catch((err) => notification(t(`messages.${err}`), "error"));
};
const getVRChatConfig = async () =>
  invoke("vrchat_config")
    .catch((err) => notification(t(`messages.${err}`), "error", 0))
    .then((result) => (vrchatConfig = JSON.parse(result as string)));
getVRChatConfig().then(
  () => vrchatConfig.cache_directory || (vrchatConfig.cache_directory = "")
);
const openVRChatPath = () =>
  invoke("open_vrchat_path").catch((err) =>
    notification(t(`messages.${err}`), "error")
  );
const selectDirectory = () =>
  dialog
    .open({ directory: true })
    .then((path) => path && (vrchatConfig.cache_directory = path as string));
const copyConfig = () =>
  clipboard
    .writeText(JSON.stringify(vrchatConfig))
    .catch((err) => notification(t(`messages.${err}`), "error"))
    .then(() => notification(t("messages.success"), "success", 1500));
const saveConfig = () => {
  const save = () => {
    invoke("save_config", { config: JSON.stringify(vrchatConfig) })
      .then(() => notification(t("messages.success"), "success", 1500))
      .catch((err) => notification(t(`messages.${err}`), "error"));
  };
  if (vrchatConfig.cache_directory !== "") {
    return invoke("check_new_path", { newPath: vrchatConfig.cache_directory })
      .then(save)
      .catch((err) => notification(t(`messages.${err}`), "error"));
  }
  save();
};
const configNumberKeys = ["cache_size", "cache_expiry_delay"];
const configChange = () => {
  const c = vrchatConfig;
  for (const k in vrchatConfig) {
    if (configNumberKeys.includes(k)) {
      //@ts-ignore
      if (c[k] === "") c[k] = undefined;
      //@ts-ignore
      else c[k] = Number(c[k]);
    }
  }
};
</script>

<template>
  <el-config-provider>
    <use-dark v-slot="{ isDark, toggleDark }">
      <app-header />

      <drop-hover />

      <div
        :style="{
          position: 'fixed',
          top: '8px',
          right: '8px',
          userSelect: 'none',
        }"
      >
        <el-tag
          @click="totalCache"
          :style="{ cursor: 'pointer' }"
          :title="t('cache-total.tooltip')"
        >
          {{ t("cache-total", [totalCacheSize]) }}
        </el-tag>
        <el-tag :style="{ marginLeft: '8px' }">
          {{ t("version", [appVersion]) }}
        </el-tag>

        <el-switch
          :model-value="isDark"
          inline-prompt
          :active-icon="Moon"
          :inactive-icon="Sun"
          @click="toggleDark()"
          :style="{ marginLeft: '8px' }"
        />
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
        <el-button @click="openVRChatPath" :icon="Folder">
          {{ t("open-config-path-button") }}
        </el-button>

        <el-button
          @click="moveCache"
          :disabled="!vrchatConfig.cache_directory || disabled"
          type="warning"
          :icon="SwitchHorizontal"
        >
          {{ t("move-cache-button") }}
        </el-button>
        <el-button
          @click="removeCache"
          type="danger"
          :icon="TrashX"
          :disabled="disabled"
        >
          {{ t("delete-cache-button") }}
        </el-button>

        <el-button
          @click="saveConfig"
          type="primary"
          style="float: right"
          :disabled="disabled"
        >
          {{ t("apply-button") }}
        </el-button>
        <el-button @click="copyConfig" :icon="Copy" style="float: right">
          {{ t("copy-config-content-button") }}
        </el-button>
      </div>

      <div style="margin-top: 10px">
        <el-button type="primary" link @click="showAdvanced = !showAdvanced">
          {{ t("advanced") }}
        </el-button>

        <div class="advanced" v-show="showAdvanced">
          <el-space wrap>
            <el-input
              type="number"
              v-model="vrchatConfig.cache_expiry_delay"
              :placeholder="t('advanced.cache-expiry-delay.placeholder')"
              @change="configChange"
            >
              <template #prepend>
                {{ t("advanced.cache-expiry-delay") }}
              </template>
              <template #append>
                {{ t("advanced.cache-expiry-delay.suffix") }}
              </template>
            </el-input>
            <el-input
              type="number"
              v-model="vrchatConfig.cache_size"
              :placeholder="t('advanced.cache-size.placeholder')"
              @change="configChange"
            >
              <template #prepend>
                {{ t("advanced.cache-size") }}
              </template>
              <template #append>
                {{ t("advanced.cache-size.suffix") }}
              </template>
            </el-input>
          </el-space>
        </div>
      </div>

      <div class="links">
        <el-link :href="pkg.homepage" target="_blank" type="primary">
          <brand-github /> GitHub
        </el-link>
        <el-link
          href="https://gizmooooo.booth.pm/"
          target="_blank"
          type="primary"
        >
          <building-store /> BOOTH
        </el-link>
      </div>
    </use-dark>
  </el-config-provider>
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

.advanced {
  padding-left: 2px;
}
</style>

<style lang="scss">
.el-input-group__prepend {
  user-select: none;
}

.el-switch__core {
  .el-icon.is-show {
    color: var(--el-color-black);
  }
}

.el-link {
  * {
    user-select: none;
  }
}

html.dark {
  .el-switch__core {
    border-color: var(--el-switch-off-color) !important;
    background-color: var(--el-switch-off-color) !important;

    .el-icon.is-show {
      color: var(--el-color-white);
    }
  }
}
input[type="number"]::-webkit-inner-spin-button,
input[type="number"]::-webkit-outer-spin-button {
  appearance: none;
  margin: 0;
}
</style>
