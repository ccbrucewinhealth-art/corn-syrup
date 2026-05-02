import { useTranslation } from 'react-i18next';

export default function JiraServiceManagement() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/JiraServiceManagement.vue">
      <h2>JiraServiceManagement</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
