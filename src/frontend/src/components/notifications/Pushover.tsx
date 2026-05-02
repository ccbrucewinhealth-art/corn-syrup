import { useTranslation } from 'react-i18next';

export default function Pushover() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Pushover.vue">
      <h2>Pushover</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
