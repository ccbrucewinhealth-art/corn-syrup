import SettingsSection from './SettingsSection';

export default function RemoteBrowsers() {
  return <SettingsSection source="components/settings/RemoteBrowsers.vue" title="Remote Browsers" fields={[
    { label: 'Not available, please setup.', type: 'paragraph' },
    { label: 'Add Remote Browser', type: 'button' },
    { label: 'What is a Remote Browser?', type: 'paragraph' },
    { label: 'Remote Browser URL', value: 'ws://chrome.browserless.io/playwright?token=YOUR-API-TOKEN' },
  ]} />;
}
