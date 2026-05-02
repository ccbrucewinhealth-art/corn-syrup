import { Link } from 'react-router-dom';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { setByPath, type AnyRecord } from './common';
import { debugAction } from '../lib/logger';

export interface MonitorListProps { model?: AnyRecord; onChange?: (model: AnyRecord) => void; onSubmit?: (model: AnyRecord) => void; }
const sampleMonitors = [
  { id: 1, name: 'HTTP(s) Example', status: 'active', statusText: 'Up', url: 'https://example.com' },
  { id: 2, name: 'Ping Example', status: 'pending', statusText: 'Pending', url: 'ping.example.com' },
];

export default function MonitorList({ model = {}, onChange, onSubmit }: MonitorListProps) {
  const { t } = useTranslation();
  const [draft, setDraft] = useState<AnyRecord>(model);
  const update = (name: string, value: any) => {
    const next={ ...draft };
    setByPath(next, name, value);
    setDraft(next);
    debugAction('MonitorList','field.update',{name,value});
    onChange?.(next);
  };
  const submit = () => { debugAction('MonitorList','submit', draft); onSubmit?.(draft); };
  const keyword = String(draft.keyword ?? '').toLowerCase();
  const filteredMonitors = sampleMonitors.filter((monitor) => monitor.name.toLowerCase().includes(keyword));

  return (
    <div className="monitor-list-card shadow-box mb-3 p-0" data-source="components/MonitorList.vue">
      <div className="list-header">
        <div className="filter-row">
          <div className="search-wrapper">
            <form onSubmit={(event) => { event.preventDefault(); submit(); }}>
              <input
                className="form-control search-input"
                placeholder={t('Search...')}
                aria-label={t('Search monitored sites')}
                autoComplete="off"
                value={String(draft.keyword ?? '')}
                onChange={(event) => update('keyword', event.target.value)}
              />
            </form>
          </div>
          <div className="filters-group">
            <input className="form-check-input" type="checkbox" aria-label={t('Select all monitors')} />
            <button type="button" className="filter-dropdown-status">{t('Status')} ▾</button>
          </div>
        </div>
      </div>
      <div className="monitor-list px-2 scrollbar" data-testid="monitor-list">
        {filteredMonitors.length > 0 ? filteredMonitors.map((monitor) => (
          <Link key={monitor.id} to={`/dashboard/${monitor.id}`} className={`monitor-list-item item ${monitor.status}`}>
            <div className="left-part">
              <span className="circle" aria-hidden="true" />
              <span className="info">
                <span className="title">{monitor.name}</span>
                <span className="status">{t(monitor.statusText)} · {monitor.url}</span>
              </span>
            </div>
          </Link>
        )) : (
          <div className="text-center mt-3">
            {t('No Monitors, please')}{' '}<Link to="/add">{t('add one')}</Link>
          </div>
        )}
      </div>
    </div>
  );
}
