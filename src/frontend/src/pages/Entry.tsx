import { useTranslation } from 'react-i18next';

export default function Entry() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="pages/Entry.vue">
      <h2>Entry</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
