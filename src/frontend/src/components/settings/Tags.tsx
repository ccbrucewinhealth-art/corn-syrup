import { apiClient } from '../../lib/api';
import SettingsSection, { FieldSpec } from './SettingsSection';

export default function Tags() {
  const handleLoad = async (): Promise<Record<string, string>> => {
    const data = await apiClient.getSettings();
    return data.settings || {};
  };

  const handleSave = async (data: Record<string, string>): Promise<void> => {
    await apiClient.updateSettings(data);
  };

  const fields: FieldSpec[] = [
    { label: 'Add Tag', type: 'button' },
    { label: 'Edit Tag', type: 'paragraph' },
    { label: 'Tag Name', key: 'tagName', value: '' },
    { label: 'Tag Color', type: 'select', key: 'tagColor', options: ['green', 'blue', 'red', 'yellow'] },
  ];

  return <SettingsSection
    source="components/settings/Tags.vue"
    title="Tags"
    fields={fields}
    actions={['Load', 'Save']}
    onLoad={handleLoad}
    onSave={handleSave}
  />;
}
