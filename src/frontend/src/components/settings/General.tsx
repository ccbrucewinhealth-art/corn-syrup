import SettingsSection from './SettingsSection';

export default function General() {
  return <SettingsSection source="components/settings/General.vue" title="General" fields={[
    { label: 'Display Timezone', type: 'select', options: ['Auto: Asia/Taipei', 'UTC', 'Asia/Taipei'] },
    { label: 'Server Timezone', type: 'select', options: ['(UTC+08:00) Asia/Taipei'] },
    { label: 'Search Engine Visibility', type: 'radio', value: 'Discourage search engines from indexing site', options: ['Allow indexing', 'Discourage search engines from indexing site'] },
    { label: 'Entry Page', type: 'radio', value: 'Dashboard', options: ['Dashboard', 'Status Page - aa', 'Status Page - ff'] },
    { label: 'Primary Base URL', value: 'https://', help: 'Auto Get' },
    { label: 'Steam API Key', type: 'password', help: 'Steam API Key Help' },
    { label: 'Globalping API Token', type: 'password', help: 'Globalping API Token Help' },
    { label: 'Enable NSCD', type: 'radio', value: 'Enable', options: ['Enable', 'Disable'] },
    { label: 'Chrome/Chromium Executable', value: 'Auto Detect', help: 'Chrome executable help' },
  ]} />;
}
