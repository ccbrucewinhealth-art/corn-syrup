import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface TwoFADialogProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function TwoFADialog({ model = {}, onChange, onSubmit }: TwoFADialogProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('TwoFADialog','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('TwoFADialog','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/TwoFADialog.vue" model={draft} onChange={update} fields={[{ name: 'token', label: '2FA Token', type: 'text', textarea: false }, { name: 'enable', label: 'Enabled', type: 'checkbox', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
