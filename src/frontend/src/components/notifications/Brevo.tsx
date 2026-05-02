import { useTranslation } from 'react-i18next';

export default function Brevo() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Brevo.vue">
      <h2>Brevo</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
