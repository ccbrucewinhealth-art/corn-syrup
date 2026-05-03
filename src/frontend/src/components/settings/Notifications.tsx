import { apiClient } from '../../lib/api';
import SettingsSection, { FieldSpec } from './SettingsSection';

export default function Notifications() {
  const handleLoad = async (): Promise<Record<string, string>> => {
    const data = await apiClient.getSettings();
    return data.settings || {};
  };

  const handleSave = async (data: Record<string, string>): Promise<void> => {
    await apiClient.updateSettings(data);
  };

  const fields: FieldSpec[] = [
    { label: 'Not available, please setup.', type: 'paragraph' },
    { label: 'Setup Notification', type: 'button' },
    { label: 'Monitor Toast Notifications', type: 'paragraph' },
    { label: 'Error notification timeout', type: 'number', key: 'errorNotificationTimeout', value: '20' },
    { label: 'Success notification timeout', type: 'number', key: 'successNotificationTimeout', value: '20' },
    { label: 'TLS Expiry Notify Days', key: 'tlsExpiryNotifyDays', value: '7,14,21' },
  ];

  return <SettingsSection
    source="components/settings/Notifications.vue"
    title="Notifications"
    fields={fields}
    actions={['Load', 'Save']}
    onLoad={handleLoad}
    onSave={handleSave}
  />;
}
