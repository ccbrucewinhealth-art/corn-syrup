import { apiClient } from '../../lib/api';
import SettingsSection, { FieldSpec } from './SettingsSection';

export default function Proxies() {
  const handleLoad = async (): Promise<Record<string, string>> => {
    const data = await apiClient.getSettings();
    return data.settings || {};
  };

  const handleSave = async (data: Record<string, string>): Promise<void> => {
    await apiClient.updateSettings(data);
  };

  const fields: FieldSpec[] = [
    { label: 'Not available, please setup.', type: 'paragraph' },
    { label: 'Setup Proxy', type: 'button' },
    { label: 'Proxy Protocol', type: 'select', key: 'proxyProtocol', options: ['http', 'https', 'socks'] },
    { label: 'Proxy Server', key: 'proxyServer', value: '' },
    { label: 'Proxy Server has authentication', type: 'checkbox', key: 'proxyAuth' },
    { label: 'Enable', type: 'checkbox', key: 'proxyEnabled', checked: true },
  ];

  return <SettingsSection
    source="components/settings/Proxies.vue"
    title="Proxies"
    fields={fields}
    actions={['Load', 'Save']}
    onLoad={handleLoad}
    onSave={handleSave}
  />;
}
