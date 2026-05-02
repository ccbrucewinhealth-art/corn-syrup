import { useTranslation } from 'react-i18next';

export default function GrafanaOncall() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/GrafanaOncall.vue">
      <h2>GrafanaOncall</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
