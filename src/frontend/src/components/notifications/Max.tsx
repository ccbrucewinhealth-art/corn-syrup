import { useTranslation } from 'react-i18next';

export default function Max() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Max.vue">
      <h2>Max</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
