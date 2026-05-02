import { useTranslation } from 'react-i18next';

export default function Evolution() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Evolution.vue">
      <h2>Evolution</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
