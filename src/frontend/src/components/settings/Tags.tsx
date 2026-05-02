import SettingsSection from './SettingsSection';

export default function Tags() {
  return <SettingsSection source="components/settings/Tags.vue" title="Tags" fields={[
    { label: 'Add Tag', type: 'button' },
    { label: 'Edit Tag', type: 'paragraph' },
    { label: 'Tag Name', value: '' },
    { label: 'Tag Color', type: 'select', options: ['green', 'blue', 'red', 'yellow'] },
  ]} />;
}
