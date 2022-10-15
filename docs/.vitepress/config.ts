import { defineConfig } from "vitepress";

export default defineConfig({
  lang: "zh-CN",
  title: "VRChat Cache Mover",
  description: "将VRChat缓存移动到指定目录的小工具, 拥有相对现代并且易用的UI.",
  lastUpdated: true,
  base: "/vrchat-cache-mover/",
  cleanUrls: "with-subfolders",
  themeConfig: {
    footer: {
      message: "Released under the AGPL-3.0 License.",
      copyright: "Copyright (c) 2022 Gizmo",
    },
    nav: [
      {
        text: "指南",
        link: "/guide",
      },
      {
        text: "GitHub",
        link: "https://github.com/gizmo-ds/vrchat-cache-mover",
      },
    ],
    sidebar: {
      "/guide": [
        {
          items: [
            { text: "介绍", link: "/guide#vrchat-缓存转移工具" },
            { text: "安装及使用", link: "/guide#安装及使用" },
            { text: "网页版本", link: "/guide#网页版本" },
            { text: "常见问题", link: "/guide#常见问题" },
          ],
        },
      ],
    },
    socialLinks: [
      {
        icon: "github",
        link: "https://github.com/gizmo-ds/vrchat-cache-mover",
      },
    ],
  },
});
