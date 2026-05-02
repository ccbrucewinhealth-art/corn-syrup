import { useTranslation } from 'react-i18next';

export default function ClickSendSMS() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/ClickSendSMS.vue">
      <h2>ClickSendSMS</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
