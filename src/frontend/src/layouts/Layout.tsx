import { NavLink, Outlet, useLocation } from 'react-router-dom';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { debugAction } from '../lib/logger';

const logoUrl = '/icon.svg';

const keroSidebarItems = [
  { to: '/dashboard', label: 'Dashboard', group: 'MENU', icon: 'fa-chart-line' },
  { to: '/manage-status-page', label: 'Status Pages', group: 'MENU', icon: 'fa-house-signal' },
  { to: '/maintenance', label: 'Maintenance', group: 'MENU', icon: 'fa-screwdriver-wrench' },
  { to: '/settings/general', label: 'General', group: 'SETTINGS', icon: 'fa-gear' },
  { to: '/settings/appearance', label: 'Appearance', group: 'SETTINGS', icon: 'fa-palette' },
  { to: '/settings/notifications', label: 'Notifications', group: 'SETTINGS', icon: 'fa-bell' },
  { to: '/settings/reverse-proxy', label: 'Reverse Proxy', group: 'SETTINGS', icon: 'fa-right-left' },
  { to: '/settings/tags', label: 'Tags', group: 'SETTINGS', icon: 'fa-tags' },
  { to: '/settings/monitor-history', label: 'Monitor History', group: 'SETTINGS', icon: 'fa-clock-rotate-left' },
  { to: '/settings/docker-hosts', label: 'Docker Hosts', group: 'SETTINGS', icon: 'fa-docker' },
  { to: '/settings/remote-browsers', label: 'Remote Browsers', group: 'SETTINGS', icon: 'fa-globe' },
  { to: '/settings/security', label: 'Security', group: 'SETTINGS', icon: 'fa-shield-halved' },
  { to: '/settings/api-keys', label: 'API Keys', group: 'SETTINGS', icon: 'fa-key' },
  { to: '/settings/proxies', label: 'Proxies', group: 'SETTINGS', icon: 'fa-network-wired' },
  { to: '/settings/about', label: 'About', group: 'SETTINGS', icon: 'fa-circle-info' },
];

const primaryNavItems = [
  { to: '/manage-status-page', label: 'Status Pages', icon: 'fa-house-signal' },
  { to: '/dashboard', label: 'Dashboard', icon: 'fa-chart-line' },
];

const profileMenuItems = [
  { to: '/maintenance', label: 'Maintenance', icon: 'fa-screwdriver-wrench' },
  { to: '/settings/about', label: 'Help', icon: 'fa-circle-info' },
];

export default function Layout() {
  const { t, i18n } = useTranslation();
  const location = useLocation();
  const [profileOpen, setProfileOpen] = useState(false);
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

  debugAction('Layout', 'render', { pathname: location.pathname, language: i18n.language });

  return (
    <div className={`kero-layout-shell${sidebarCollapsed ? ' sidebar-collapsed' : ''}`} data-source="layouts/Layout.vue" data-kero-template="analytics">
      <aside className={`kero-sidebar${sidebarCollapsed ? ' collapsed' : ''}`} aria-label={t('Main Navigation')}>
        <div className="kero-brand-row">
          <NavLink to="/dashboard" className="kero-brand" aria-label={t('Dashboard')}>
            <object className="desktop-logo" width="34" height="34" data={logoUrl} aria-label={t('app.name')} />
            {!sidebarCollapsed && <span>Corn-Syrup</span>}
          </NavLink>
          <button
            type="button"
            className="kero-menu-button"
            aria-label={t('Main Navigation')}
            onClick={() => setSidebarCollapsed((collapsed) => !collapsed)}
          >
            {sidebarCollapsed ? '▶' : '☰'}
          </button>
        </div>
        <nav className="kero-sidebar-nav">
          {['MENU', 'SETTINGS'].map((group) => (
            <div key={group} className="kero-nav-group">
              <div className="kero-nav-heading">{t(group)}</div>
              {keroSidebarItems.filter((item) => item.group === group).map((item) => (
                <NavLink key={item.to} to={item.to} className={({ isActive }) => `kero-nav-item${isActive ? ' active' : ''}`}>
                  <i className={`fa ${item.icon} kero-nav-icon`} aria-hidden="true" />
                  <span>{t(item.label)}</span>
                </NavLink>
              ))}
            </div>
          ))}
        </nav>
      </aside>

      <div className="kero-main-area">
        <header className="kero-topbar">
          <div>
            <h1 className="kero-page-title">{t('Analytics')}</h1>
            <p className="kero-page-subtitle">{t('Kero Template Subtitle')}</p>
          </div>
          <div className="kero-topbar-actions">
            <span className="kero-search">⌕ {t('Search...')}</span>
            <span className="kero-grid-icon">▦</span>
            <span className="kero-bell">♟</span>
            <div className="profile-menu-wrap">
              <button
                type="button"
                className="kero-profile-trigger"
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
                      <i className={`fa ${item.icon} nav-icon`} aria-hidden="true" />
                      {t(item.label)}
                    </NavLink>
                  ))}
                  <button type="button" className="dropdown-item logout-button" onClick={() => setProfileOpen(false)}>
                    <i className="fa fa-right-from-line nav-icon" aria-hidden="true" />
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
          </div>
        </header>

        <div className="kero-page-menu">
          <span className="kero-muted">☰ {t('Show page menu')}</span>
          <nav className="kero-tabs" aria-label={t('Main Navigation')}>
            {primaryNavItems.map((item) => (
              <NavLink key={item.to} to={item.to}>{t(item.label)}</NavLink>
            ))}
            <NavLink to="/settings/about">{t('More')}</NavLink>
          </nav>
          <span className="kero-muted">{t('Show right drawer')} ☰</span>
        </div>

        <section className="kero-portfolio-card" aria-label={t('Portfolio Performance')}>
          <div className="kero-card-header">
            <h2>{t('Portfolio Performance')}</h2>
            <button type="button" className="btn kero-view-all">{t('View All')}</button>
          </div>
          <div className="kero-metrics-row">
            <div className="kero-metric"><span className="kero-metric-icon yellow">▣</span><div><span>{t('Monitors')}</span><strong>1.7M</strong><small>↘ 54.1%</small></div></div>
            <div className="kero-metric"><span className="kero-metric-icon pink">♢</span><div><span>{t('Incidents')}</span><strong>9M</strong><small>↘ 14.1%</small></div></div>
            <div className="kero-metric"><span className="kero-metric-icon green">▥</span><div><span>{t('Uptime')}</span><strong>99.9%</strong><small>↗ 7.35%</small></div></div>
          </div>
        </section>

        <main className="kero-content-card" aria-live="polite">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
