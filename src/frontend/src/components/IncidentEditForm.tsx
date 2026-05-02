import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface IncidentEditFormProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function IncidentEditForm({ model = {}, onChange, onSubmit }: IncidentEditFormProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('IncidentEditForm','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('IncidentEditForm','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/IncidentEditForm.vue" model={draft} onChange={update} fields={[{ name: 'title', label: 'Title', type: 'text', textarea: false }, { name: 'content', label: 'Content', type: 'text', textarea: true }, { name: 'style', label: 'Style', type: 'text', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
