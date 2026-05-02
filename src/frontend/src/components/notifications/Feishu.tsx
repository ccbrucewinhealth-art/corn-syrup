import { useTranslation } from 'react-i18next';

export default function Feishu() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Feishu.vue">
      <h2>Feishu</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
