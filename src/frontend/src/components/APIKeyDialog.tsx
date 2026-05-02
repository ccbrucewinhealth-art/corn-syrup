import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface APIKeyDialogProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function APIKeyDialog({ model = {}, onChange, onSubmit }: APIKeyDialogProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('APIKeyDialog','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('APIKeyDialog','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/APIKeyDialog.vue" model={draft} onChange={update} fields={[{ name: 'name', label: 'Name', type: 'text', textarea: false }, { name: 'expires', label: 'Expiry date', type: 'datetime-local', textarea: false }, { name: 'noExpire', label: "Don't expire", type: 'checkbox', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
