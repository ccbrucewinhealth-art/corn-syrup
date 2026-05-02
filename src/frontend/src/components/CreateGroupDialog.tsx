import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface CreateGroupDialogProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function CreateGroupDialog({ model = {}, onChange, onSubmit }: CreateGroupDialogProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('CreateGroupDialog','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('CreateGroupDialog','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/CreateGroupDialog.vue" model={draft} onChange={update} fields={[{ name: 'name', label: 'Name', type: 'text', textarea: false }, { name: 'weight', label: 'Weight', type: 'number', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
