import { useTranslation } from 'react-i18next';

export default function SMSC() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/SMSC.vue">
      <h2>SMSC</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
