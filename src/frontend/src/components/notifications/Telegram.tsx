import { useTranslation } from 'react-i18next';

export default function Telegram() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Telegram.vue">
      <h2>Telegram</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
