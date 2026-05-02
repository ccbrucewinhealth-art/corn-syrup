import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface MonitorListFilterDropdownProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function MonitorListFilterDropdown({ model = {}, onChange, onSubmit }: MonitorListFilterDropdownProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('MonitorListFilterDropdown','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('MonitorListFilterDropdown','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/MonitorListFilterDropdown.vue" model={draft} onChange={update} fields={[{ name: 'status', label: 'Status', type: 'text', textarea: false }, { name: 'active', label: 'Active', type: 'checkbox', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
