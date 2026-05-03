import { apiClient } from '../../lib/api';
import SettingsSection, { FieldSpec } from './SettingsSection';

export default function RemoteBrowsers() {
  const handleLoad = async (): Promise<Record<string, string>> => {
    const data = await apiClient.getSettings();
    return data.settings || {};
  };

  const handleSave = async (data: Record<string, string>): Promise<void> => {
    await apiClient.updateSettings(data);
  };

  const fields: FieldSpec[] = [
    { label: 'Not available, please setup.', type: 'paragraph' },
    { label: 'Add Remote Browser', type: 'button' },
    { label: 'What is a Remote Browser?', type: 'paragraph' },
    { label: 'Remote Browser URL', key: 'remoteBrowserUrl', value: 'ws://chrome.browserless.io/playwright?token=YOUR-API-TOKEN' },
  ];

  return <SettingsSection
    source="components/settings/RemoteBrowsers.vue"
    title="Remote Browsers"
    fields={fields}
    actions={['Load', 'Save']}
    onLoad={handleLoad}
    onSave={handleSave}
  />;
}
