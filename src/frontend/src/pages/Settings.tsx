import { Outlet, useLocation } from 'react-router-dom';
import { useTranslation } from 'react-i18next';

export default function Settings() {
  const { t } = useTranslation();
  const location = useLocation();

  return (
    <section className="settings-page" data-source="pages/Settings.vue">
      <div className="card settings-shell">
        <div className="settings-content">
          <div className="settings-content-body">
            <Outlet />
          </div>
        </div>
      </div>
    </section>
  );
}
