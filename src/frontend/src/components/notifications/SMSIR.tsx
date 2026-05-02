import { useTranslation } from 'react-i18next';

export default function SMSIR() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/SMSIR.vue">
      <h2>SMSIR</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
