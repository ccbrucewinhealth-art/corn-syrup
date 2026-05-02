import { useTranslation } from 'react-i18next';

export default function LunaSea() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/LunaSea.vue">
      <h2>LunaSea</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
