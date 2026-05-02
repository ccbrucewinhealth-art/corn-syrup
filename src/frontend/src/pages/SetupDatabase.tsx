import { useTranslation } from 'react-i18next';

export default function SetupDatabase() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="pages/SetupDatabase.vue">
      <h2>SetupDatabase</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
