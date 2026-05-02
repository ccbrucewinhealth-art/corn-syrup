import { useTranslation } from 'react-i18next';

export default function HomeAssistant() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/HomeAssistant.vue">
      <h2>HomeAssistant</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
