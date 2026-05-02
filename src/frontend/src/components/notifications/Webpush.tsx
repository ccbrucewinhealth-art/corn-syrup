import { useTranslation } from 'react-i18next';

export default function Webpush() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Webpush.vue">
      <h2>Webpush</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
