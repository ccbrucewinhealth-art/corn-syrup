import SettingsSection from './SettingsSection';

export default function Appearance() {
  return <SettingsSection source="components/settings/Appearance.vue" title="Appearance" fields={[
    { label: 'Language', type: 'select', options: ['繁體中文', 'English', '日本語'] },
    { label: 'Theme', type: 'select', options: ['Auto', 'Light', 'Dark'] },
    { label: 'Theme - Heartbeat Bar', type: 'select', options: ['Normal', 'Bottom', 'None'] },
  ]} />;
}
