import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface LoginProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function Login({ model = {}, onChange, onSubmit }: LoginProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('Login','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('Login','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/Login.vue" model={draft} onChange={update} fields={[{ name: 'username', label: 'Username', type: 'text', textarea: false }, { name: 'password', label: 'Password', type: 'password', textarea: false }, { name: 'token', label: '2FA Token', type: 'text', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
