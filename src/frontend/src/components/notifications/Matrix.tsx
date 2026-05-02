import { useTranslation } from 'react-i18next';

export default function Matrix() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Matrix.vue">
      <h2>Matrix</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
