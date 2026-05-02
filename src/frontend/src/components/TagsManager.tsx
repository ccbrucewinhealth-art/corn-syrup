import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface TagsManagerProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function TagsManager({ model = {}, onChange, onSubmit }: TagsManagerProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('TagsManager','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('TagsManager','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/TagsManager.vue" model={draft} onChange={update} fields={[{ name: 'name', label: 'Name', type: 'text', textarea: false }, { name: 'value', label: 'Value', type: 'text', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
