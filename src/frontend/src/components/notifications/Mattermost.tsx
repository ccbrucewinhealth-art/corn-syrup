import { useTranslation } from 'react-i18next';

export default function Mattermost() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Mattermost.vue">
      <h2>Mattermost</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
