import { useTranslation } from 'react-i18next';

export default function Gotify() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Gotify.vue">
      <h2>Gotify</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
