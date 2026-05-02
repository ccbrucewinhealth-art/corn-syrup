import { useTranslation } from 'react-i18next';

export default function SMSPlanet() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/SMSPlanet.vue">
      <h2>SMSPlanet</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
