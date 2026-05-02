import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface MonitorListItemProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function MonitorListItem({ model = {}, onChange, onSubmit }: MonitorListItemProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('MonitorListItem','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('MonitorListItem','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/MonitorListItem.vue" model={draft} onChange={update} fields={[{ name: 'name', label: 'Name', type: 'text', textarea: false }, { name: 'url', label: 'URL', type: 'url', textarea: false }, { name: 'status', label: 'Status', type: 'text', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
