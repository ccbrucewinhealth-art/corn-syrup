import SettingsSection, { FieldSpec } from './SettingsSection';

export default function APIKeys() {
  const fields: FieldSpec[] = [
    { label: 'Add API Key', type: 'button' },
    { label: 'No API Keys', type: 'paragraph' },
  ];

  return <SettingsSection
    source="components/settings/APIKeys.vue"
    title="API Keys"
    fields={fields}
    actions={[]}
  />;
}
