import { useTranslation } from 'react-i18next';

export default function GoAlert() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/GoAlert.vue">
      <h2>GoAlert</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
