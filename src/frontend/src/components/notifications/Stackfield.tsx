import { useTranslation } from 'react-i18next';

export default function Stackfield() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Stackfield.vue">
      <h2>Stackfield</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
