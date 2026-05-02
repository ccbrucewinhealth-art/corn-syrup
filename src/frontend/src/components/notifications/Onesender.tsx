import { useTranslation } from 'react-i18next';

export default function Onesender() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Onesender.vue">
      <h2>Onesender</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
