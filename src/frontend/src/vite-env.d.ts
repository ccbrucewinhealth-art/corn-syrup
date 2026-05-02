/// <reference types="vite/client" />

declare module '*.css' {
  const content: string;
  export default content;
}

interface ImportMetaEnv {
  readonly DEV: boolean;
  readonly VITE_API_HOST: string;
  readonly VITE_API_PORT: string;
  readonly VITE_API_SECURE: string;
  readonly VITE_WS_HOST: string;
  readonly VITE_WS_PORT: string;
  readonly VITE_WS_SECURE: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
