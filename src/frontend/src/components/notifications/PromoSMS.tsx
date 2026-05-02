import { useTranslation } from 'react-i18next';

export default function PromoSMS() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/PromoSMS.vue">
      <h2>PromoSMS</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
