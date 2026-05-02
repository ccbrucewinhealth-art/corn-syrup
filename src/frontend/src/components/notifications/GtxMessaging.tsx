import { useTranslation } from 'react-i18next';

export default function GtxMessaging() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/GtxMessaging.vue">
      <h2>GtxMessaging</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
