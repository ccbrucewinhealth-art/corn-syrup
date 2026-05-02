import { useTranslation } from 'react-i18next';

export default function Opsgenie() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Opsgenie.vue">
      <h2>Opsgenie</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
