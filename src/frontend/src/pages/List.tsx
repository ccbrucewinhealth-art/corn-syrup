import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { apiClient } from '../lib/api';
import Status from '../components/Status';
import { Link } from 'react-router-dom';

interface Monitor {
  id: number;
  name: string;
  url: string;
  type: string;
  active: boolean;
  accept_status: number[];
}
export default function List() {
  const { t } = useTranslation();
  const [monitors, setMonitors] = useState<Monitor[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    apiClient.getMonitors()
      .then((data) => setMonitors(Array.isArray(data) ? data : []))
      .catch((err) => console.error('Load monitors failed:', err))
      .finally(() => setLoading(false));
  }, []);

  return (
    <section className="card" data-source="pages/List.vue">
      <h2>{t('Monitor List')}</h2>
      {loading ? (
        <p>{t('Loading...')}</p>
      ) : monitors.length === 0 ? (
        <p>{t('No monitors')}</p>
      ) : (
        <div className="table-responsive">
          <table className="table table-hover">
            <thead>
              <tr>
                <th>{t('Name')}</th>
                <th>{t('URL')}</th>
                <th>{t('Type')}</th>
                <th>{t('Status')}</th>
                <th>{t('Actions')}</th>
              </tr>
            </thead>
            <tbody>
              {monitors.map((m) => (
                <tr key={m.id}>
                  <td>{m.name}</td>
                  <td className="url">{m.url}</td>
                  <td>{m.type}</td>
                  <td><Status status={m.active ? 1 : 0} /></td>
                  <td>
                    <div className="btn-group">
                      <Link to={`/edit/${m.id}`} className="btn btn-normal">{t('Edit')}</Link>
                      <Link to={`/clone/${m.id}`} className="btn btn-normal">{t('Clone')}</Link>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </section>
  );
}
