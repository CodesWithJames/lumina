import adapter from '@sveltejs/adapter-auto';
// import node from '@sveltejs/adapter-node'
import preprocess from 'svelte-preprocess';
import { dirname, resolve } from 'path'
import { fileURLToPath } from 'url'

let __dirname = dirname(fileURLToPath(import.meta.url))

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
    preprocess: preprocess({
        stylus: {
            // this will allow us to @import 'variables' inside of our svelte stylus css section
            paths: [resolve(__dirname, './src/stylus')]
        }
    }),
	kit: {
		adapter: adapter(),
		// adapter: node(),
        vite: {
            server: {
                hmr: {
                    clientPort: 443,
                    port: 80,
                    protocol: "wss",
                }
            },
            resolve: {
                // tell vite how to resolve our import aliases
                alias: {
                    '$components': resolve(__dirname, './src/components'),
                    '$templates': resolve(__dirname, './src/templates'),
                    '$layouts': resolve(__dirname, './src/layouts'),
                    "$utils": resolve(__dirname, './src/utils'),
                    "$icons": resolve(__dirname, './node_modules/svelte-material-icons'),
                    "$stores": resolve(__dirname, './src/stores'),
                    "$main": resolve(__dirname, './src/main'),
                    "$types": resolve(__dirname, './src/types'),
                }
            }
        }
	}
};

export default config;
