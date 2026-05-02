import { useTranslation } from 'react-i18next';

export default function SMSManager() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/SMSManager.vue">
      <h2>SMSManager</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
