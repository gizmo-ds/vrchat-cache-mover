import path from "path";
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import { ElementPlusResolver } from "unplugin-vue-components/resolvers";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue({
      reactivityTransform: true,
    }),

    AutoImport({
      resolvers: [
        ElementPlusResolver(),
      ],
      dts: path.resolve("src/auto-imports.d.ts"),
    }),

    Components({
      resolvers: [
        ElementPlusResolver(),
      ],
      dts: path.resolve("src/components.d.ts"),
    }),
  ],
});
