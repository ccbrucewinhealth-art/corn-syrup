import { useTranslation } from 'react-i18next';

export default function Bark() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Bark.vue">
      <h2>Bark</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
