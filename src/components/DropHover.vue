<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { Folder } from "@element-plus/icons-vue";

let show = $ref(false);

listen("tauri://file-drop-hover", () => (show = true));
listen("tauri://file-drop-cancelled", () => (show = false));
listen("tauri://file-drop", () => (show = false));
</script>

<template>
  <div class="drop-hover" v-if="show">
    <div>
      <div>
        <el-icon size="3rem">
          <Folder />
        </el-icon>
        <p>Drop directory here</p>
      </div>
    </div>
  </div>
</template>

<style lang="scss">
.drop-hover {
  z-index: 100;
  width: 100vw;
  height: 100vh;
  position: fixed;
  display: block;
  top: 0;
  left: 0;

  >div {
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: var(--el-bg-color);

    >div {
      margin: 0 auto;
      width: 90vw;
      height: 90vh;
      opacity: 1;
      border-radius: 6px;
      text-align: center;
      border: 1px dashed var(--el-color-primary);
      user-select: none;

      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
    }
  }
}
</style>
