import { useTranslation } from 'react-i18next';

export default function SMSEagle() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/SMSEagle.vue">
      <h2>SMSEagle</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
