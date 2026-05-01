use crate::backend::config::{load_config, AppConfig};
use crate::backend::database::{build_knex_like_config, init_data_dir, DatabaseContext, DatabasePaths, DatabaseRuntime, MigrationPlan};
use crate::backend::settings::{InMemorySettingsStore, SettingsStore};
use crate::backend::util::{allow_dev_all_origin_headers, init_jwt_secret, setting};
use std::path::PathBuf;
use crate::backend::logging;

mod backend;

#[derive(Debug, Clone)]
pub struct MainContext {
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
}

impl Default for MainContext {
    fn default() -> Self {
        Self {
            args: Vec::new(),
            env: std::env::vars().collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BootstrapResult {
    pub config: AppConfig,
    pub data_dir: PathBuf,
    pub database_paths: DatabasePaths,
    pub entry_page: String,
    pub jwt_secret: String,
    pub middleware_headers: Vec<(String, String)>,
    pub process_title: String,
    pub node_env: String,
    pub ws_origin_check: String,
    pub database_config: Vec<(String, String)>,
    pub migration_plan: MigrationPlan,
    pub startup_steps: Vec<String>,
}

fn env_value(env: &[(String, String)], key: &str, default_value: &str) -> String {
    env.iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.clone())
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| default_value.to_string())
}

pub fn main_run(ctx: &MainContext) -> Result<BootstrapResult, String> {
    logging::debug("auto.main", "main_run", "enter");
    let config = load_config(&ctx.args, &ctx.env)?;
    let node_env = env_value(&ctx.env, "NODE_ENV", "production");
    let ws_origin_check = env_value(&ctx.env, "UPTIME_KUMA_WS_ORIGIN_CHECK", "cors-like");

    let db_ctx = DatabaseContext {
        data_dir: config.data_dir.clone(),
    };
    let database_paths = init_data_dir(&db_ctx)?;
    let database_runtime = DatabaseRuntime::sqlite(&db_ctx)?;
    let database_config = build_knex_like_config(&database_runtime);
    let migration_plan = database_runtime.migration_plan();

    let store = InMemorySettingsStore::default();
    store.set_typed(
        "entryPage",
        "dashboard".to_string(),
        Some("general".to_string()),
    );
    let jwt_secret = init_jwt_secret(&store);
    let entry_page = setting(&store, "entryPage", "dashboard");

    let mut middleware_headers = vec![("X-Powered-By".to_string(), String::new())];
    if !config.disable_frame_same_origin {
        middleware_headers.push(("X-Frame-Options".to_string(), "SAMEORIGIN".to_string()));
    }
    if node_env == "development" {
        middleware_headers.extend(allow_dev_all_origin_headers());
    }

    let startup_steps = vec![
        "load-dotenv-and-config".to_string(),
        "check-runtime-version".to_string(),
        "parse-arguments".to_string(),
        "init-data-dir".to_string(),
        "init-database-runtime".to_string(),
        "init-settings-cache".to_string(),
        "init-jwt-secret".to_string(),
        "configure-middlewares".to_string(),
        "register-routes-socket-jobs".to_string(),
    ];

    Ok(BootstrapResult {
        config,
        data_dir: database_paths.data_dir.clone(),
        database_paths,
        entry_page,
        jwt_secret,
        middleware_headers,
        process_title: "uptime-kuma".to_string(),
        node_env,
        ws_origin_check,
        database_config,
        migration_plan,
        startup_steps,
    })
}

fn main() {
    if let Err(err) = main_run(&Default::default()) {
        eprintln!("startup failed: {}", err);
        std::process::exit(1);
    }
}
