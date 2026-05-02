import { useTranslation } from 'react-i18next';

export default function ZohoCliq() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/ZohoCliq.vue">
      <h2>ZohoCliq</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
