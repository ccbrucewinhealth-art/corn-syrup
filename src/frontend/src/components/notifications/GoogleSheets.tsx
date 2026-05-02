import { useTranslation } from 'react-i18next';

export default function GoogleSheets() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/GoogleSheets.vue">
      <h2>GoogleSheets</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
