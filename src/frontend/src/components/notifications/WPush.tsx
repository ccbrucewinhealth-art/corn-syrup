import { useTranslation } from 'react-i18next';

export default function WPush() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/WPush.vue">
      <h2>WPush</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
