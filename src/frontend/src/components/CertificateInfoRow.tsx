export interface CertificateInfoRowProps { label?: string; value?: string | number | null; }
export default function CertificateInfoRow({ label = '', value = '' }: CertificateInfoRowProps) { return <tr data-source="components/CertificateInfoRow.vue"><th>{label}</th><td>{String(value ?? '')}</td></tr>; }
