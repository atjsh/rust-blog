import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	optimizeDeps: {
		exclude: ['codemirror']
	},

	kit: {
		adapter: adapter()
	}
};

export default config;
