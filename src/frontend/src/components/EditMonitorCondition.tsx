import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface EditMonitorConditionProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function EditMonitorCondition({ model = {}, onChange, onSubmit }: EditMonitorConditionProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('EditMonitorCondition','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('EditMonitorCondition','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/EditMonitorCondition.vue" model={draft} onChange={update} fields={[{ name: 'variable', label: 'Variable', type: 'text', textarea: false }, { name: 'operator', label: 'Operator', type: 'text', textarea: false }, { name: 'value', label: 'Value', type: 'text', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
