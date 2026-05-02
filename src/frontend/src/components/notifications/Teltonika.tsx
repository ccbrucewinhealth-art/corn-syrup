import { useTranslation } from 'react-i18next';

export default function Teltonika() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Teltonika.vue">
      <h2>Teltonika</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
