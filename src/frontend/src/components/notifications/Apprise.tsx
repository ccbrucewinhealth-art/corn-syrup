import { useTranslation } from 'react-i18next';

export default function Apprise() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Apprise.vue">
      <h2>Apprise</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
