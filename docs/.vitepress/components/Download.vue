<script lang="ts" setup>
import { ref } from "vue";

const props = defineProps({
  repo: String,
  filename: String,
});
const tag = ref("???")
const downloadUrl = ref(["javascript:;", "javascript:;", "javascript:;"])

fetch("https://api.lumina.moe/gh/repos/" + props.repo + "/releases/latest")
  .then((resp) => resp.json())
  .then((data) => {
    if (!data.assets) return;
    tag.value = data.tag_name;
    data.assets.forEach((asset) => {
      if (props.filename) if (!new RegExp(props.filename).exec(asset.name)) return;
      downloadUrl.value = [
        asset.browser_download_url,
        'https://ghproxy.com/' + asset.browser_download_url,
        'https://hub.fastgit.xyz/' + asset.browser_download_url.replace('https://github.com', ''),
      ];
    })
  })
</script>

<template>
  <div style="height:64px">
    <p style="margin-bottom:0">Release: {{ tag }} </p>
    <div>
      <a class="download-button" :href="downloadUrl[0]">Download</a>
      <a class="download-button" :href="downloadUrl[1]">加速下载1</a>
      <a class="download-button" :href="downloadUrl[2]">加速下载2</a>
    </div>
  </div>
</template>

<style scoped>
.download-button {
  display: inline-block;
  border: 1px solid transparent;
  text-align: center;
  font-weight: 500;
  border-radius: 20px;
  padding: 0 20px;
  margin-right: 12px;
  line-height: 26px;
  font-size: 12px;
  border-color: var(--vp-button-alt-border);
  color: var(--vp-button-alt-text);
  background-color: var(--vp-button-alt-bg);
  transition: color 0.25s, border-color 0.25s, background-color 0.25s;
}

.download-button:active {
  transition: color 0.1s, border-color 0.1s, background-color 0.1s;
}

.download-button:hover {
  border-color: var(--vp-button-alt-hover-border);
  color: var(--vp-button-alt-hover-text);
  background-color: var(--vp-button-alt-hover-bg);
}
</style>