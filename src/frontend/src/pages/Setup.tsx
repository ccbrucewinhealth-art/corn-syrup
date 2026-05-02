import { useTranslation } from 'react-i18next';

export default function Setup() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="pages/Setup.vue">
      <h2>Setup</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
