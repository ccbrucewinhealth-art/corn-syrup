use std::collections::BTreeMap;
use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct RadiusRequest {
    pub host: String,
    pub port: u16,
    pub secret: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct RadiusPacket {
    pub code: String,
    pub attributes: BTreeMap<String, String>,
}

impl RadiusRequest {
    pub fn build_access_request(&self) -> Result<RadiusPacket, String> {
        logging::debug("auto.client", "build_access_request", "enter");
        if self.host.is_empty() || self.secret.is_empty() {
            return Err("radius host/secret required".to_string());
        }
        let mut attributes = BTreeMap::new();
        attributes.insert("User-Name".to_string(), self.username.clone());
        attributes.insert("User-Password".to_string(), self.password.clone());
        Ok(RadiusPacket {
            code: "Access-Request".to_string(),
            attributes,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClientContext {
    pub request: RadiusRequest,
}

pub fn client_run(ctx: &ClientContext) -> Result<RadiusPacket, String> {
    logging::debug("auto.client", "client_run", "enter");
    ctx.request.build_access_request()
}
