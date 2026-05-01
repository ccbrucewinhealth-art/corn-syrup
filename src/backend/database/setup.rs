use crate::backend::logging;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseType {
    Sqlite,
    MariaDb,
    Mysql,
}

#[derive(Debug, Clone)]
pub struct DatabaseConnectionOptions {
    pub db_type: DatabaseType,
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub database: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub use_embedded_mariadb: bool,
}

#[derive(Debug, Clone)]
pub struct SetupDatabase {
    pub selected_type: Option<DatabaseType>,
    pub setup_required: bool,
    pub connection: Option<DatabaseConnectionOptions>,
    pub messages: Vec<String>,
}

impl SetupDatabase {
    pub fn new(selected_type: Option<DatabaseType>) -> Self {
        logging::debug("auto.setup", "new", "enter");
        Self {
            setup_required: selected_type.is_none(),
            selected_type,
            connection: None,
            messages: Vec::new(),
        }
    }

    pub fn is_need_setup(&self) -> bool {
        logging::debug("auto.setup", "is_need_setup", "enter");
        self.setup_required
    }

    pub fn choose(&mut self, db_type: DatabaseType) {
        logging::debug("auto.setup", "choose", "enter");
        self.selected_type = Some(db_type);
        self.setup_required = false;
        self.messages.push("database type selected".to_string());
    }

    pub fn apply_connection(&mut self, options: DatabaseConnectionOptions) -> Result<(), String> {
        logging::debug("auto.setup", "apply_connection", "enter");
        validate_connection_options(&options)?;
        self.selected_type = Some(options.db_type.clone());
        self.connection = Some(options);
        self.setup_required = false;
        self.messages.push("database connection options accepted".to_string());
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct SetupContext {
    pub db_type: Option<DatabaseType>,
    pub force_setup: bool,
}

pub fn validate_connection_options(options: &DatabaseConnectionOptions) -> Result<(), String> {
    logging::debug("auto.setup", "validate_connection_options", "enter");
    match options.db_type {
        DatabaseType::Sqlite => Ok(()),
        DatabaseType::MariaDb | DatabaseType::Mysql => {
            if options.use_embedded_mariadb {
                return Ok(());
            }
            if options.hostname.as_deref().unwrap_or_default().trim().is_empty() {
                return Err("database hostname is required".to_string());
            }
            if options.database.as_deref().unwrap_or_default().trim().is_empty() {
                return Err("database name is required".to_string());
            }
            if options.username.as_deref().unwrap_or_default().trim().is_empty() {
                return Err("database username is required".to_string());
            }
            Ok(())
        }
    }
}

pub fn setup_run(ctx: &SetupContext) -> Result<SetupDatabase, String> {
    logging::debug("auto.setup", "setup_run", "enter");
    let mut setup = SetupDatabase::new(ctx.db_type.clone());
    if ctx.force_setup {
        setup.setup_required = true;
        setup.messages.push("setup forced by context".to_string());
    }
    Ok(setup)
}
