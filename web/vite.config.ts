import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],

	server: {
		proxy: {
			// 配置代理规则
			'/api': {
				target: 'http://localhost:8090', // API 服务的地址
				changeOrigin: true,
				rewrite: (path) => path.replace(/^\/api/, '/api')
			}
		},
		cors: true
	},
	build: {
		chunkSizeWarningLimit: 1500
	}
});
