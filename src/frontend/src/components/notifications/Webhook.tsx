import { useTranslation } from 'react-i18next';

export default function Webhook() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Webhook.vue">
      <h2>Webhook</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
