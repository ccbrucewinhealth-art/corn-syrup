use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct TranslatableError {
    pub key: String,
    pub meta: BTreeMap<String, String>,
    pub msg_i18n: bool,
}

impl TranslatableError {
    pub fn new(key: impl Into<String>, meta: BTreeMap<String, String>) -> Self {
        logging::debug("auto.mod", "new", "enter");
        // 對齊 translatable-error.js：message 本身是 i18n key，並以 msgi18n=true 提醒前端翻譯。
        Self {
            key: key.into(),
            meta,
            msg_i18n: true,
        }
    }
}

impl Display for TranslatableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl Error for TranslatableError {}

#[derive(Debug, Clone)]
pub struct ModContext {
    pub key: String,
}

pub fn mod_run(ctx: &ModContext) -> Result<TranslatableError, String> {
    logging::debug("auto.mod", "mod_run", "enter");
    if ctx.key.trim().is_empty() {
        return Err("translation key is required".to_string());
    }
    Ok(TranslatableError::new(ctx.key.clone(), BTreeMap::new()))
}
