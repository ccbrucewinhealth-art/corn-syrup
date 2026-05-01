use crate::backend::auth::password;
use crate::backend::util::decode_jwt_payload_unverified;
use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct UserCredential {
    pub username: String,
    pub password_hash: String,
}

pub fn login(input_password: &str, credential: &UserCredential) -> Result<bool, String> {
    logging::debug("auto.mod", "login", "enter");
    if credential.username.trim().is_empty() {
        return Err("username is required".to_string());
    }
    Ok(password::verify(input_password, &credential.password_hash))
}

pub fn api_auth(bearer_token: &str) -> Result<String, String> {
    logging::debug("auto.mod", "api_auth", "enter");
    let token = bearer_token.strip_prefix("Bearer ").unwrap_or(bearer_token);
    // auth.js API middleware 會解析 JWT/Token 並把使用者資訊掛到 request；此處回傳未驗簽 payload 供上層測試與串接。
    decode_jwt_payload_unverified(token)
}

#[derive(Debug, Clone)]
pub struct ModContext {
    pub token: String,
}

pub fn mod_run(ctx: &ModContext) -> Result<String, String> {
    logging::debug("auto.mod", "mod_run", "enter");
    api_auth(&ctx.token)
}
