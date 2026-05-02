import { useTranslation } from 'react-i18next';

export default function DingDing() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/DingDing.vue">
      <h2>DingDing</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
