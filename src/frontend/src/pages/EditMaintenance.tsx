import { useTranslation } from 'react-i18next';

export default function EditMaintenance() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="pages/EditMaintenance.vue">
      <h2>EditMaintenance</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
