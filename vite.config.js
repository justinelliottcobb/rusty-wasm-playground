import { defineConfig, loadEnv } from 'vite';

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '');
  
  return {
    server: {
      host: '0.0.0.0', // Bind to all interfaces
      port: 3000,      // Use unprivileged port
      strictPort: true, // Fail if port is occupied
      allowedHosts: env.VITE_ALLOWED_HOSTS 
        ? env.VITE_ALLOWED_HOSTS.split(',').map(host => host.trim())
        : ['localhost'],
      fs: {
        allow: ['..']
      },
      watch: {
        ignored: ['**/pkg/**', '**/target/**']
      }
    },
    optimizeDeps: {
      exclude: ['./pkg']
    }
  };
});