import { useTranslation } from 'react-i18next';

export default function CallMeBot() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/CallMeBot.vue">
      <h2>CallMeBot</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
