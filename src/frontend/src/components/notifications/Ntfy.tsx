import { useTranslation } from 'react-i18next';

export default function Ntfy() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Ntfy.vue">
      <h2>Ntfy</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
