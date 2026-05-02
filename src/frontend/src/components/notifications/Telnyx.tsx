import { useTranslation } from 'react-i18next';

export default function Telnyx() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Telnyx.vue">
      <h2>Telnyx</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
