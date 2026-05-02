import { useTranslation } from 'react-i18next';
import { debugAction } from '../lib/logger';
export interface StatusProps { status?: number; }
export default function Status({ status = 0 }: StatusProps) {
  const { t } = useTranslation();
  const map: Record<number, [string,string]> = { 0: ['danger','Down'], 1: ['primary','Up'], 2: ['warning','Pending'], 3: ['maintenance','statusMaintenance'] };
  const [color, key] = map[status] || ['secondary','Unknown'];
  debugAction('Status','render', { status, color });
  return <span className={`badge rounded-pill bg-${color}`} data-source="components/Status.vue">{t(key)}</span>;
}
