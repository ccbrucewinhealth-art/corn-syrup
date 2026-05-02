import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { apiClient } from '../../lib/api';
import SettingsSection from './SettingsSection';

interface GeneralSettings {
  timezone?: string;
  serverTimezone?: string;
  searchEngineVisibility?: string;
  entryPage?: string;
  primaryBaseUrl?: string;
  steamApiKey?: string;
  globalpingApiToken?: string;
  nscdEnabled?: string;
  chromeExecutable?: string;
}

export default function General() {
  const { t } = useTranslation();
  const [settings, setSettings] = useState<GeneralSettings>({});
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    apiClient.getSettings()
      .then((data) => {
        if (data && typeof data === 'object') {
          setSettings(data as GeneralSettings);
        }
      })
      .catch((err) => console.error('Load settings failed:', err))
      .finally(() => setLoading(false));
  }, []);

  return <SettingsSection source="components/settings/General.vue" title="General" fields={[
    { label: 'Display Timezone', type: 'select', value: settings.timezone, options: ['Auto: Asia/Taipei', 'UTC', 'Asia/Taipei'] },
    { label: 'Server Timezone', type: 'select', value: settings.serverTimezone, options: ['(UTC+08:00) Asia/Taipei'] },
    { label: 'Search Engine Visibility', type: 'radio', value: settings.searchEngineVisibility, options: ['Allow indexing', 'Discourage search engines from indexing site'] },
    { label: 'Entry Page', type: 'radio', value: settings.entryPage, options: ['Dashboard', 'Status Page - aa', 'Status Page - ff'] },
    { label: 'Primary Base URL', value: settings.primaryBaseUrl, help: 'Auto Get' },
    { label: 'Steam API Key', type: 'password', value: settings.steamApiKey, help: 'Steam API Key Help' },
    { label: 'Globalping API Token', type: 'password', value: settings.globalpingApiToken, help: 'Globalping API Token Help' },
    { label: 'Enable NSCD', type: 'radio', value: settings.nscdEnabled, options: ['Enable', 'Disable'] },
    { label: 'Chrome/Chromium Executable', value: settings.chromeExecutable, help: 'Chrome executable help' },
  ]} />;
}
