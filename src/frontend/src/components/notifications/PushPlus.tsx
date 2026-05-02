import { useTranslation } from 'react-i18next';

export default function PushPlus() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/PushPlus.vue">
      <h2>PushPlus</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
