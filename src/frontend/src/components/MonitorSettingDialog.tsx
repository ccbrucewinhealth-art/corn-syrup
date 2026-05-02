import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface MonitorSettingDialogProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function MonitorSettingDialog({ model = {}, onChange, onSubmit }: MonitorSettingDialogProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('MonitorSettingDialog','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('MonitorSettingDialog','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/MonitorSettingDialog.vue" model={draft} onChange={update} fields={[{ name: 'interval', label: 'Interval', type: 'number', textarea: false }, { name: 'retryInterval', label: 'Retry Interval', type: 'number', textarea: false }, { name: 'maxretries', label: 'Retries', type: 'number', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
