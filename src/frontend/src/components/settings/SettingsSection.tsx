import { useState, useEffect, useRef } from 'react';
import { useTranslation } from 'react-i18next';

export interface FieldSpec {
  label: string;
  type?: 'text' | 'password' | 'number' | 'select' | 'radio' | 'checkbox' | 'textarea' | 'button' | 'paragraph';
  value?: string;
  options?: string[];
  help?: string;
  checked?: boolean;
  key?: string;
}

export interface SettingsSectionProps {
  source: string;
  title: string;
  fields: FieldSpec[];
  actions?: string[];
  onLoad?: () => Promise<Record<string, string>>;
  onSave?: (data: Record<string, string>) => Promise<void>;
}

export default function SettingsSection({ source, title, fields, actions = ['Save'], onLoad, onSave }: SettingsSectionProps) {
  const { t } = useTranslation();
  const [formData, setFormData] = useState<Record<string, string>>({});
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [message, setMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);
  const formRefs = useRef<Map<string, HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>>(new Map());

  useEffect(() => {
    if (onLoad) {
      handleLoad();
    }
  }, [onLoad]);

  const handleLoad = async () => {
    if (!onLoad) return;
    setLoading(true);
    setMessage(null);
    try {
      const data = await onLoad();
      setFormData(data);
      data && Object.entries(data).forEach(([key, value]) => {
        const input = formRefs.current.get(key);
        if (input) {
          if (input.type === 'radio') {
            const radio = document.querySelector(`input[name="${key}"][value="${value}"]`) as HTMLInputElement;
            if (radio) radio.checked = true;
          } else if ((input as HTMLInputElement).type === 'checkbox') {
            const strValue = String(value);
            (input as HTMLInputElement).checked = strValue === 'true';
          } else {
            input.value = value;
          }
        }
      });
    } catch (err) {
      setMessage({ type: 'error', text: t('Failed to load settings') });
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    if (!onSave) return;
    setSaving(true);
    setMessage(null);
    try {
      const data: Record<string, string> = {};
      formRefs.current.forEach((input, key) => {
        if (input.type === 'radio') {
          const checkedRadio = document.querySelector(`input[name="${key}"]:checked`) as HTMLInputElement;
          if (checkedRadio) data[key] = checkedRadio.value;
        } else if (input.type === 'checkbox') {
          data[key] = (input as HTMLInputElement).checked ? 'true' : 'false';
        } else {
          data[key] = input.value;
        }
      });
      await onSave(data);
      setMessage({ type: 'success', text: t('Settings saved successfully') });
    } catch (err) {
      setMessage({ type: 'error', text: t('Failed to save settings') });
    } finally {
      setSaving(false);
    }
  };

  const registerRef = (key: string) => (el: HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement | null) => {
    if (el) formRefs.current.set(key, el);
  };

  const actionHandlers: Record<string, () => void> = {
    Load: handleLoad,
    Save: handleSave,
  };

  return (
    <section className="settings-panel" data-source={source}>
      <div className="settings-panel-header">{t(title)}</div>
      {loading && <div className="alert alert-info">{t('Loading...')}</div>}
      {message && <div className={`alert alert-${message.type}`}>{message.text}</div>}
      <div className="settings-form-body">
        {fields.map((field, index) => {
          const fieldKey = field.key || `${source}-${index}`;
          return (
            <div key={fieldKey} className={field.type === 'checkbox' ? 'my-3 form-check' : 'my-3'}>
              {field.type === 'paragraph' ? (
                <p>{t(field.label)}</p>
              ) : field.type === 'button' ? (
                <button type="button" className="btn btn-primary me-2">{t(field.label)}</button>
              ) : field.type === 'checkbox' ? (
                <>
                  <input
                    id={fieldKey}
                    ref={registerRef(fieldKey)}
                    className="form-check-input"
                    type="checkbox"
                    defaultChecked={field.checked}
                  />
                  <label className="form-check-label" htmlFor={fieldKey}>{t(field.label)}</label>
                  {field.help && <div className="form-text">{t(field.help)}</div>}
                </>
              ) : (
                <>
                  <label className="form-label" htmlFor={fieldKey}>{t(field.label)}</label>
                  {field.type === 'select' ? (
                    <select
                      id={fieldKey}
                      ref={registerRef(fieldKey)}
                      className="form-select"
                      defaultValue={field.value ?? field.options?.[0]}
                    >
                      {(field.options ?? []).map((option) => <option key={option} value={option}>{t(option)}</option>)}
                    </select>
                  ) : field.type === 'radio' ? (
                    <div className="settings-radio-group">
                      {(field.options ?? []).map((option) => (
                        <label key={option} className="settings-radio-option">
                          <input
                            type="radio"
                            name={fieldKey}
                            ref={registerRef(fieldKey)}
                            defaultChecked={option === field.value}
                            value={option}
                          />
                          {t(option)}
                        </label>
                      ))}
                    </div>
                  ) : field.type === 'textarea' ? (
                    <textarea
                      id={fieldKey}
                      ref={registerRef(fieldKey)}
                      className="form-control code-textarea"
                      defaultValue={field.value ?? ''}
                    />
                  ) : (
                    <input
                      id={fieldKey}
                      ref={registerRef(fieldKey)}
                      className="form-control"
                      type={field.type ?? 'text'}
                      defaultValue={field.value ?? ''}
                    />
                  )}
                  {field.help && <div className="form-text">{t(field.help)}</div>}
                </>
              )}
            </div>
          );
        })}
      </div>
      <div className="settings-actions">
        {actions.map((action) => (
          <button
            key={action}
            type="button"
            className="btn btn-primary"
            onClick={actionHandlers[action]}
            disabled={saving || (loading && action === 'Save')}
          >
            {saving ? t('Saving...') : t(action)}
          </button>
        ))}
      </div>
    </section>
  );
}
