import type { ChangeEventHandler, HTMLInputTypeAttribute } from 'react';
import { debugAction } from '../lib/logger';

export interface ActionInputProps { modelValue?: string; enabled?: boolean; placeholder?: string; icon: string; type?: HTMLInputTypeAttribute; action?: () => void; actionAriaLabel: string; onUpdateModelValue?: (value: string) => void; }
export default function ActionInput({ modelValue = '', enabled = true, placeholder = '', icon, type = 'text', action = () => {}, actionAriaLabel, onUpdateModelValue }: ActionInputProps) {
  const onChange: ChangeEventHandler<HTMLInputElement> = (event) => { debugAction('ActionInput', 'model.update', event.target.value); onUpdateModelValue?.(event.target.value); };
  const onAction = () => { debugAction('ActionInput', 'action.click', { icon }); action(); };
  return <div className="input-group mb-3" data-source="components/ActionInput.vue"><input className="form-control" value={modelValue} type={type} placeholder={placeholder} disabled={!enabled} onChange={onChange} /><button type="button" className="btn btn-outline-primary" aria-label={actionAriaLabel} onClick={onAction}>{icon}</button></div>;
}
