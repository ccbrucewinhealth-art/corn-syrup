import SettingsSection from './SettingsSection';

export default function APIKeys() {
  return <SettingsSection source="components/settings/APIKeys.vue" title="API Keys" fields={[
    { label: 'Add API Key', type: 'button' },
    { label: 'No API Keys', type: 'paragraph' },
  ]} actions={[]} />;
}
