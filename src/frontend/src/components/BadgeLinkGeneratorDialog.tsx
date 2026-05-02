import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface BadgeLinkGeneratorDialogProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function BadgeLinkGeneratorDialog({ model = {}, onChange, onSubmit }: BadgeLinkGeneratorDialogProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('BadgeLinkGeneratorDialog','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('BadgeLinkGeneratorDialog','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/BadgeLinkGeneratorDialog.vue" model={draft} onChange={update} fields={[{ name: 'monitorId', label: 'Monitor', type: 'text', textarea: false }, { name: 'type', label: 'Type', type: 'text', textarea: false }, { name: 'duration', label: 'Duration', type: 'text', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
