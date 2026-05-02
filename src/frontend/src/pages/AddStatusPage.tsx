import { useTranslation } from 'react-i18next';

export default function AddStatusPage() {
  const { t } = useTranslation();
  return (
    <section className="card status-page-form" data-source="pages/AddStatusPage.vue">
      <h1 className="mb-3">{t('New Status Page')}</h1>
      <div className="my-3">
        <label className="form-label" htmlFor="status-page-title">{t('Title')}</label>
        <input id="status-page-title" className="form-control" placeholder={t('Default Status Page')} />
      </div>
      <div className="my-3">
        <label className="form-label" htmlFor="status-page-slug">{t('Slug')}</label>
        <input id="status-page-slug" className="form-control" placeholder="default" />
      </div>
      <button type="button" className="btn btn-primary">{t('Save')}</button>
    </section>
  );
}
