use crate::backend::logging;
#[derive(Debug, Clone)]
pub struct RemoteBrowser {
    pub id: u64,
    pub name: String,
    pub url: String,
}

impl RemoteBrowser {
    pub fn validate(&self) -> Result<(), String> {
        logging::debug("auto.mod", "validate", "enter");
        // remote-browser.js 主要管理遠端瀏覽器 endpoint；這裡確保名稱與 ws/http endpoint 可用。
        if self.name.trim().is_empty() {
            return Err("remote browser name is required".to_string());
        }
        if !(self.url.starts_with("ws://")
            || self.url.starts_with("wss://")
            || self.url.starts_with("http://")
            || self.url.starts_with("https://"))
        {
            return Err("remote browser url must be ws/wss/http/https".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ModContext {
    pub id: u64,
    pub name: String,
    pub url: String,
}

pub fn mod_run(ctx: &ModContext) -> Result<RemoteBrowser, String> {
    logging::debug("auto.mod", "mod_run", "enter");
    let browser = RemoteBrowser {
        id: ctx.id,
        name: ctx.name.clone(),
        url: ctx.url.clone(),
    };
    browser.validate()?;
    Ok(browser)
}
