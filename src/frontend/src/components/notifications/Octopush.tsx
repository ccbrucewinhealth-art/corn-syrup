import { useTranslation } from 'react-i18next';

export default function Octopush() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Octopush.vue">
      <h2>Octopush</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
