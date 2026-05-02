import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface IncidentHistoryProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function IncidentHistory({ model = {}, onChange, onSubmit }: IncidentHistoryProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('IncidentHistory','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('IncidentHistory','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/IncidentHistory.vue" model={draft} onChange={update} fields={[{ name: 'keyword', label: 'Search', type: 'text', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
