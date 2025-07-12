import { TanStackRouterVite } from "@tanstack/router-plugin/vite";
import react from "@vitejs/plugin-react";
// import { visualizer } from "rollup-plugin-visualizer";
import type { Plugin } from "vite";
import { defineConfig } from "vite";
import compression from "vite-plugin-compression";
// import { VitePWA } from "vite-plugin-pwa";

// Custom Buffer polyfill plugin
const bufferPolyfillPlugin: Plugin = {
  name: "buffer-polyfill",
  resolveId(id) {
    if (id === "buffer") {
      return "buffer";
    }
    return null;
  },
  load(id) {
    if (id === "buffer") {
      return `
        import { Buffer } from 'buffer';
        export { Buffer };
        export default { Buffer };
      `;
    }
    return null;
  },
};

const handlePureAnnotations: Plugin = {
  name: "handle-pure-annotations",
  transform(code: string, id: string) {
    if (id.includes("@privy-io/react-auth")) {
      return {
        code: code.replace(/\/\*#__PURE__\*\//g, ""),
        map: null,
      };
    }
  },
};

export default defineConfig({
  plugins: [
    bufferPolyfillPlugin,
    handlePureAnnotations,
    TanStackRouterVite(),
    react(),
    // VitePWA({
    //   registerType: "autoUpdate",
    //   manifest: {
    //     name: "Listen",
    //     short_name: "Listen",
    //     description: "Listen",
    //     theme_color: "#000000",
    //     background_color: "#151518",
    //     display: "standalone",
    //     start_url: "/",
    //     icons: [
    //       {
    //         src: "/listen-icon.png",
    //         sizes: "192x192",
    //         type: "image/png",
    //         purpose: "any maskable",
    //       },
    //       {
    //         src: "/listen-icon.png",
    //         sizes: "512x512",
    //         type: "image/png",
    //         purpose: "any maskable",
    //       },
    //     ],
    //   },
    //   workbox: {
    //     maximumFileSizeToCacheInBytes: 10 * 1024 * 1024, // 增加到 10MB
    //   },
    // }),
    // visualizer(), // 暂时注释掉这个插件
    compression(),
  ],
  optimizeDeps: {
    exclude: ["@tanstack/router-vite-plugin"],
  },
  define: {
    global: "globalThis",
    "process.env": {},
  },
  build: {
    target: "esnext",
    minify: "esbuild",
    sourcemap: false,
    chunkSizeWarningLimit: 2000, // 增加警告限制
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom'],
          router: ['@tanstack/react-router'],
          charts: ['lightweight-charts', 'scichart'],
        },
      },
    },
  },
  server: {
    fs: {
      allow: [".."],
    },
  },
  assetsInclude: ["**/*.wasm"],
});
