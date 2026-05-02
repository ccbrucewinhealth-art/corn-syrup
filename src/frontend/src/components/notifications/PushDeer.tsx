import { useTranslation } from 'react-i18next';

export default function PushDeer() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/PushDeer.vue">
      <h2>PushDeer</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
