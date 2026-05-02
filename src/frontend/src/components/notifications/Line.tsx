import { useTranslation } from 'react-i18next';

export default function Line() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Line.vue">
      <h2>Line</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
