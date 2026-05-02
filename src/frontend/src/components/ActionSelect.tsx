import { debugAction } from '../lib/logger';
export interface ActionSelectOption { value: string; label: string; disabled?: boolean; }
export interface ActionSelectProps { modelValue?: string; options?: ActionSelectOption[]; action?: () => void; actionLabel?: string; onUpdateModelValue?: (value: string) => void; }
export default function ActionSelect({ modelValue = '', options = [], action = () => {}, actionLabel = '+', onUpdateModelValue }: ActionSelectProps) {
  return <div className="input-group mb-3" data-source="components/ActionSelect.vue"><select className="form-select" value={modelValue} onChange={(e) => { debugAction('ActionSelect','select.change', e.target.value); onUpdateModelValue?.(e.target.value); }}>{options.map(o => <option key={o.value} value={o.value} disabled={o.disabled}>{o.label}</option>)}</select><button type="button" className="btn btn-outline-primary" onClick={() => { debugAction('ActionSelect','action.click'); action(); }}>{actionLabel}</button></div>;
}
