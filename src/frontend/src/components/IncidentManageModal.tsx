import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface IncidentManageModalProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function IncidentManageModal({ model = {}, onChange, onSubmit }: IncidentManageModalProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('IncidentManageModal','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('IncidentManageModal','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/IncidentManageModal.vue" model={draft} onChange={update} fields={[{ name: 'title', label: 'Title', type: 'text', textarea: false }, { name: 'status', label: 'Status', type: 'text', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
