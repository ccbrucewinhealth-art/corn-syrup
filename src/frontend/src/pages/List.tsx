import { useTranslation } from 'react-i18next';

export default function List() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="pages/List.vue">
      <h2>List</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
