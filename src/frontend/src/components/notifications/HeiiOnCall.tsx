import { useTranslation } from 'react-i18next';

export default function HeiiOnCall() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/HeiiOnCall.vue">
      <h2>HeiiOnCall</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
