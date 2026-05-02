import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { FormSection, setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface DockerHostDialogProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
export default function DockerHostDialog({ model = {}, onChange, onSubmit }: DockerHostDialogProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => { const next={ ...draft }; setByPath(next, name, value); setDraft(next); debugAction('DockerHostDialog','field.update',{name,value}); onChange?.(next); };
  const submit = () => { debugAction('DockerHostDialog','submit', draft); onSubmit?.(draft); };
  return <FormSection scope="components/DockerHostDialog.vue" model={draft} onChange={update} fields={[{ name: 'name', label: 'Name', type: 'text', textarea: false }, { name: 'dockerDaemon', label: 'Docker Daemon', type: 'text', textarea: false }, { name: 'dockerType', label: 'Connection Type', type: 'text', textarea: false }]}><button type="button" className="btn btn-primary" onClick={submit}>{t('Save')}</button></FormSection>;
}
