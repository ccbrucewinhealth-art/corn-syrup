import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface PingChartProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function PingChart({ model = {}, onChange, onSubmit }: PingChartProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('PingChart','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('PingChart','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/PingChart.vue" model={draft} onChange={update} fields={[{ name: 'monitorId', label: 'Monitor', type: 'text', textarea: false }, { name: 'period', label: 'Period', type: 'text', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
