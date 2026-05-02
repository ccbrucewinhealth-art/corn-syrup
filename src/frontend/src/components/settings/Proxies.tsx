import SettingsSection from './SettingsSection';

export default function Proxies() {
  return <SettingsSection source="components/settings/Proxies.vue" title="Proxies" fields={[
    { label: 'Not available, please setup.', type: 'paragraph' },
    { label: 'Setup Proxy', type: 'button' },
    { label: 'Proxy Protocol', type: 'select', options: ['http', 'https', 'socks'] },
    { label: 'Proxy Server', value: '' },
    { label: 'Proxy Server has authentication', type: 'checkbox' },
    { label: 'Enable', type: 'checkbox', checked: true },
  ]} />;
}
