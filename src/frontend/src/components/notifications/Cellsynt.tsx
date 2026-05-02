import { useTranslation } from 'react-i18next';

export default function Cellsynt() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Cellsynt.vue">
      <h2>Cellsynt</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
