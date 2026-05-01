use crate::backend::logging;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DockerConnectionType {
    Socket,
    Tcp,
}

#[derive(Debug, Clone)]
pub struct DockerHost {
    pub name: String,
    pub connection_type: DockerConnectionType,
    pub socket_path: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub tls: bool,
    pub tls_ca: Option<String>,
    pub tls_cert: Option<String>,
    pub tls_key: Option<String>,
}

impl DockerHost {
    pub fn endpoint(&self) -> Result<String, String> {
        logging::debug("auto.mod", "endpoint", "enter");
        match self.connection_type {
            DockerConnectionType::Socket => Ok(self
                .socket_path
                .clone()
                .unwrap_or_else(|| "/var/run/docker.sock".to_string())),
            DockerConnectionType::Tcp => Ok(format!(
                "{}://{}:{}",
                if self.tls { "https" } else { "tcp" },
                self.host.clone().ok_or("docker tcp host required")?,
                self.port.unwrap_or(if self.tls { 2376 } else { 2375 })
            )),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        logging::debug("auto.mod", "validate", "enter");
        if self.name.trim().is_empty() {
            return Err("docker host name is required".to_string());
        }
        match self.connection_type {
            DockerConnectionType::Socket => {
                if self.socket_path.as_deref().unwrap_or("/var/run/docker.sock").trim().is_empty() {
                    return Err("docker socket path is required".to_string());
                }
            }
            DockerConnectionType::Tcp => {
                if self.host.as_deref().unwrap_or_default().trim().is_empty() {
                    return Err("docker tcp host is required".to_string());
                }
            }
        }
        if self.tls && (self.tls_cert.is_none() || self.tls_key.is_none()) {
            return Err("docker tls cert and key are required when tls is enabled".to_string());
        }
        Ok(())
    }

    pub fn request_options(&self) -> Result<Vec<(String, String)>, String> {
        logging::debug("auto.mod", "request_options", "enter");
        self.validate()?;
        let mut opts = vec![("endpoint".to_string(), self.endpoint()?)];
        opts.push(("type".to_string(), match self.connection_type { DockerConnectionType::Socket => "socket", DockerConnectionType::Tcp => "tcp" }.to_string()));
        opts.push(("tls".to_string(), self.tls.to_string()));
        if let Some(ca) = &self.tls_ca { opts.push(("ca".to_string(), ca.clone())); }
        if let Some(cert) = &self.tls_cert { opts.push(("cert".to_string(), cert.clone())); }
        if let Some(key) = &self.tls_key { opts.push(("key".to_string(), key.clone())); }
        Ok(opts)
    }
}

#[derive(Debug, Clone)]
pub struct ModContext {
    pub name: String,
}

pub fn mod_run(ctx: &ModContext) -> Result<DockerHost, String> {
    logging::debug("auto.mod", "mod_run", "enter");
    let host = DockerHost {
        name: ctx.name.clone(),
        connection_type: DockerConnectionType::Socket,
        socket_path: Some("/var/run/docker.sock".to_string()),
        host: None,
        port: None,
        tls: false,
        tls_ca: None,
        tls_cert: None,
        tls_key: None,
    };
    host.validate()?;
    Ok(host)
}
