// checklist No.255 translated from backend/i18n.js
// 修正說明：補足繁體中文、英文、日文語系資源、預設繁體中文 fallback 與翻譯查找。
use crate::backend::logging;

pub const DEFAULT_LOCALE: &str = "zh-TW";
pub const SUPPORTED_LOCALES: [&str; 3] = ["zh-TW", "en-US", "ja-JP"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslationEntry { pub key:&'static str, pub zh_tw:&'static str, pub en_us:&'static str, pub ja_jp:&'static str }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct I18nCatalog { pub locale:String, pub fallback_locale:String, pub entries:Vec<TranslationEntry> }

impl Default for I18nCatalog {
    fn default()->Self { logging::debug("frontend.i18n", "default", "init i18n catalog"); Self{locale:DEFAULT_LOCALE.to_string(),fallback_locale:DEFAULT_LOCALE.to_string(),entries:default_entries()} }
}

impl I18nCatalog {
    pub fn set_locale(&mut self, locale:&str)->Result<(),String>{ logging::debug("frontend.i18n", "set_locale", locale); if !SUPPORTED_LOCALES.contains(&locale){return Err(format!("unsupported locale {locale}"));} self.locale=locale.to_string(); Ok(()) }
    pub fn translate(&self, key:&str)->String{
        logging::debug("frontend.i18n", "translate", key);
        let Some(entry)=self.entries.iter().find(|e|e.key==key) else { return key.to_string(); };
        match self.locale.as_str(){ "en-US"=>entry.en_us.to_string(), "ja-JP"=>entry.ja_jp.to_string(), _=>entry.zh_tw.to_string() }
    }
    pub fn has_key(&self, key:&str)->bool{ self.entries.iter().any(|e|e.key==key) }
    pub fn render_model(&self)->Vec<(String,String)>{
        logging::debug("frontend.i18n", "render_model", format!("locale={}", self.locale));
        let mut rows=vec![("locale".to_string(),self.locale.clone()),("fallback".to_string(),self.fallback_locale.clone())];
        for e in &self.entries { rows.push((format!("key:{}",e.key), self.translate(e.key))); }
        rows
    }
}

pub fn default_entries()->Vec<TranslationEntry>{
    logging::debug("frontend.i18n", "default_entries", "build zh-TW/en-US/ja-JP resources");
    vec![
        TranslationEntry{key:"dashboard",zh_tw:"儀表板",en_us:"Dashboard",ja_jp:"ダッシュボード"},
        TranslationEntry{key:"monitor_list",zh_tw:"監控列表",en_us:"Monitor List",ja_jp:"監視一覧"},
        TranslationEntry{key:"settings",zh_tw:"設定",en_us:"Settings",ja_jp:"設定"},
        TranslationEntry{key:"notifications",zh_tw:"通知",en_us:"Notifications",ja_jp:"通知"},
        TranslationEntry{key:"status_page",zh_tw:"狀態頁面",en_us:"Status Page",ja_jp:"ステータスページ"},
        TranslationEntry{key:"setup",zh_tw:"初始設定",en_us:"Setup",ja_jp:"初期設定"},
        TranslationEntry{key:"login",zh_tw:"登入",en_us:"Login",ja_jp:"ログイン"},
        TranslationEntry{key:"save",zh_tw:"儲存",en_us:"Save",ja_jp:"保存"},
        TranslationEntry{key:"cancel",zh_tw:"取消",en_us:"Cancel",ja_jp:"キャンセル"},
        TranslationEntry{key:"not_found",zh_tw:"找不到頁面",en_us:"Not Found",ja_jp:"見つかりません"},
    ]
}

pub fn t(key:&str)->String{ I18nCatalog::default().translate(key) }
pub fn render_model(catalog:&I18nCatalog)->Vec<(String,String)>{ catalog.render_model() }
