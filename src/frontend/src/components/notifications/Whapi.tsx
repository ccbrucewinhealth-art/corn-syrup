import { useTranslation } from 'react-i18next';

export default function Whapi() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Whapi.vue">
      <h2>Whapi</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
