import { useTranslation } from 'react-i18next';

export default function Slack() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Slack.vue">
      <h2>Slack</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
