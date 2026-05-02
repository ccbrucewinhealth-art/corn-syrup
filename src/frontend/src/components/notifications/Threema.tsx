import { useTranslation } from 'react-i18next';

export default function Threema() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Threema.vue">
      <h2>Threema</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
