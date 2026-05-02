import { useTranslation } from 'react-i18next';

export default function Kook() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Kook.vue">
      <h2>Kook</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
