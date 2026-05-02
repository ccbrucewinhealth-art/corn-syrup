import { Link, Outlet } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import MonitorList from '../components/MonitorList';

export default function Dashboard() {
  const { t } = useTranslation();
  return (
    <section className="dashboard-container" data-source="pages/Dashboard.vue">
      <div className="dashboard-row">
        <aside className="dashboard-monitor-column col-12 col-md-5 col-xl-4 ps-0">
          <Link to="/add" className="btn btn-primary mb-3 dashboard-add-monitor">
            + {t('Add New Monitor')}
          </Link>
          <MonitorList />
        </aside>
        <div className="dashboard-content-column">
          <Outlet />
        </div>
      </div>
    </section>
  );
}
