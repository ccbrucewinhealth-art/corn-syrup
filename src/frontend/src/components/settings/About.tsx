import { useTranslation } from 'react-i18next';

export default function About() {
  const { t } = useTranslation();
  return (
    <section className="settings-panel about-page" data-source="components/settings/About.vue">
      <div className="settings-panel-header">{t('About')}</div>
      <p>{t('Help Description')}</p>
      <div className="help-link-list">
        <a href="https://github.com/louislam/uptime-kuma/wiki" target="_blank" rel="noreferrer">
          {t('Uptime Kuma Wiki')}
        </a>
        <a href="https://github.com/louislam/uptime-kuma" target="_blank" rel="noreferrer">
          {t('Uptime Kuma Repository')}
        </a>
      </div>
    </section>
  );
}
