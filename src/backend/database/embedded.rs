use std::path::PathBuf;
use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct EmbeddedMariaDB {
    pub data_dir: PathBuf,
    pub port: u16,
    pub running: bool,
}

impl EmbeddedMariaDB {
    pub fn new(data_dir: PathBuf, port: u16) -> Self {
        logging::debug("auto.embedded", "new", "enter");
        Self {
            data_dir,
            port,
            running: false,
        }
    }
    pub fn start(&mut self) -> Result<(), String> {
        logging::debug("auto.embedded", "start", "enter");
        // embedded-mariadb.js 負責啟停內嵌 MariaDB；Rust 版保留狀態機，供 setup-database 呼叫。
        std::fs::create_dir_all(&self.data_dir)
            .map_err(|e| format!("create mariadb dir failed: {e}"))?;
        self.running = true;
        Ok(())
    }
    pub fn stop(&mut self) {
        logging::debug("auto.embedded", "stop", "enter");
        self.running = false;
    }
    pub fn dsn(&self) -> String {
        logging::debug("auto.embedded", "dsn", "enter");
        format!("mysql://127.0.0.1:{}/uptime_kuma", self.port)
    }
}

#[derive(Debug, Clone)]
pub struct EmbeddedContext {
    pub data_dir: PathBuf,
    pub port: u16,
}

pub fn embedded_run(ctx: &EmbeddedContext) -> Result<EmbeddedMariaDB, String> {
    logging::debug("auto.embedded", "embedded_run", "enter");
    let mut db = EmbeddedMariaDB::new(ctx.data_dir.clone(), ctx.port);
    db.start()?;
    Ok(db)
}
