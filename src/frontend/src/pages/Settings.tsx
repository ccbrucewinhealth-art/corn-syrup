import { NavLink, Outlet, useLocation } from 'react-router-dom';
import { useTranslation } from 'react-i18next';

const settingMenus = [
  { key: 'general', title: 'General' },
  { key: 'appearance', title: 'Appearance' },
  { key: 'notifications', title: 'Notifications' },
  { key: 'reverse-proxy', title: 'Reverse Proxy' },
  { key: 'tags', title: 'Tags' },
  { key: 'monitor-history', title: 'Monitor History' },
  { key: 'docker-hosts', title: 'Docker Hosts' },
  { key: 'remote-browsers', title: 'Remote Browsers' },
  { key: 'security', title: 'Security' },
  { key: 'api-keys', title: 'API Keys' },
  { key: 'proxies', title: 'Proxies' },
  { key: 'about', title: 'About' },
];

export default function Settings() {
  const { t } = useTranslation();
  const location = useLocation();
  const currentKey = location.pathname.split('/').filter(Boolean).at(-1);
  const currentMenu = settingMenus.find((menu) => menu.key === currentKey);

  return (
    <section className="settings-page" data-source="pages/Settings.vue">
      <h1 className="mb-3">{t('Settings')}</h1>
      <div className="card settings-shell">
        <nav className="settings-menu" aria-label={t('Settings Menu')}>
          {settingMenus.map((menu) => (
            <NavLink key={menu.key} to={`/settings/${menu.key}`} className="settings-menu-item">
              {t(menu.title)}
            </NavLink>
          ))}
        </nav>
        <div className="settings-content">
          {currentMenu && <div className="settings-content-header">{t(currentMenu.title)}</div>}
          <div className="settings-content-body">
            <Outlet />
          </div>
        </div>
      </div>
    </section>
  );
}
