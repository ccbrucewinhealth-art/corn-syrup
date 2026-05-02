import { useTranslation } from 'react-i18next';

export default function Pumble() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Pumble.vue">
      <h2>Pumble</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
