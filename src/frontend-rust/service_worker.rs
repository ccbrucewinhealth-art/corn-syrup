// checklist No.470 translated from public/serviceWorker.js
// 修正說明：補足 Service Worker install/activate/fetch/cache 行為模型，不再以通用 UI 表單替代。
use crate::backend::logging;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceWorkerEvent { Install, Activate, Fetch(String), Message(String), ClearCache }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheStrategy { CacheFirst, NetworkFirst, StaleWhileRevalidate }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedAsset { pub url:String, pub revision:String, pub immutable:bool }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceWorkerComponent { pub source_path:&'static str, pub cache_name:String, pub strategy:CacheStrategy, pub precache:Vec<CachedAsset>, pub runtime_cache:Vec<String>, pub emitted:Vec<String>, pub activated:bool }

impl Default for ServiceWorkerComponent {
    fn default()->Self { logging::debug("service_worker", "default", "init service worker cache model"); Self{source_path:"public/serviceWorker.js",cache_name:"corn-syrup-v1".to_string(),strategy:CacheStrategy::StaleWhileRevalidate,precache:default_precache(),runtime_cache:Vec::new(),emitted:Vec::new(),activated:false} }
}

impl ServiceWorkerComponent {
    pub fn validate(&self)->Result<(),String>{ logging::debug("service_worker", "validate", format!("precache={}", self.precache.len())); if self.cache_name.trim().is_empty(){return Err("cache name required".to_string())} if self.precache.is_empty(){return Err("precache assets required".to_string())} Ok(()) }
    pub fn dispatch_event(&mut self,event:ServiceWorkerEvent)->Result<String,String>{
        logging::debug("service_worker", "dispatch_event", format!("{:?}",event));
        match event{
            ServiceWorkerEvent::Install=>{ self.validate()?; self.emitted.push("install:precache".to_string()); Ok(format!("precache:{}", self.precache.len())) }
            ServiceWorkerEvent::Activate=>{ self.activated=true; self.emitted.push("activate:cleanup-old-caches".to_string()); Ok("activated".to_string()) }
            ServiceWorkerEvent::Fetch(url)=>{ let plan=self.fetch_plan(&url); self.runtime_cache.push(url); self.emitted.push("fetch".to_string()); Ok(plan) }
            ServiceWorkerEvent::Message(msg)=>{ self.emitted.push(format!("message:{msg}")); Ok("message accepted".to_string()) }
            ServiceWorkerEvent::ClearCache=>{ self.runtime_cache.clear(); self.emitted.push("clear-cache".to_string()); Ok("cache cleared".to_string()) }
        }
    }
    pub fn fetch_plan(&self,url:&str)->String{
        let same_origin = url.starts_with('/') || url.contains("corn-syrup");
        match self.strategy { CacheStrategy::CacheFirst if same_origin => format!("cache-first:{url}"), CacheStrategy::NetworkFirst => format!("network-first:{url}"), _ => format!("stale-while-revalidate:{url}") }
    }
    pub fn render_model(&self)->Vec<(String,String)>{
        logging::debug("service_worker", "render_model", format!("runtime={}", self.runtime_cache.len()));
        let mut rows=vec![("cache_name".to_string(),self.cache_name.clone()),("activated".to_string(),self.activated.to_string()),("strategy".to_string(),format!("{:?}",self.strategy))];
        for a in &self.precache { rows.push((format!("precache:{}",a.url), format!("revision={}|immutable={}",a.revision,a.immutable))); }
        for u in &self.runtime_cache { rows.push(("runtime_cache".to_string(),u.clone())); }
        rows
    }
}

pub fn default_precache()->Vec<CachedAsset>{ logging::debug("service_worker", "default_precache", "build static assets"); vec![CachedAsset{url:"/".to_string(),revision:"app-shell".to_string(),immutable:false},CachedAsset{url:"/icon.svg".to_string(),revision:"asset".to_string(),immutable:true},CachedAsset{url:"/favicon.ico".to_string(),revision:"asset".to_string(),immutable:true}] }
pub fn render_model(component:&ServiceWorkerComponent)->Vec<(String,String)>{ component.render_model() }
