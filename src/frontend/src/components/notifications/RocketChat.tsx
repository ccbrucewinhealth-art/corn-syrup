import { useTranslation } from 'react-i18next';

export default function RocketChat() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/RocketChat.vue">
      <h2>RocketChat</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
