import { useTranslation } from 'react-i18next';

export default function Teams() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Teams.vue">
      <h2>Teams</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
