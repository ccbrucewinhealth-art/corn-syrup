import { useTranslation } from 'react-i18next';

export default function GoogleChat() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/GoogleChat.vue">
      <h2>GoogleChat</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
