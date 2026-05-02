import { useTranslation } from 'react-i18next';

export default function Signal() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Signal.vue">
      <h2>Signal</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
