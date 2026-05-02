import { useTranslation } from 'react-i18next';

export default function Discord() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Discord.vue">
      <h2>Discord</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
