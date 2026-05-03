import { apiClient } from '../../lib/api';
import SettingsSection, { FieldSpec } from './SettingsSection';

export default function Docker() {
  const handleLoad = async (): Promise<Record<string, string>> => {
    const data = await apiClient.getSettings();
    return data.settings || {};
  };

  const handleSave = async (data: Record<string, string>): Promise<void> => {
    await apiClient.updateSettings(data);
  };

  const fields: FieldSpec[] = [
    { label: 'Not available, please setup.', type: 'paragraph' },
    { label: 'Setup Docker Host', type: 'button' },
    { label: 'Docker Daemon', key: 'dockerDaemon', value: '/var/run/docker.sock' },
  ];

  return <SettingsSection
    source="components/settings/Docker.vue"
    title="Docker Hosts"
    fields={fields}
    actions={['Load', 'Save']}
    onLoad={handleLoad}
    onSave={handleSave}
  />;
}
