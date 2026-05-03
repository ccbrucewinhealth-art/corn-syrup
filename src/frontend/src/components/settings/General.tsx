import { useTranslation } from 'react-i18next';
import { apiClient } from '../../lib/api';
import SettingsSection, { FieldSpec } from './SettingsSection';

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

  const handleLoad = async (): Promise<Record<string, string>> => {
    const data = await apiClient.getSettings();
    return data.settings || {};
  };

  const handleSave = async (data: Record<string, string>): Promise<void> => {
    await apiClient.updateSettings(data);
  };

  const fields: FieldSpec[] = [
    { label: 'Display Timezone', type: 'select', key: 'timezone', options: ['Auto: Asia/Taipei', 'UTC', 'Asia/Taipei'] },
    { label: 'Server Timezone', type: 'select', key: 'serverTimezone', options: ['(UTC+08:00) Asia/Taipei'] },
    { label: 'Search Engine Visibility', type: 'radio', key: 'searchEngineVisibility', options: ['Allow indexing', 'Discourage search engines from indexing site'] },
    { label: 'Entry Page', type: 'radio', key: 'entryPage', options: ['Dashboard', 'Status Page - aa', 'Status Page - ff'] },
    { label: 'Primary Base URL', key: 'primaryBaseUrl', help: 'Auto Get' },
    { label: 'Steam API Key', type: 'password', key: 'steamApiKey', help: 'Steam API Key Help' },
    { label: 'Globalping API Token', type: 'password', key: 'globalpingApiToken', help: 'Globalping API Token Help' },
    { label: 'Enable NSCD', type: 'radio', key: 'nscdEnabled', options: ['Enable', 'Disable'] },
    { label: 'Chrome/Chromium Executable', key: 'chromeExecutable', help: 'Chrome executable help' },
  ];

  return <SettingsSection
    source="components/settings/General.vue"
    title="General"
    fields={fields}
    actions={['Load', 'Save']}
    onLoad={handleLoad}
    onSave={handleSave}
  />;
}
