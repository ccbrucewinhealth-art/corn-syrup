use std::collections::BTreeMap;
use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct JobDescriptor {
    pub name: String,
    pub enabled: bool,
    pub interval_seconds: u64,
}

#[derive(Debug, Clone, Default)]
pub struct JobRegistry {
    pub jobs: BTreeMap<String, JobDescriptor>,
    pub running: bool,
}

impl JobRegistry {
    pub fn init_background_jobs() -> Self {
        logging::debug("auto.mod", "init_background_jobs", "enter");
        let mut reg = Self {
            jobs: BTreeMap::new(),
            running: true,
        };
        reg.register("clear-old-data", 3600);
        reg.register("incremental-vacuum", 24 * 3600);
        reg
    }
    pub fn register(&mut self, name: &str, interval_seconds: u64) {
        logging::debug("auto.mod", "register", "enter");
        self.jobs.insert(
            name.to_string(),
            JobDescriptor {
                name: name.to_string(),
                enabled: true,
                interval_seconds,
            },
        );
    }
    pub fn stop_background_jobs(&mut self) {
        logging::debug("auto.mod", "stop_background_jobs", "enter");
        self.running = false;
    }
}

#[derive(Debug, Clone, Default)]
pub struct ModContext;

pub fn mod_run(_: &ModContext) -> Result<JobRegistry, String> {
    logging::debug("auto.mod", "mod_run", "enter");
    Ok(JobRegistry::init_background_jobs())
}
