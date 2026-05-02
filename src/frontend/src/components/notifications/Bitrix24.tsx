import { useTranslation } from 'react-i18next';

export default function Bitrix24() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Bitrix24.vue">
      <h2>Bitrix24</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
