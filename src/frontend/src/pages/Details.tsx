import { Link, useParams } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import Status from '../components/Status';
import HeartbeatBar from '../components/HeartbeatBar';

const fallbackMonitor = {
  id: '1',
  name: 'HTTP(s) Example',
  type: 'http',
  url: 'https://example.com',
  interval: 60,
  status: 1,
  active: true,
};

export default function Details() {
  const { t } = useTranslation();
  const { id } = useParams();
  const monitor = { ...fallbackMonitor, id: id ?? fallbackMonitor.id };

  return (
    <section className="monitor-details" data-source="pages/Details.vue">
      <h1>
        {monitor.name}
        <span className="monitor-id"><span className="hash">#</span>{monitor.id}</span>
      </h1>

      <p className="url">
        <a href={monitor.url} target="_blank" rel="noopener noreferrer">{monitor.url}</a>
      </p>

      <div className="functions mb-3">
        <div className="btn-group" role="group">
          <button type="button" className="btn btn-normal">{t('Pause')}</button>
          <Link to={`/edit/${monitor.id}`} className="btn btn-normal">{t('Edit')}</Link>
          <Link to={`/clone/${monitor.id}`} className="btn btn-normal">{t('Clone')}</Link>
          <button type="button" className="btn btn-normal text-danger">{t('Delete')}</button>
        </div>
      </div>

      <div className="shadow-box monitor-summary-box">
        <div className="monitor-summary-main">
          <HeartbeatBar heartbeats={[{ status: monitor.status, ping: 42, time: new Date().toISOString() }]} />
          <span className="word">{t('checkEverySecond', { seconds: monitor.interval })}</span>
        </div>
        <div className="monitor-summary-status">
          <Status status={monitor.status} />
        </div>
      </div>
    </section>
  );
}
