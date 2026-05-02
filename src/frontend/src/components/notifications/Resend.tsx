import { useTranslation } from 'react-i18next';

export default function Resend() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Resend.vue">
      <h2>Resend</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
