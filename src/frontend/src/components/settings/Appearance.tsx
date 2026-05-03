import { apiClient } from '../../lib/api';
import SettingsSection, { FieldSpec } from './SettingsSection';

export default function Appearance() {
  const handleLoad = async (): Promise<Record<string, string>> => {
    const data = await apiClient.getSettings();
    return data.settings || {};
  };

  const handleSave = async (data: Record<string, string>): Promise<void> => {
    await apiClient.updateSettings(data);
  };

  const fields: FieldSpec[] = [
    { label: 'Language', type: 'select', key: 'language', options: ['繁體中文', 'English', '日本語'] },
    { label: 'Theme', type: 'select', key: 'theme', options: ['Auto', 'Light', 'Dark'] },
    { label: 'Theme - Heartbeat Bar', type: 'select', key: 'heartbeatBarTheme', options: ['Normal', 'Bottom', 'None'] },
  ];

  return <SettingsSection
    source="components/settings/Appearance.vue"
    title="Appearance"
    fields={fields}
    actions={['Load', 'Save']}
    onLoad={handleLoad}
    onSave={handleSave}
  />;
}
