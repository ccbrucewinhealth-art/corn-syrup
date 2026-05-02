import { useTranslation } from 'react-i18next';

export default function FreeMobile() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/FreeMobile.vue">
      <h2>FreeMobile</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
