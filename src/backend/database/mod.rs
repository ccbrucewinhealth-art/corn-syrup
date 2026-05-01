use std::fs;
use std::path::{Path, PathBuf};
use crate::backend::logging;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseDialect {
    Sqlite,
    MariaDb,
    Mysql,
}

#[derive(Debug, Clone)]
pub struct DatabaseContext {
    pub data_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DatabasePaths {
    pub data_dir: PathBuf,
    pub sqlite_path: PathBuf,
    pub upload_dir: PathBuf,
    pub screenshot_dir: PathBuf,
    pub docker_tls_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DatabaseRuntime {
    pub dialect: DatabaseDialect,
    pub paths: DatabasePaths,
    pub template_path: PathBuf,
    pub patched: bool,
    pub readonly: bool,
    pub migration_table: String,
}

#[derive(Debug, Clone)]
pub struct MigrationPlan {
    pub backup_before_migration: bool,
    pub migration_table: String,
    pub steps: Vec<String>,
}

impl DatabasePaths {
    pub fn path_for_upload(&self, name: impl AsRef<Path>) -> PathBuf {
        logging::debug("auto.mod", "path_for_upload", "enter");
        self.upload_dir.join(name)
    }

    pub fn path_for_screenshot(&self, name: impl AsRef<Path>) -> PathBuf {
        logging::debug("auto.mod", "path_for_screenshot", "enter");
        self.screenshot_dir.join(name)
    }

    pub fn path_for_docker_tls(&self, name: impl AsRef<Path>) -> PathBuf {
        logging::debug("auto.mod", "path_for_docker_tls", "enter");
        self.docker_tls_dir.join(name)
    }
}

impl DatabaseRuntime {
    pub fn sqlite(ctx: &DatabaseContext) -> Result<Self, String> {
        logging::debug("auto.mod", "sqlite", "enter");
        let paths = init_data_dir(ctx)?;
        Ok(Self {
            dialect: DatabaseDialect::Sqlite,
            paths,
            template_path: PathBuf::from("./db/kuma.db"),
            patched: false,
            readonly: false,
            migration_table: "patch_list".to_string(),
        })
    }

    pub fn connection_label(&self) -> String {
        logging::debug("auto.mod", "connection_label", "enter");
        match self.dialect {
            DatabaseDialect::Sqlite => format!("sqlite:{}", self.paths.sqlite_path.display()),
            DatabaseDialect::MariaDb => "mariadb".to_string(),
            DatabaseDialect::Mysql => "mysql".to_string(),
        }
    }

    pub fn migration_plan(&self) -> MigrationPlan {
        logging::debug("auto.mod", "migration_plan", "enter");
        MigrationPlan {
            backup_before_migration: matches!(self.dialect, DatabaseDialect::Sqlite),
            migration_table: self.migration_table.clone(),
            steps: vec![
                "ensure-data-dir".to_string(),
                "connect-database".to_string(),
                "run-simple-migration-server".to_string(),
                "load-settings-cache".to_string(),
                "init-uptime-calculator".to_string(),
            ],
        }
    }
}

pub fn init_data_dir(ctx: &DatabaseContext) -> Result<DatabasePaths, String> {
    logging::debug("auto.mod", "init_data_dir", "enter");
    let upload = ctx.data_dir.join("upload");
    let screenshots = ctx.data_dir.join("screenshots");
    let docker_tls = ctx.data_dir.join("docker-tls");
    let sqlite_path = ctx.data_dir.join("kuma.db");

    // 對齊 database.js initDataDir()：先決定 dataDir/sqlitePath，再確保 upload、screenshots、docker-tls 存在。
    fs::create_dir_all(&ctx.data_dir).map_err(|e| format!("create data dir failed: {e}"))?;
    fs::create_dir_all(&upload).map_err(|e| format!("create upload dir failed: {e}"))?;
    fs::create_dir_all(&screenshots).map_err(|e| format!("create screenshots dir failed: {e}"))?;
    fs::create_dir_all(&docker_tls).map_err(|e| format!("create docker-tls dir failed: {e}"))?;

    Ok(DatabasePaths {
        data_dir: ctx.data_dir.clone(),
        sqlite_path,
        upload_dir: upload,
        screenshot_dir: screenshots,
        docker_tls_dir: docker_tls,
    })
}

pub fn should_copy_sqlite_template(paths: &DatabasePaths) -> bool {
    logging::debug("auto.mod", "should_copy_sqlite_template", "enter");
    !paths.sqlite_path.exists()
}

pub fn build_knex_like_config(runtime: &DatabaseRuntime) -> Vec<(String, String)> {
    logging::debug("auto.mod", "build_knex_like_config", "enter");
    let mut cfg = Vec::new();
    cfg.push(("client".to_string(), runtime.connection_label()));
    cfg.push(("migrationTable".to_string(), runtime.migration_table.clone()));
    cfg.push(("dataDir".to_string(), runtime.paths.data_dir.display().to_string()));
    cfg.push(("uploadDir".to_string(), runtime.paths.upload_dir.display().to_string()));
    cfg.push(("screenshotDir".to_string(), runtime.paths.screenshot_dir.display().to_string()));
    cfg.push(("dockerTLSDir".to_string(), runtime.paths.docker_tls_dir.display().to_string()));
    cfg
}
