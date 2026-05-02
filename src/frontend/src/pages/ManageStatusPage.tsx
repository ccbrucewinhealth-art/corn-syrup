import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { apiClient } from '../lib/api';

interface StatusPage {
  id: number;
  title: string;
  slug: string;
  published: boolean;
}

export default function ManageStatusPage() {
  const { t } = useTranslation();
  const [pages, setPages] = useState<StatusPage[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    apiClient.getStatusPages()
      .then((data) => {
        setPages(Array.isArray(data) ? data : []);
      })
      .catch((err) => {
        console.error('Failed to load status pages:', err);
        setError(t('Failed to load'));
      })
      .finally(() => setLoading(false));
  }, [t]);

  const handleDelete = async (id: number) => {
    if (!confirm(t('Confirm delete'))) return;
    try {
      await apiClient.deleteStatusPage(id);
      setPages(pages.filter((p) => p.id !== id));
    } catch (err) {
      console.error('Delete failed:', err);
    }
  };

  return (
    <section className="status-page-management" data-source="pages/ManageStatusPage.vue">
      <h1 className="mb-3">{t('Status Pages')}</h1>
      <div>
        <Link to="/add-status-page" className="btn btn-primary mb-3">+ {t('New Status Page')}</Link>
      </div>
      {loading ? (
        <div className="shadow-box">{t('Loading...')}</div>
      ) : error ? (
        <div className="shadow-box text-danger">{error}</div>
      ) : pages.length === 0 ? (
        <div className="shadow-box status-page-empty-box">
          <span className="d-flex align-items-center justify-content-center my-3">{t('No Status Pages')}</span>
        </div>
      ) : (
        <div className="table-responsive">
          <table className="table table-hover">
            <thead>
              <tr>
                <th>{t('Title')}</th>
                <th>{t('Slug')}</th>
                <th>{t('Status')}</th>
                <th>{t('Actions')}</th>
              </tr>
            </thead>
            <tbody>
              {pages.map((page) => (
                <tr key={page.id}>
                  <td><Link to={`/status-page/${page.slug}`}>{page.title}</Link></td>
                  <td><code>{page.slug}</code></td>
                  <td>
                    <span className={`badge ${page.published ? 'bg-primary' : 'bg-secondary'}`}>
                      {page.published ? t('Published') : t('Draft')}
                    </span>
                  </td>
                  <td>
                    <div className="btn-group">
                      <Link to={`/edit-status-page/${page.id}`} className="btn btn-normal">{t('Edit')}</Link>
                      <button type="button" className="btn btn-normal" onClick={() => handleDelete(page.id)}>
                        {t('Delete')}
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </section>
  );
}
