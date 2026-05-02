import { useTranslation } from 'react-i18next';

export default function SIGNL4() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/SIGNL4.vue">
      <h2>SIGNL4</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
