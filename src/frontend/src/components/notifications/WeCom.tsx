import { useTranslation } from 'react-i18next';

export default function WeCom() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/WeCom.vue">
      <h2>WeCom</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
