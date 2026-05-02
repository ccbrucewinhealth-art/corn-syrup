import { useTranslation } from 'react-i18next';

export default function Nostr() {
  const { t } = useTranslation();
  return (
    <section className="card" data-source="components/notifications/Nostr.vue">
      <h2>Nostr</h2>
      <p>{t('common.notImplemented')}</p>
    </section>
  );
}
