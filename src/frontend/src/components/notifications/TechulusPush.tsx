import { useTranslation } from 'react-i18next';

export default function TechulusPush() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/TechulusPush.vue">
      <h2>TechulusPush</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
