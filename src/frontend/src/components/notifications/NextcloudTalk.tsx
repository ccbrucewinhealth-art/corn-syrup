import { useTranslation } from 'react-i18next';

export default function NextcloudTalk() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/NextcloudTalk.vue">
      <h2>NextcloudTalk</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
