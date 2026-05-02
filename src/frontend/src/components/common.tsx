import type { ChangeEvent, ReactNode } from 'react';
import { useTranslation } from 'react-i18next';
import { debugAction } from '../lib/logger';

export type AnyRecord = Record<string, any>;

export function valueFromEvent(event: ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) {
  const target = event.target;
  if (target instanceof HTMLInputElement && target.type === 'checkbox') {
    return target.checked;
  }
  return target.value;
}

export function setByPath(object: AnyRecord, path: string, value: any) {
  // Vue v-model 可直接繫結深層欄位；React 版本以 path 拆解後建立必要節點，保留相同行為。
  const segments = path.split('.').filter(Boolean);
  let cursor = object;
  for (const segment of segments.slice(0, -1)) {
    if (!cursor[segment] || typeof cursor[segment] !== 'object') cursor[segment] = {};
    cursor = cursor[segment];
  }
  cursor[segments[segments.length - 1] || path] = value;
}

export function getByPath(object: AnyRecord | undefined, path: string, fallback = ''): any {
  return path.split('.').filter(Boolean).reduce<any>((cursor, segment) => cursor?.[segment], object) ?? fallback;
}

export interface FieldSpec {
  name: string;
  label: string;
  type?: string;
  placeholder?: string;
  required?: boolean;
  options?: Array<{ value: string | number; label: string }>;
  textarea?: boolean;
  help?: string;
}

export interface FormSectionProps {
  scope: string;
  model?: AnyRecord;
  onChange?: (name: string, value: any) => void;
  fields: FieldSpec[];
  children?: ReactNode;
}

export function FormSection({ scope, model = {}, onChange, fields, children }: FormSectionProps) {
  const { t } = useTranslation();
  const update = (name: string, value: any) => {
    debugAction(scope, 'field.update', { name, value });
    onChange?.(name, value);
  };
  return (
    <div className="card translated-component" data-source={scope}>
      {fields.map((field) => {
        const id = `${scope.replace(/[^a-z0-9]/gi, '-')}-${field.name}`;
        const value = getByPath(model, field.name, field.type === 'checkbox' ? undefined : '');
        return (
          <div className="mb-3" key={field.name}>
            <label className="form-label" htmlFor={id}>{t(field.label)}</label>
            {field.options ? (
              <select id={id} className="form-select" value={value} required={field.required} onChange={(event) => update(field.name, event.target.value)}>
                {field.options.map((option) => <option key={option.value} value={option.value}>{t(option.label)}</option>)}
              </select>
            ) : field.textarea ? (
              <textarea id={id} className="form-control" value={value} required={field.required} placeholder={field.placeholder ? t(field.placeholder) : undefined} onChange={(event) => update(field.name, event.target.value)} />
            ) : field.type === 'checkbox' ? (
              <input id={id} className="form-check-input d-block" type="checkbox" checked={Boolean(value)} onChange={(event) => update(field.name, event.target.checked)} />
            ) : (
              <input id={id} className="form-control" type={field.type || 'text'} value={value} required={field.required} placeholder={field.placeholder ? t(field.placeholder) : undefined} onChange={(event) => update(field.name, valueFromEvent(event))} />
            )}
            {field.help && <div className="form-text">{t(field.help)}</div>}
          </div>
        );
      })}
      {children}
    </div>
  );
}
