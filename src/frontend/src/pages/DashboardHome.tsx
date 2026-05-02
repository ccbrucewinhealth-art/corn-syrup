import { Link } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import Status from '../components/Status';
import Datetime from '../components/Datetime';

const stats = [
  { key: 'up', label: 'Up', value: 0, className: 'text-muted-dark' },
  { key: 'down', label: 'Down', value: 0, className: 'text-muted-dark' },
  { key: 'maintenance', label: 'Maintenance', value: 0, className: 'text-muted-dark' },
  { key: 'unknown', label: 'Unknown', value: 0, className: 'text-muted-dark' },
  { key: 'pause', label: 'pauseDashboardHome', value: 0, className: 'text-muted-dark' },
];

const importantEvents: Array<{ monitorID: number; name: string; status: number; time: string; msg: string }> = [];

export default function DashboardHome() {
  const { t } = useTranslation();
  return (
    <section className="dashboard-home" data-source="pages/DashboardHome.vue">
      <h1 className="mb-3">{t('Status Overview')}</h1>

      <div className="shadow-box big-padding text-center mb-3">
        <div className="stats-row">
          {stats.map((item) => (
            <div key={item.key} className="stat-col">
              <h3>{t(item.label)}</h3>
              <span className={`num ${item.className}`}>{item.value}</span>
            </div>
          ))}
        </div>
      </div>

      <div className="shadow-box table-shadow-box table-wrapper">
        <div className="mb-3 text-end">
          <button type="button" className="btn btn-sm btn-outline-danger">
            {t('Clear All Events')}
          </button>
        </div>
        <table className="table table-borderless table-hover dashboard-events-table">
          <thead>
            <tr>
              <th className="name-column">{t('Name')}</th>
              <th>{t('Status')}</th>
              <th>{t('DateTime')}</th>
              <th>{t('Message')}</th>
            </tr>
          </thead>
          <tbody>
            {importantEvents.map((beat) => (
              <tr key={beat.monitorID}>
                <td className="name-column">
                  <Link to={`/dashboard/${beat.monitorID}`}>{beat.name}</Link>
                </td>
                <td><Status status={beat.status} /></td>
                <td><Datetime value={beat.time} /></td>
                <td>{beat.msg}</td>
              </tr>
            ))}
            {importantEvents.length === 0 && (
              <tr>
                <td colSpan={4}>{t('No important events')}</td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </section>
  );
}
