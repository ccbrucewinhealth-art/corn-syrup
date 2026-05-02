import { useTranslation } from 'react-i18next';

export default function OneBot() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/OneBot.vue">
      <h2>OneBot</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
