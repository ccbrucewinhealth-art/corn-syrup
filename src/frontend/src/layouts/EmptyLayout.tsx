import { Outlet, useLocation } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { debugAction } from '../lib/logger';

const logoUrl = '/icon.svg';

export default function EmptyLayout() {
  const { t, i18n } = useTranslation();
  const location = useLocation();

  debugAction('EmptyLayout', 'render', { pathname: location.pathname, language: i18n.language });

  return (
    <main className="empty-layout" data-source="layouts/EmptyLayout.vue">
      <div className="empty-layout-brand" aria-label={t('app.name')}>
        <img src={logoUrl} alt="" className="brand-icon" />
        <span>{t('app.name')}</span>
      </div>
      <section className="empty-layout-content">
        <Outlet />
      </section>
    </main>
  );
}
