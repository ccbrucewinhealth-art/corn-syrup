import { useTranslation } from 'react-i18next';

export default function FlashDuty() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/FlashDuty.vue">
      <h2>FlashDuty</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
