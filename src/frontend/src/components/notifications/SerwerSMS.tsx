import { useTranslation } from 'react-i18next';

export default function SerwerSMS() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/SerwerSMS.vue">
      <h2>SerwerSMS</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
