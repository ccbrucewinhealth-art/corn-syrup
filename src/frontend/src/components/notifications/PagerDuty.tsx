import { useTranslation } from 'react-i18next';

export default function PagerDuty() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/PagerDuty.vue">
      <h2>PagerDuty</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
