import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'
import wasmPack from 'vite-plugin-wasm-pack'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), vueDevTools(), wasmPack([], ['mon-fs-web-box'])],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
  server: {
    fs: {
      allow: ['..']
    }
  }
})
