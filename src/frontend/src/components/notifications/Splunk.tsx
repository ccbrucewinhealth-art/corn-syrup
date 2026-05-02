import { useTranslation } from 'react-i18next';

export default function Splunk() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Splunk.vue">
      <h2>Splunk</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
