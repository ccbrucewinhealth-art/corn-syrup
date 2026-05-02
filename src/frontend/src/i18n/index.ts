import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import zhTW from './locales/zh-TW.json';
import en from './locales/en.json';
import ja from './locales/ja.json';
import { debugAction } from '../lib/logger';

export const defaultLocale = 'zh-TW';
export const resources = { 'zh-TW': { translation: zhTW }, en: { translation: en }, ja: { translation: ja } };

i18n.use(initReactI18next).init({
  resources,
  lng: localStorage.getItem('i18nextLng') || defaultLocale,
  fallbackLng: defaultLocale,
  supportedLngs: ['zh-TW', 'en', 'ja'],
  interpolation: { escapeValue: false },
  returnEmptyString: false,
  missingKeyHandler: (lngs, ns, key) => debugAction('i18n', 'missing.key', { lngs, ns, key }),
});

debugAction('i18n', 'initialized', { defaultLocale, languages: Object.keys(resources) });

export default i18n;
