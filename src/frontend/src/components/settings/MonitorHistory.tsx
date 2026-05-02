import SettingsSection from './SettingsSection';

export default function MonitorHistory() {
  return <SettingsSection source="components/settings/MonitorHistory.vue" title="Monitor History" fields={[
    { label: 'Keep Monitor History', type: 'number', value: '180', help: 'Keep Monitor History Help' },
    { label: 'Enable Data Retention', type: 'checkbox', checked: true },
    { label: 'Delete All Stats', type: 'button' },
  ]} />;
}
