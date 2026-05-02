import { useTranslation } from 'react-i18next';

export default function NotFound() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="pages/NotFound.vue">
      <h2>NotFound</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
