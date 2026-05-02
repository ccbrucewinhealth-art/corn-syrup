import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface ProxyDialogProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function ProxyDialog({ model = {}, onChange, onSubmit }: ProxyDialogProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('ProxyDialog','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('ProxyDialog','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/ProxyDialog.vue" model={draft} onChange={update} fields={[{ name: 'host', label: 'Host', type: 'text', textarea: false }, { name: 'port', label: 'Port', type: 'number', textarea: false }, { name: 'auth', label: 'Authentication', type: 'checkbox', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
