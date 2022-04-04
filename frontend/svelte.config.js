import adapter from '@sveltejs/adapter-auto'
import preprocess from 'svelte-preprocess'
import path, { dirname, resolve } from 'path'

import { fileURLToPath } from 'url'

let __dirname = dirname(fileURLToPath(import.meta.url))

/** @type {import('@sveltejs/kit').Config} */
const config = {
    // Consult https://github.com/sveltejs/svelte-preprocess
    // for more information about preprocessors
    preprocess: preprocess({
        stylus: {
            paths: [resolve(__dirname, './src/stylus')] //allow absolute import from /app directory
        }
    }),

    kit: {
        adapter: adapter(),
        vite: {
            server: {
                hmr: {
                    clientPort: 443,
                    port: 80,
                    protocol: "wss",
                }
            },
            resolve: {
                alias: {
                    '@components': resolve(__dirname, './src/components'),
                    '@lib': resolve(__dirname, './src/lib'),
                    '@layouts': resolve(__dirname, './src/layouts'),
                    '@includes': resolve(__dirname, './src/includes'),
                    "@utils": resolve(__dirname, './src/utils'),
                    "@icons": resolve(__dirname, './node_modules/svelte-material-icons'),
                    "@stores": resolve(__dirname, './src/stores'),
                    "@icons": resolve(__dirname, './node_modules/svelte-material-icons'),
                }
            }
        }
    }
};


export default config;
