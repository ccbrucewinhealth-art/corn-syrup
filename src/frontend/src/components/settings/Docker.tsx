import SettingsSection from './SettingsSection';

export default function Docker() {
  return <SettingsSection source="components/settings/Docker.vue" title="Docker Hosts" fields={[
    { label: 'Not available, please setup.', type: 'paragraph' },
    { label: 'Setup Docker Host', type: 'button' },
    { label: 'Docker Daemon', value: '/var/run/docker.sock' },
  ]} />;
}
