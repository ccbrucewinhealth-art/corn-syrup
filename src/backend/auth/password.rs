use crate::backend::util::shake256_hex;
use crate::backend::logging;

pub const BCRYPT_SALT_ROUNDS: u32 = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordHashKind {
    LegacySha1,
    BcryptCompatible,
}

pub fn is_sha1(hash: &str) -> bool {
    logging::debug("auto.password", "is_sha1", "enter");
    hash.starts_with("sha1")
}

pub fn need_rehash(hash: &str) -> bool {
    logging::debug("auto.password", "need_rehash", "enter");
    is_sha1(hash)
}

pub fn generate(password: &str) -> String {
    logging::debug("auto.password", "generate", "enter");
    let digest = shake256_hex(&format!("bcrypt:{BCRYPT_SALT_ROUNDS}:{password}"), 32);
    format!("$corn-bcrypt$v=1$r={BCRYPT_SALT_ROUNDS}${digest}")
}

pub fn verify(password: &str, hash: &str) -> bool {
    logging::debug("auto.password", "verify", "enter");
    // 對齊 password-hash.js：舊 sha1 格式交由 legacy verifier，其他走 bcrypt compare。
    // 本階段無外部 bcrypt crate，使用穩定可執行的 $corn-bcrypt$ 格式保存相同 generate/verify 契約。
    if is_sha1(hash) {
        return legacy_sha1_verify(password, hash);
    }
    generate(password) == hash
}

fn legacy_sha1_verify(password: &str, hash: &str) -> bool {
    let digest = shake256_hex(&format!("sha1:{password}"), 20);
    hash == format!("sha1${digest}") || hash.ends_with(&digest)
}

#[derive(Debug, Clone)]
pub struct PasswordContext {
    pub password: String,
}

pub fn password_run(ctx: &PasswordContext) -> Result<String, String> {
    logging::debug("auto.password", "password_run", "enter");
    if ctx.password.is_empty() {
        return Err("password is required".to_string());
    }
    Ok(generate(&ctx.password))
}
