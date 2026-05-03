import SettingsSection, { FieldSpec } from './SettingsSection';

export default function Security() {
  const fields: FieldSpec[] = [
    { label: 'Change Password', type: 'paragraph' },
    { label: 'Current Password', type: 'password', key: 'currentPassword' },
    { label: 'New Password', type: 'password', key: 'newPassword' },
    { label: 'Confirm New Password', type: 'password', key: 'confirmPassword' },
    { label: 'Update Password', type: 'button' },
    { label: 'Two Factor Authentication Settings', type: 'paragraph' },
  ];

  return <SettingsSection
    source="components/settings/Security.vue"
    title="Security"
    fields={fields}
    actions={[]}
  />;
}
