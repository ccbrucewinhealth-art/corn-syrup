import { useTranslation } from 'react-i18next';

export default function Alerta() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Alerta.vue">
      <h2>Alerta</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
