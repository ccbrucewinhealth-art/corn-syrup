import { useTranslation } from 'react-i18next';

export default function SpugPush() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/SpugPush.vue">
      <h2>SpugPush</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
