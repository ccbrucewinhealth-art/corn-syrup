import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { apiClient } from '../lib/api';
import Status from '../components/Status';
import Datetime from '../components/Datetime';

interface MonitorStats {
  up: number;
  down: number;
  maintenance: number;
  unknown: number;
  pause: number;
}

interface ImportantEvent {
  monitorID: number;
  name: string;
  status: number;
  time: string;
  msg: string;
}

export default function DashboardHome() {
  const { t } = useTranslation();
  const [stats, setStats] = useState<MonitorStats>({ up: 0, down: 0, maintenance: 0, unknown: 0, pause: 0 });
  const [events, setEvents] = useState<ImportantEvent[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    Promise.all([
      apiClient.getMonitors(),
      apiClient.getHeartbeats(0, 50),
    ])
      .then(([monitors, heartbeats]) => {
        const arr = Array.isArray(monitors) ? monitors : [];
        const s: MonitorStats = { up: 0, down: 0, maintenance: 0, unknown: 0, pause: 0 };
        arr.forEach((m: { active: boolean; maintenance: boolean }) => {
          if (!m.active) {
            s.pause++;
          } else if (m.maintenance) {
            s.maintenance++;
          } else {
            s.unknown++;
          }
        });
        setStats(s);
        
        const eb: ImportantEvent[] = [];
        if (Array.isArray(heartbeats)) {
          heartbeats.slice(0, 10).forEach((h: { monitor_id: number; name: string; status: number; time: string; msg: string }) => {
            if (h.status !== 1) {
              eb.push({
                monitorID: h.monitor_id,
                name: h.name,
                status: h.status,
                time: h.time,
                msg: h.msg,
              });
            }
          });
        }
        setEvents(eb);
      })
      .catch((err) => console.error('Load failed:', err))
      .finally(() => setLoading(false));
  }, []);

  return (
    <section className="dashboard-home" data-source="pages/DashboardHome.vue">
      <h1 className="mb-3">{t('Status Overview')}</h1>

      <div className="shadow-box big-padding text-center mb-3">
        <div className="stats-row">
          <div className="stat-col">
            <h3>{t('Up')}</h3>
            <span className="num text-muted-dark">{stats.up}</span>
          </div>
          <div className="stat-col">
            <h3>{t('Down')}</h3>
            <span className="num text-danger">{stats.down}</span>
          </div>
          <div className="stat-col">
            <h3>{t('Maintenance')}</h3>
            <span className="num bg-maintenance">{stats.maintenance}</span>
          </div>
          <div className="stat-col">
            <h3>{t('Unknown')}</h3>
            <span className="num text-muted-dark">{stats.unknown}</span>
          </div>
          <div className="stat-col">
            <h3>{t('pauseDashboardHome')}</h3>
            <span className="num text-muted-dark">{stats.pause}</span>
          </div>
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
            {events.map((beat) => (
              <tr key={beat.monitorID}>
                <td className="name-column">
                  <Link to={`/dashboard/${beat.monitorID}`}>{beat.name}</Link>
                </td>
                <td><Status status={beat.status} /></td>
                <td><Datetime value={beat.time} /></td>
                <td>{beat.msg}</td>
              </tr>
            ))}
            {events.length === 0 && !loading && (
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
