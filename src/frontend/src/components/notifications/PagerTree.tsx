import { useTranslation } from 'react-i18next';

export default function PagerTree() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/PagerTree.vue">
      <h2>PagerTree</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
