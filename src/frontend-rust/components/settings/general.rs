// checklist translated from General.vue
// 修正說明：補足 provider/page/component 專屬欄位、敏感值遮罩、狀態事件與 render model，移除 name/value 佔位模板。
use crate::backend::logging;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiEvent { Init, Load, Save, Cancel, Delete, Toggle, Navigate(String), Custom(String) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiField { pub name:String, pub label:String, pub required:bool, pub value:Option<String>, pub visible:bool, pub field_type:String, pub sensitive:bool }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiAction { pub name:String, pub label:String, pub enabled:bool, pub target_route:Option<String>, pub destructive:bool }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneralComponent { pub source_path:&'static str, pub title:String, pub route:String, pub loading:bool, pub dirty:bool, pub error:Option<String>, pub fields:Vec<UiField>, pub actions:Vec<UiAction>, pub emitted:Vec<String> }

impl Default for GeneralComponent {
    fn default() -> Self {
        logging::debug("ui.general", "default", "init component with translated field schema");
        Self { source_path:"General.vue", title:"一般設定".to_string(), route:"/settings/general".to_string(), loading:false, dirty:false, error:None, fields:default_fields(), actions:default_actions(), emitted:Vec::new() }
    }
}

impl GeneralComponent {
    pub fn validate(&self)->Result<(),String>{
        logging::debug("ui.general", "validate", format!("route={}, fields={}", self.route, self.fields.len()));
        if self.title.trim().is_empty() { return Err("title required".to_string()); }
        if self.route.trim().is_empty() { return Err("route required".to_string()); }
        for f in &self.fields {
            if f.required && f.visible && f.value.as_deref().unwrap_or_default().trim().is_empty() { return Err(format!("field {} required", f.name)); }
        }
        Ok(())
    }
    pub fn set_field(&mut self,name:&str,value:impl Into<String>)->Result<(),String>{
        logging::debug("ui.general", "set_field", name);
        let f=self.fields.iter_mut().find(|f|f.name==name).ok_or_else(||format!("field {name} not found"))?;
        f.value=Some(value.into()); self.dirty=true; Ok(())
    }
    pub fn dispatch_action(&mut self,event:UiEvent)->Result<(),String>{
        logging::debug("ui.general", "dispatch_action", format!("{:?}",event));
        match event {
            UiEvent::Init|UiEvent::Load=>{self.loading=true;self.emitted.push("load".to_string())},
            UiEvent::Save=>{self.validate()?;self.loading=false;self.dirty=false;self.emitted.push("save".to_string())},
            UiEvent::Cancel=>{self.dirty=false;self.emitted.push("cancel".to_string())},
            UiEvent::Delete=>{self.emitted.push("delete".to_string())},
            UiEvent::Toggle=>{self.dirty=true;self.emitted.push("toggle".to_string())},
            UiEvent::Navigate(route)=>{self.route=route.clone();self.emitted.push(format!("navigate:{}",route))},
            UiEvent::Custom(v)=>self.emitted.push(v),
        } Ok(())
    }
    pub fn render_model(&self)->Result<Vec<(String,String)>,String>{
        self.validate()?; logging::debug("ui.general", "render_model", format!("actions={}", self.actions.len()));
        let mut rows=vec![("title".to_string(),self.title.clone()),("route".to_string(),self.route.clone()),("loading".to_string(),self.loading.to_string()),("dirty".to_string(),self.dirty.to_string())];
        if let Some(e)=&self.error { rows.push(("error".to_string(),e.clone())); }
        for f in &self.fields { if f.visible { let value=if f.sensitive && f.value.is_some() { "********".to_string() } else { f.value.clone().unwrap_or_default() }; rows.push((format!("field:{}",f.name),format!("{}|type={}|required={}|value={}",f.label,f.field_type,f.required,value))); } }
        for a in &self.actions { rows.push((format!("action:{}",a.name),format!("{}|enabled={}|target={}|destructive={}",a.label,a.enabled,a.target_route.clone().unwrap_or_default(),a.destructive))); }
        Ok(rows)
    }
}

pub fn default_fields()->Vec<UiField>{
    logging::debug("ui.general", "default_fields", "build component-specific schema");
    vec![
        UiField { name: "site_name".to_string(), label: "Site Name".to_string(), required: true, value: None, visible: true, field_type: "text".to_string(), sensitive: false },
        UiField { name: "language".to_string(), label: "Language".to_string(), required: true, value: None, visible: true, field_type: "select".to_string(), sensitive: false },
        UiField { name: "timezone".to_string(), label: "Timezone".to_string(), required: true, value: None, visible: true, field_type: "select".to_string(), sensitive: false },
        UiField { name: "enable_auth".to_string(), label: "Enable Auth".to_string(), required: false, value: None, visible: true, field_type: "checkbox".to_string(), sensitive: false }
    ]
}

pub fn default_actions()->Vec<UiAction>{
    logging::debug("ui.general", "default_actions", "build supported actions");
    vec![
        UiAction{name:"save".to_string(),label:"Save".to_string(),enabled:true,target_route:None,destructive:false},
        UiAction{name:"cancel".to_string(),label:"Cancel".to_string(),enabled:true,target_route:Some("/dashboard".to_string()),destructive:false},
        UiAction{name:"delete".to_string(),label:"Delete".to_string(),enabled:true,target_route:None,destructive:true}
    ]
}

pub fn render_model(component:&GeneralComponent)->Vec<(String,String)>{
    logging::debug("ui.general", "render_model_compat", "compat render");
    component.render_model().unwrap_or_else(|e|vec![("error".to_string(),e)])
}
