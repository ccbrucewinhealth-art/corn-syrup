import { Link } from 'react-router-dom';
import { useTranslation } from 'react-i18next';

export default function ManageStatusPage() {
  const { t } = useTranslation();
  return (
    <section className="status-page-management" data-source="pages/ManageStatusPage.vue">
      <h1 className="mb-3">{t('Status Pages')}</h1>
      <div>
        <Link to="/add-status-page" className="btn btn-primary mb-3">+ {t('New Status Page')}</Link>
      </div>
      <div className="shadow-box status-page-empty-box">
        <span className="d-flex align-items-center justify-content-center my-3">{t('No Status Pages')}</span>
      </div>
    </section>
  );
}
