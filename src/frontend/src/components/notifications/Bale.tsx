import { useTranslation } from 'react-i18next';

export default function Bale() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Bale.vue">
      <h2>Bale</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
