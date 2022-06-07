import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),
        webappContainer: resolve(__dirname, 'webapp-container/index.html'),
        webappLock: resolve(__dirname, 'webapp-lock/index.html'),
      },
      output:{

      }
    }
  },
  plugins: [svelte()]
})
