import { useTranslation } from 'react-i18next';

export default function Pushbullet() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Pushbullet.vue">
      <h2>Pushbullet</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
