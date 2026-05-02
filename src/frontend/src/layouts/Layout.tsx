import { NavLink, Outlet, useLocation } from 'react-router-dom';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { debugAction } from '../lib/logger';

const logoUrl = '/icon.svg';

const primaryNavItems = [
  { to: '/manage-status-page', label: 'Status Pages', icon: '☰' },
  { to: '/dashboard', label: 'Dashboard', icon: '☯' },
];

const profileMenuItems = [
  { to: '/maintenance', label: 'Maintenance', icon: '🔧' },
  { to: '/settings/general', label: 'Settings', icon: '⚙' },
  { to: '/settings/about', label: 'Help', icon: 'ⓘ' },
];

export default function Layout() {
  const { t, i18n } = useTranslation();
  const location = useLocation();
  const [profileOpen, setProfileOpen] = useState(false);

  debugAction('Layout', 'render', { pathname: location.pathname, language: i18n.language });

  return (
    <div className="layout-shell" data-source="layouts/Layout.vue">
      <header className="desktop-header">
        <NavLink to="/dashboard" className="desktop-brand" aria-label={t('Dashboard')}>
          <object className="desktop-logo" width="40" height="40" data={logoUrl} aria-label={t('app.name')} />
          <span>{t('app.name')}</span>
        </NavLink>

        <nav className="nav primary-nav nav-pills" aria-label={t('Main Navigation')}>
          {primaryNavItems.map((item) => (
            <NavLink
              key={item.to}
              to={item.to}
              className={({ isActive }) => (isActive ? 'active' : undefined)}
            >
              <span aria-hidden="true" className="nav-icon">{item.icon}</span>
              {t(item.label)}
            </NavLink>
          ))}
        </nav>

        <div className="profile-menu-wrap">
          <button
            type="button"
            className="profile-trigger"
            aria-expanded={profileOpen}
            aria-label={t('User Menu')}
            onClick={() => setProfileOpen((open) => !open)}
          >
            <span className="profile-pic">B</span>
            <span className="profile-caret">⌄</span>
          </button>
          {profileOpen && (
            <div className="profile-dropdown" role="menu">
              <div className="dropdown-item-text">{t('signedInDisp', { username: 'bruce' })}</div>
              <div className="dropdown-divider" />
              {profileMenuItems.map((item) => (
                <NavLink key={item.to} to={item.to} className="dropdown-item" onClick={() => setProfileOpen(false)}>
                  <span aria-hidden="true" className="nav-icon">{item.icon}</span>
                  {t(item.label)}
                </NavLink>
              ))}
              <button type="button" className="dropdown-item logout-button" onClick={() => setProfileOpen(false)}>
                <span aria-hidden="true" className="nav-icon">⇱</span>
                {t('Logout')}
              </button>
              <div className="dropdown-divider" />
              <select
                className="form-select language-select"
                value={i18n.language}
                aria-label={t('Language')}
                onChange={(event) => i18n.changeLanguage(event.target.value)}
              >
                <option value="zh-TW">繁體中文</option>
                <option value="en">English</option>
                <option value="ja">日本語</option>
              </select>
            </div>
          )}
        </div>
      </header>

      <div className="layout-main main-system-content">
        <main className="page-content" aria-live="polite">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
