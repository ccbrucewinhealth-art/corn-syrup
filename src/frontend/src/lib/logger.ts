import log from 'loglevel';

const viteEnv = (import.meta as ImportMeta & { env?: Record<string, string | undefined> }).env;
const configuredLevel = (viteEnv?.VITE_LOG_LEVEL || 'debug') as log.LogLevelDesc;
log.setLevel(configuredLevel);

export const logger = log.getLogger('corn-syrup-frontend');
logger.setLevel(configuredLevel);

export function debugAction(scope: string, action: string, payload?: unknown) {
  logger.debug(`[${scope}] ${action}`, payload ?? '');
}

export default logger;
