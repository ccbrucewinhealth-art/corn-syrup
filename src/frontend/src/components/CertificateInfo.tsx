import { useTranslation } from 'react-i18next';
import CertificateInfoRow from './CertificateInfoRow';
import { debugAction } from '../lib/logger';
export interface CertificateInfoProps { cert?: Record<string, any> | null; }
export default function CertificateInfo({ cert = null }: CertificateInfoProps) { const { t }=useTranslation(); const rows=[['Subject',cert?.subject],['Issuer',cert?.issuer],['Valid From',cert?.valid_from],['Valid To',cert?.valid_to],['Fingerprint',cert?.fingerprint]]; debugAction('CertificateInfo','render',{hasCert:Boolean(cert)}); return <table className="card" data-source="components/CertificateInfo.vue"><tbody>{rows.map(([k,v])=><CertificateInfoRow key={k} label={t(k)} value={v ?? t('notAvailableShort')} />)}</tbody></table>; }
