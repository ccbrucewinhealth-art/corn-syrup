import { useTranslation } from 'react-i18next';

export default function Notifery() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Notifery.vue">
      <h2>Notifery</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
