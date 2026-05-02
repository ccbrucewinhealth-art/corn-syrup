import { useTranslation } from 'react-i18next';

export default function SevenIO() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/SevenIO.vue">
      <h2>SevenIO</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
