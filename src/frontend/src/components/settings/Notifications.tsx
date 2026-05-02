import SettingsSection from './SettingsSection';

export default function Notifications() {
  return <SettingsSection source="components/settings/Notifications.vue" title="Notifications" fields={[
    { label: 'Not available, please setup.', type: 'paragraph' },
    { label: 'Setup Notification', type: 'button' },
    { label: 'Monitor Toast Notifications', type: 'paragraph' },
    { label: 'Error notification timeout', type: 'number', value: '20' },
    { label: 'Success notification timeout', type: 'number', value: '20' },
    { label: 'TLS Expiry Notify Days', value: '7,14,21' },
  ]} />;
}
