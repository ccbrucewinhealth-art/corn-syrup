import { Link } from 'react-router-dom';
import { useTranslation } from 'react-i18next';

export default function ManageMaintenance() {
  const { t } = useTranslation();
  return (
    <section className="management-page" data-source="pages/ManageMaintenance.vue">
      <div className="page-title-row">
        <h1>{t('Maintenance')}</h1>
        <Link to="/add-maintenance" className="btn btn-primary">+ {t('Add Maintenance')}</Link>
      </div>
      <div className="shadow-box table-shadow-box">
        <table className="table table-borderless table-hover">
          <thead>
            <tr>
              <th>{t('Name')}</th>
              <th>{t('Status')}</th>
              <th>{t('DateTime')}</th>
              <th>{t('Actions')}</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>{t('No maintenance configured')}</td>
              <td><span className="badge bg-secondary">{t('Unknown')}</span></td>
              <td>—</td>
              <td><Link to="/add-maintenance" className="btn btn-normal">{t('Add')}</Link></td>
            </tr>
          </tbody>
        </table>
      </div>
    </section>
  );
}
