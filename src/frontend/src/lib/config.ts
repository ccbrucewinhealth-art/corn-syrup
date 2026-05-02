import { devMode } from './dev';

export const config = {
  api: {
    host: devMode ? 'localhost' : (import.meta.env.VITE_API_HOST || 'localhost'),
    port: devMode ? 30010 : (Number(import.meta.env.VITE_API_PORT) || 30010),
    secure: import.meta.env.VITE_API_SECURE === 'true',
  },
  ws: {
    host: devMode ? 'localhost' : (import.meta.env.VITE_WS_HOST || 'localhost'),
    port: devMode ? 30010 : (Number(import.meta.env.VITE_WS_PORT) || 30010),
    secure: import.meta.env.VITE_WS_SECURE === 'true',
  },
};

console.log('[Config] Environment:', {
  devMode,
  config,
  env: import.meta.env,
});

const apiProtocol = config.api.secure ? 'https' : 'http';
const wsProtocol = config.ws.secure ? 'wss' : 'ws';

export const apiBase = `${apiProtocol}://${config.api.host}:${config.api.port}`;
export const wsBase = `${wsProtocol}://${config.ws.host}:${config.ws.port}`;

console.log('[Config] API Base URL:', apiBase);
console.log('[Config] WS Base URL:', wsBase);

export function getApiUrl(path: string) {
  return `${apiBase}${path}`;
}

export function getWsUrl() {
  return wsBase;
}

export default config;