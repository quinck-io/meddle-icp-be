import adapter from "@sveltejs/adapter-static"
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';


/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: adapter({
			pages: "dist",
			assets: "dist",
			fallback: "200.html",
			precompress: false,
			strict: true
		}),
		alias: {
			"$declarations/*": "../declarations/*"
		},
	}
}

export default config
