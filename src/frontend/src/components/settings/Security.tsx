import SettingsSection from './SettingsSection';

export default function Security() {
  return <SettingsSection source="components/settings/Security.vue" title="Security" fields={[
    { label: 'Change Password', type: 'paragraph' },
    { label: 'Current Password', type: 'password' },
    { label: 'New Password', type: 'password' },
    { label: 'Confirm New Password', type: 'password' },
    { label: 'Update Password', type: 'button' },
    { label: 'Two Factor Authentication Settings', type: 'paragraph' },
  ]} actions={[]} />;
}
