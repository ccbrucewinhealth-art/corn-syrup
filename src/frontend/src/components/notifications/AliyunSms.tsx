import { useTranslation } from 'react-i18next';

export default function AliyunSms() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/AliyunSms.vue">
      <h2>AliyunSms</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
