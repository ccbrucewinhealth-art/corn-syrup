import { useTranslation } from 'react-i18next';

export interface FieldSpec {
  label: string;
  type?: 'text' | 'password' | 'number' | 'select' | 'radio' | 'checkbox' | 'textarea' | 'button' | 'paragraph';
  value?: string;
  options?: string[];
  help?: string;
  checked?: boolean;
}

export interface SettingsSectionProps {
  source: string;
  title: string;
  fields: FieldSpec[];
  actions?: string[];
}

export default function SettingsSection({ source, title, fields, actions = ['Save'] }: SettingsSectionProps) {
  const { t } = useTranslation();

  return (
    <section className="settings-panel" data-source={source}>
      <div className="settings-panel-header">{t(title)}</div>
      <div className="settings-form-body">
        {fields.map((field, index) => (
          <div key={`${field.label}-${index}`} className={field.type === 'checkbox' ? 'my-3 form-check' : 'my-3'}>
            {field.type === 'paragraph' ? (
              <p>{t(field.label)}</p>
            ) : field.type === 'button' ? (
              <button type="button" className="btn btn-primary me-2">{t(field.label)}</button>
            ) : field.type === 'checkbox' ? (
              <>
                <input id={`${source}-${index}`} className="form-check-input" type="checkbox" defaultChecked={field.checked} />
                <label className="form-check-label" htmlFor={`${source}-${index}`}>{t(field.label)}</label>
                {field.help && <div className="form-text">{t(field.help)}</div>}
              </>
            ) : (
              <>
                <label className="form-label" htmlFor={`${source}-${index}`}>{t(field.label)}</label>
                {field.type === 'select' ? (
                  <select id={`${source}-${index}`} className="form-select" defaultValue={field.value ?? field.options?.[0]}>
                    {(field.options ?? []).map((option) => <option key={option} value={option}>{t(option)}</option>)}
                  </select>
                ) : field.type === 'radio' ? (
                  <div className="settings-radio-group">
                    {(field.options ?? []).map((option) => (
                      <label key={option} className="settings-radio-option">
                        <input type="radio" name={`${source}-${index}`} defaultChecked={option === field.value} />
                        {t(option)}
                      </label>
                    ))}
                  </div>
                ) : field.type === 'textarea' ? (
                  <textarea id={`${source}-${index}`} className="form-control code-textarea" defaultValue={field.value ?? ''} />
                ) : (
                  <input id={`${source}-${index}`} className="form-control" type={field.type ?? 'text'} defaultValue={field.value ?? ''} />
                )}
                {field.help && <div className="form-text">{t(field.help)}</div>}
              </>
            )}
          </div>
        ))}
      </div>
      <div className="settings-actions">
        {actions.map((action) => <button key={action} type="button" className="btn btn-primary">{t(action)}</button>)}
      </div>
    </section>
  );
}
