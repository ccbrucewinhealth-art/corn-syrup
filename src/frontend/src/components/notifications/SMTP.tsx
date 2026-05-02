import { useTranslation } from 'react-i18next';

export default function SMTP() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/SMTP.vue">
      <h2>SMTP</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
