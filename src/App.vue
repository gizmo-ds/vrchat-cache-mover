<script setup lang="ts">
import AppHeader from "./components/Header.vue";
import { invoke, dialog, clipboard } from "@tauri-apps/api";
import {
  FolderOpened,
  Switch,
  Delete,
  CopyDocument,
} from "@element-plus/icons-vue";

interface cache_directory {
  cache_directory: string;
}

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
  invoke("move_cache", { newPath: "D:\\Cache\\VRChat" })
    .then(() => {
      notification("操作成功", "success", 1500);
      totalCacheSize = "0 B";
    })
    .catch((err) => notification(err, "error"));
};
const removeCache = () => {
  invoke("remove_cache")
    .then(() => {
      notification("操作成功", "success", 1500);
      totalCacheSize = "0 B";
    })
    .catch((err) => notification(err, "error"));
};
const getVRChatConfig = async () =>
  invoke("vrchat_config")
    .catch((err) => notification(err, "error", 0))
    .then((result) => (vrchatConfig = JSON.parse(result as string)));
getVRChatConfig();
const openVRChatPath = () =>
  invoke("open_vrchat_path").catch((err) => notification(err, "error"));
const setNewCachePath = () =>
  dialog
    .open({ directory: true })
    .then((path) => path && (vrchatConfig.cache_directory = path as string));
const copyConfig = () =>
  clipboard
    .writeText(JSON.stringify(vrchatConfig))
    .catch((err) => notification(err, "error"))
    .then(() => notification("配置已复制到剪贴板", "success", 1500));
</script>

<template>
  <app-header />

  <el-tag class="total-cache"> 缓存: {{ totalCacheSize }} </el-tag>

  <el-input v-model="vrchatConfig.cache_directory" placeholder="请选择缓存目录">
    <template #prepend>缓存目录</template>
    <template #append>
      <el-button @click="setNewCachePath">选择目录</el-button>
    </template>
  </el-input>

  <div style="margin-top: 10px">
    <el-button @click="openVRChatPath" :icon="FolderOpened">
      打开配置目录
    </el-button>

    <el-button
      @click="moveCache"
      :disabled="!vrchatConfig.cache_directory"
      type="danger"
      :icon="Switch"
    >
      移动缓存文件
    </el-button>
    <el-button @click="removeCache" type="danger" :icon="Delete">
      删除缓存
    </el-button>

    <el-button type="primary" style="float: right"> 保存设置 </el-button>
    <el-button @click="copyConfig" :icon="CopyDocument" style="float: right">
      复制配置内容
    </el-button>
  </div>
</template>

<style>
.total-cache {
  position: fixed;
  top: 8px;
  right: 8px;
}
.total-cache,
.el-input-group__prepend {
  user-select: none;
}
</style>
