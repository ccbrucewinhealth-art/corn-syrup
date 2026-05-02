import { useTranslation } from 'react-i18next';

export default function VK() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/VK.vue">
      <h2>VK</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
