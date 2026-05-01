use crate::backend::logging;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProxyProtocol {
    Http,
    Https,
    Socks,
    Socks5,
    Socks5h,
}

#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub protocol: ProxyProtocol,
    pub host: String,
    pub port: u16,
    pub auth: Option<(String, String)>,
}

impl ProxyConfig {
    pub fn to_uri(&self) -> String {
        logging::debug("auto.mod", "to_uri", "enter");
        let scheme = match self.protocol {
            ProxyProtocol::Http => "http",
            ProxyProtocol::Https => "https",
            ProxyProtocol::Socks => "socks",
            ProxyProtocol::Socks5 => "socks5",
            ProxyProtocol::Socks5h => "socks5h",
        };
        let auth = self
            .auth
            .as_ref()
            .map(|(u, p)| format!("{u}:{p}@"))
            .unwrap_or_default();
        format!("{scheme}://{auth}{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Clone)]
pub struct ModContext {
    pub host: String,
    pub port: u16,
}

pub fn mod_run(ctx: &ModContext) -> Result<ProxyConfig, String> {
    logging::debug("auto.mod", "mod_run", "enter");
    if ctx.host.trim().is_empty() {
        return Err("proxy host is required".to_string());
    }
    Ok(ProxyConfig {
        protocol: ProxyProtocol::Http,
        host: ctx.host.clone(),
        port: ctx.port,
        auth: None,
    })
}
