import { useTranslation } from 'react-i18next';

export default function WAHA() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/WAHA.vue">
      <h2>WAHA</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
