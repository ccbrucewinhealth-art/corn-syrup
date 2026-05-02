import { useTranslation } from 'react-i18next';

export default function Gorush() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Gorush.vue">
      <h2>Gorush</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
