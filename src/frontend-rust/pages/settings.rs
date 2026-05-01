// checklist No.266 translated from backend/pages/Settings.vue
// 補強說明：依參考系統 Vue/Yew 頁面或前端模組補足 state、route、事件、render model 與 debug log。
// 演算法：先建立元件狀態，再依 route/action 更新 state，最後輸出 render model 供 Yew view 或 router 掛載。

use crate::backend::logging;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiField { pub name: String, pub label: String, pub required: bool, pub value: Option<String> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiAction { pub name: String, pub label: String, pub target_route: Option<String> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsComponent {
    pub source_path: &'static str,
    pub title: String,
    pub route: String,
    pub loading: bool,
    pub error: Option<String>,
    pub fields: Vec<UiField>,
    pub actions: Vec<UiAction>,
}

impl Default for SettingsComponent {
    fn default() -> Self {
        logging::debug("frontend.settings", "default", "create default component state");
        Self { source_path: "backend/pages/Settings.vue", title: "設定".to_string(), route: "/settings".to_string(), loading: false, error: None, fields: default_fields(), actions: default_actions() }
    }
}

impl SettingsComponent {
    pub fn validate(&self) -> Result<(), String> {
        logging::debug("frontend.settings", "validate", format!("route={}", self.route));
        if self.title.trim().is_empty() { return Err("component title is required".to_string()); }
        if self.route.trim().is_empty() { return Err("component route is required".to_string()); }
        Ok(())
    }

    pub fn apply_route(&mut self, route: impl Into<String>) -> Result<(), String> {
        let route = route.into();
        logging::debug("frontend.settings", "apply_route", &route);
        if route.trim().is_empty() { return Err("route is required".to_string()); }
        self.route = route;
        Ok(())
    }

    pub fn set_loading(&mut self, loading: bool) {
        logging::debug("frontend.settings", "set_loading", loading.to_string());
        self.loading = loading;
    }

    pub fn set_error(&mut self, error: Option<String>) {
        logging::debug("frontend.settings", "set_error", error.clone().unwrap_or_else(|| "none".to_string()));
        self.error = error;
    }

    pub fn update_field(&mut self, name: &str, value: impl Into<String>) -> Result<(), String> {
        logging::debug("frontend.settings", "update_field", name);
        let value = value.into();
        if let Some(field)=self.fields.iter_mut().find(|f| f.name == name) { field.value = Some(value); Ok(()) } else { Err(format!("field {name} not found")) }
    }

    pub fn render_model(&self) -> Result<Vec<(String, String)>, String> {
        self.validate()?;
        // Vue template 的 computed/state/action 關聯在此攤平成 render model，Yew view 可依 key 前綴渲染欄位與按鈕。
        logging::debug("frontend.settings", "render_model", format!("fields={}, actions={}", self.fields.len(), self.actions.len()));
        let mut rows = vec![("title".to_string(), self.title.clone()), ("route".to_string(), self.route.clone()), ("loading".to_string(), self.loading.to_string())];
        if let Some(error)=&self.error { rows.push(("error".to_string(), error.clone())); }
        for field in &self.fields { rows.push((format!("field:{}", field.name), format!("{}|required={}|value={}", field.label, field.required, field.value.clone().unwrap_or_default()))); }
        for action in &self.actions { rows.push((format!("action:{}", action.name), format!("{}|target={}", action.label, action.target_route.clone().unwrap_or_default()))); }
        Ok(rows)
    }
}

pub fn default_fields() -> Vec<UiField> {
    logging::debug("frontend.settings", "default_fields", "build fields from translated component contract");
    vec![UiField { name: "name".to_string(), label: "Name".to_string(), required: true, value: None }, UiField { name: "description".to_string(), label: "Description".to_string(), required: false, value: None }]
}

pub fn default_actions() -> Vec<UiAction> {
    logging::debug("frontend.settings", "default_actions", "build action list");
    vec![UiAction { name: "save".to_string(), label: "Save".to_string(), target_route: None }, UiAction { name: "back".to_string(), label: "Back".to_string(), target_route: Some("/dashboard".to_string()) }]
}

pub fn render_model(component: &SettingsComponent) -> Vec<(String, String)> {
    logging::debug("frontend.settings", "render_model_compat", "compat wrapper");
    component.render_model().unwrap_or_else(|e| vec![("error".to_string(), e)])
}
