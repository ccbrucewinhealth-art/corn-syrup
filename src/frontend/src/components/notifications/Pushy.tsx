import { useTranslation } from 'react-i18next';

export default function Pushy() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Pushy.vue">
      <h2>Pushy</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
