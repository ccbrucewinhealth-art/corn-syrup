import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface MonitorListFilterProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function MonitorListFilter({ model = {}, onChange, onSubmit }: MonitorListFilterProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('MonitorListFilter','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('MonitorListFilter','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/MonitorListFilter.vue" model={draft} onChange={update} fields={[{ name: 'keyword', label: 'Search', type: 'text', textarea: false }, { name: 'status', label: 'Status', type: 'text', textarea: false }, { name: 'tag', label: 'Tag', type: 'text', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
