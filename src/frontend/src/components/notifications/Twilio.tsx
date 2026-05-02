import { useTranslation } from 'react-i18next';

export default function Twilio() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Twilio.vue">
      <h2>Twilio</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
