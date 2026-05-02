import { useTranslation } from 'react-i18next';

export default function SMSPartner() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/SMSPartner.vue">
      <h2>SMSPartner</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
