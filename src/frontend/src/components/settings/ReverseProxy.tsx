import SettingsSection from './SettingsSection';

export default function ReverseProxy() {
  return <SettingsSection source="components/settings/ReverseProxy.vue" title="Reverse Proxy" fields={[
    { label: 'Reverse Proxy Guide', type: 'paragraph' },
    { label: 'Current Password', type: 'password' },
    { label: 'Trusted Proxies', type: 'textarea', value: '127.0.0.1\n::1' },
  ]} />;
}
