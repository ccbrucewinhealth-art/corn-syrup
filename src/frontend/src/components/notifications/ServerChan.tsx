import { useTranslation } from 'react-i18next';

export default function ServerChan() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/ServerChan.vue">
      <h2>ServerChan</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
