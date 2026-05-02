import { useTranslation } from 'react-i18next';

export default function Keep() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Keep.vue">
      <h2>Keep</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
