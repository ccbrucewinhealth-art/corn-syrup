import { apiClient } from '../../lib/api';
import SettingsSection, { FieldSpec } from './SettingsSection';

export default function ReverseProxy() {
  const handleLoad = async (): Promise<Record<string, string>> => {
    const data = await apiClient.getSettings();
    return data.settings || {};
  };

  const handleSave = async (data: Record<string, string>): Promise<void> => {
    await apiClient.updateSettings(data);
  };

  const fields: FieldSpec[] = [
    { label: 'Reverse Proxy Guide', type: 'paragraph' },
    { label: 'Current Password', type: 'password', key: 'currentPassword' },
    { label: 'Trusted Proxies', type: 'textarea', key: 'trustedProxies', value: '127.0.0.1\n::1' },
  ];

  return <SettingsSection
    source="components/settings/ReverseProxy.vue"
    title="Reverse Proxy"
    fields={fields}
    actions={['Load', 'Save']}
    onLoad={handleLoad}
    onSave={handleSave}
  />;
}
