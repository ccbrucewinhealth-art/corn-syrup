import { useTranslation } from 'react-i18next';

export default function YZJ() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/YZJ.vue">
      <h2>YZJ</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
