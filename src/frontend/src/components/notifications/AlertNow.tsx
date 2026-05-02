import { useTranslation } from 'react-i18next';

export default function AlertNow() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/AlertNow.vue">
      <h2>AlertNow</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
