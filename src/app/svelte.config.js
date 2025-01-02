// import adapter from '@sveltejs/adapter-auto';
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

const dev = process.env.NODE_ENV === 'development';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		// adapter-auto only supports some environments, see https://svelte.dev/docs/kit/adapter-auto for a list.
		// If your environment is not supported, or you settled on a specific environment, switch out the adapter.
		// See https://svelte.dev/docs/kit/adapters for more information about adapters.
		// adapter: adapter()
		adapter: adapter({
			pages: 'dist',
			assets: 'dist',
			fallback: undefined,
			precompress: false,
			strict: true
		}),
		alias: {
			$lib: 'src/lib/',
			$declarations: '../declarations',
			$components: 'src/components',
			$routes: 'src/routes'
		}
	}
};

export default config;
