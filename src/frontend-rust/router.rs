// checklist No.252 translated from backend/router.js
// 修正說明：補足實際前端 route table、巢狀設定路由、公開狀態頁與 404 fallback，不再使用單一 /router 佔位元件。
use crate::backend::logging;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteGuard { Public, Authenticated, SetupOnly }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteDefinition {
    pub name: &'static str,
    pub path: &'static str,
    pub component: &'static str,
    pub layout: &'static str,
    pub guard: RouteGuard,
    pub title_key: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouterComponent { pub source_path:&'static str, pub current_path:String, pub routes:Vec<RouteDefinition>, pub fallback:RouteDefinition, pub error:Option<String> }

impl Default for RouterComponent {
    fn default() -> Self {
        logging::debug("frontend.router", "default", "init route table");
        Self { source_path:"backend/router.js", current_path:"/dashboard".to_string(), routes:default_routes(), fallback:not_found_route(), error:None }
    }
}

impl RouterComponent {
    pub fn validate(&self)->Result<(),String>{
        logging::debug("frontend.router", "validate", format!("routes={}", self.routes.len()));
        if self.routes.is_empty(){ return Err("route table is empty".to_string()); }
        if !self.routes.iter().any(|r| r.path == "/dashboard") { return Err("dashboard route missing".to_string()); }
        if !self.routes.iter().any(|r| r.path == "/setup") { return Err("setup route missing".to_string()); }
        Ok(())
    }
    pub fn apply_route(&mut self, route: impl Into<String>) -> Result<(), String> {
        let route=route.into(); logging::debug("frontend.router", "apply_route", &route);
        if route.trim().is_empty(){ return Err("route is required".to_string()); }
        self.current_path=route; Ok(())
    }
    pub fn resolve(&self, path:&str)->&RouteDefinition{
        logging::debug("frontend.router", "resolve", path);
        self.routes.iter().find(|r| r.path == path || (r.path.ends_with(":slug") && path.starts_with("/status/"))).unwrap_or(&self.fallback)
    }
    pub fn breadcrumbs(&self, path:&str)->Vec<&'static str>{
        let r=self.resolve(path);
        if r.path.starts_with("/settings") { vec!["dashboard", "settings", r.title_key] } else { vec![r.title_key] }
    }
    pub fn render_model(&self)->Result<Vec<(String,String)>,String>{
        self.validate()?; logging::debug("frontend.router", "render_model", format!("current={}", self.current_path));
        let mut rows=vec![("current".to_string(), self.current_path.clone())];
        for r in &self.routes { rows.push((format!("route:{}", r.name), format!("{}|component={}|layout={}|guard={:?}|title={}", r.path, r.component, r.layout, r.guard, r.title_key))); }
        rows.push(("fallback".to_string(), format!("{}|component={}", self.fallback.path, self.fallback.component)));
        Ok(rows)
    }
}

pub fn default_routes()->Vec<RouteDefinition>{
    logging::debug("frontend.router", "default_routes", "build uptime-kuma compatible route table");
    vec![
        RouteDefinition{name:"entry",path:"/",component:"EntryPage",layout:"EmptyLayout",guard:RouteGuard::Public,title_key:"entry"},
        RouteDefinition{name:"setup",path:"/setup",component:"SetupPage",layout:"EmptyLayout",guard:RouteGuard::SetupOnly,title_key:"setup"},
        RouteDefinition{name:"setup_database",path:"/setup-database",component:"SetupDatabasePage",layout:"EmptyLayout",guard:RouteGuard::SetupOnly,title_key:"setup_database"},
        RouteDefinition{name:"dashboard",path:"/dashboard",component:"DashboardPage",layout:"Layout",guard:RouteGuard::Authenticated,title_key:"dashboard"},
        RouteDefinition{name:"dashboard_home",path:"/dashboard/home",component:"DashboardHomePage",layout:"Layout",guard:RouteGuard::Authenticated,title_key:"dashboard_home"},
        RouteDefinition{name:"list",path:"/list",component:"ListPage",layout:"Layout",guard:RouteGuard::Authenticated,title_key:"monitor_list"},
        RouteDefinition{name:"details",path:"/details",component:"DetailsPage",layout:"Layout",guard:RouteGuard::Authenticated,title_key:"details"},
        RouteDefinition{name:"edit_monitor",path:"/edit-monitor",component:"EditMonitorPage",layout:"Layout",guard:RouteGuard::Authenticated,title_key:"edit_monitor"},
        RouteDefinition{name:"edit_maintenance",path:"/edit-maintenance",component:"EditMaintenancePage",layout:"Layout",guard:RouteGuard::Authenticated,title_key:"edit_maintenance"},
        RouteDefinition{name:"manage_maintenance",path:"/manage-maintenance",component:"ManageMaintenancePage",layout:"Layout",guard:RouteGuard::Authenticated,title_key:"manage_maintenance"},
        RouteDefinition{name:"settings",path:"/settings",component:"SettingsPage",layout:"Layout",guard:RouteGuard::Authenticated,title_key:"settings"},
        RouteDefinition{name:"status_page",path:"/status/:slug",component:"StatusPage",layout:"EmptyLayout",guard:RouteGuard::Public,title_key:"status_page"},
        RouteDefinition{name:"manage_status_page",path:"/manage-status-page",component:"ManageStatusPage",layout:"Layout",guard:RouteGuard::Authenticated,title_key:"manage_status_page"},
        RouteDefinition{name:"add_status_page",path:"/add-status-page",component:"AddStatusPage",layout:"Layout",guard:RouteGuard::Authenticated,title_key:"add_status_page"},
    ]
}

pub fn not_found_route()->RouteDefinition{ RouteDefinition{name:"not_found",path:"*",component:"NotFoundPage",layout:"EmptyLayout",guard:RouteGuard::Public,title_key:"not_found"} }
pub fn render_model(component:&RouterComponent)->Vec<(String,String)>{ logging::debug("frontend.router", "render_model_compat", "compat wrapper"); component.render_model().unwrap_or_else(|e|vec![("error".to_string(),e)]) }
