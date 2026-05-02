import { useTranslation } from 'react-i18next';

export default function Squadcast() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Squadcast.vue">
      <h2>Squadcast</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
