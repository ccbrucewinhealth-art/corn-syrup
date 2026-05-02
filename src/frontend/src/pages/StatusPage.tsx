import { useTranslation } from 'react-i18next';

export default function StatusPage() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="pages/StatusPage.vue">
      <h2>StatusPage</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
