import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface EditMonitorConditionsProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function EditMonitorConditions({ model = {}, onChange, onSubmit }: EditMonitorConditionsProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('EditMonitorConditions','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('EditMonitorConditions','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/EditMonitorConditions.vue" model={draft} onChange={update} fields={[{ name: 'enabled', label: 'Enabled', type: 'checkbox', textarea: false }, { name: 'conditions', label: 'Conditions', type: 'text', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
