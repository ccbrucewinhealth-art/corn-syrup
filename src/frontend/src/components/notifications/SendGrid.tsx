import { useTranslation } from 'react-i18next';

export default function SendGrid() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/SendGrid.vue">
      <h2>SendGrid</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
