import { useTranslation } from 'react-i18next';

export default function FortySixelks() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/46elks.vue">
      <h2>FortySixelks</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
