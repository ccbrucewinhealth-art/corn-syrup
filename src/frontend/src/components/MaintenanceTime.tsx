import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface MaintenanceTimeProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function MaintenanceTime({ model = {}, onChange, onSubmit }: MaintenanceTimeProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('MaintenanceTime','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('MaintenanceTime','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/MaintenanceTime.vue" model={draft} onChange={update} fields={[{ name: 'startDateTime', label: 'Start Date Time', type: 'datetime-local', textarea: false }, { name: 'endDateTime', label: 'End Date Time', type: 'datetime-local', textarea: false }, { name: 'timezone', label: 'Timezone', type: 'text', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
