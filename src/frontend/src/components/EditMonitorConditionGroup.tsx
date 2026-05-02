import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface EditMonitorConditionGroupProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function EditMonitorConditionGroup({ model = {}, onChange, onSubmit }: EditMonitorConditionGroupProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('EditMonitorConditionGroup','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('EditMonitorConditionGroup','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/EditMonitorConditionGroup.vue" model={draft} onChange={update} fields={[{ name: 'operator', label: 'Condition Operator', type: 'text', textarea: false }, { name: 'children', label: 'Conditions', type: 'text', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
