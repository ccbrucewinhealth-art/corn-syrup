import { useTranslation } from 'react-i18next';

export default function Fluxer() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Fluxer.vue">
      <h2>Fluxer</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
