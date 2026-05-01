// checklist translated from Pushover.vue
// 修正說明：通知元件改為 provider-specific schema，補足必填驗證、payload mapping 與敏感欄位遮罩。
use crate::backend::logging;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationField { pub name:String, pub label:String, pub required:bool, pub value:Option<String>, pub field_type:String, pub sensitive:bool }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationPayload { pub provider:String, pub fields:Vec<(String,String)>, pub valid:bool }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PushoverComponent { pub source_path:&'static str, pub provider:String, pub title:String, pub fields:Vec<NotificationField>, pub errors:Vec<String> }

impl Default for PushoverComponent {
    fn default()->Self {
        logging::debug("notification-ui.pushover","default","init provider-specific notification form");
        Self{source_path:"Pushover.vue",provider:"pushover".to_string(),title:"Pushover".to_string(),fields:default_fields(),errors:Vec::new()}
    }
}

impl PushoverComponent {
    pub fn validate(&self)->Result<(),String>{
        logging::debug("notification-ui.pushover","validate",format!("fields={}",self.fields.len()));
        for f in &self.fields { if f.required && f.value.as_deref().unwrap_or_default().trim().is_empty() { return Err(format!("field {} is required",f.name)); } }
        Ok(())
    }
    pub fn set_field(&mut self,name:&str,value:impl Into<String>)->Result<(),String>{
        logging::debug("notification-ui.pushover","set_field",name);
        let f=self.fields.iter_mut().find(|f|f.name==name).ok_or_else(||format!("field {name} not found"))?; f.value=Some(value.into()); Ok(())
    }
    pub fn build_payload(&self)->Result<NotificationPayload,String>{
        logging::debug("notification-ui.pushover","build_payload","compose provider config payload"); self.validate()?;
        Ok(NotificationPayload{provider:self.provider.clone(),fields:self.fields.iter().map(|f|(f.name.clone(),f.value.clone().unwrap_or_default())).collect(),valid:true})
    }
    pub fn render_model(&self)->Vec<(String,String)>{
        logging::debug("notification-ui.pushover","render_model","render notification form with masked secrets");
        let mut rows=vec![("title".to_string(),self.title.clone()),("provider".to_string(),self.provider.clone())];
        for f in &self.fields { let value=if f.sensitive && f.value.is_some() {"********".to_string()} else {f.value.clone().unwrap_or_default()}; rows.push((format!("field:{}",f.name),format!("{}|type={}|required={}|value={}",f.label,f.field_type,f.required,value))); } rows
    }
}

pub fn default_fields()->Vec<NotificationField>{
    logging::debug("notification-ui.pushover","default_fields","build provider-specific fields");
    vec![
        NotificationField{name:"user_key".to_string(),label:"User Key".to_string(),required:true,value:None,field_type:"password".to_string(),sensitive:true},
        NotificationField{name:"api_token".to_string(),label:"API Token".to_string(),required:true,value:None,field_type:"password".to_string(),sensitive:true},
        NotificationField{name:"device".to_string(),label:"Device".to_string(),required:false,value:None,field_type:"text".to_string(),sensitive:false}
    ]
}

pub fn render_model(component:&PushoverComponent)->Vec<(String,String)>{ logging::debug("notification-ui.pushover","render_model_compat","compat render"); component.render_model() }
