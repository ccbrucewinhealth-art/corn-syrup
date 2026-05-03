import { apiClient } from '../../lib/api';
import SettingsSection, { FieldSpec } from './SettingsSection';

export default function MonitorHistory() {
  const handleLoad = async (): Promise<Record<string, string>> => {
    const data = await apiClient.getSettings();
    return data.settings || {};
  };

  const handleSave = async (data: Record<string, string>): Promise<void> => {
    await apiClient.updateSettings(data);
  };

  const fields: FieldSpec[] = [
    { label: 'Keep Monitor History', type: 'number', key: 'keepMonitorHistory', value: '180', help: 'Keep Monitor History Help' },
    { label: 'Enable Data Retention', type: 'checkbox', key: 'dataRetentionEnabled', checked: true },
    { label: 'Delete All Stats', type: 'button' },
  ];

  return <SettingsSection
    source="components/settings/MonitorHistory.vue"
    title="Monitor History"
    fields={fields}
    actions={['Load', 'Save']}
    onLoad={handleLoad}
    onSave={handleSave}
  />;
}
