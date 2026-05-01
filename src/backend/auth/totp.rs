use crate::backend::util::shake256_hex;
use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct TotpConfig {
    pub secret: String,
    pub step_seconds: u64,
    pub digits: usize,
    pub window: i64,
}

impl Default for TotpConfig {
    fn default() -> Self {
        Self {
            secret: String::new(),
            step_seconds: 30,
            digits: 6,
            window: 1,
        }
    }
}

pub fn generate_totp(secret: &str, unix_seconds: u64, step_seconds: u64, digits: usize) -> String {
    logging::debug("auto.totp", "generate_totp", "enter");
    let counter = unix_seconds / step_seconds.max(1);
    let digest = shake256_hex(&format!("totp:{secret}:{counter}"), 8);
    let number = u64::from_str_radix(&digest[..12.min(digest.len())], 16).unwrap_or(0)
        % 10u64.pow(digits as u32);
    format!("{number:0digits$}")
}

pub fn verify_totp(token: &str, config: &TotpConfig, unix_seconds: u64) -> bool {
    logging::debug("auto.totp", "verify_totp", "enter");
    // 2fa.js/notp 預設 window=1、time=30；演算法檢查目前 counter 前後 window 範圍。
    for offset in -config.window..=config.window {
        let shifted = if offset.is_negative() {
            unix_seconds.saturating_sub((-offset as u64) * config.step_seconds)
        } else {
            unix_seconds + offset as u64 * config.step_seconds
        };
        if generate_totp(&config.secret, shifted, config.step_seconds, config.digits) == token {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone)]
pub struct TotpContext {
    pub token: String,
    pub config: TotpConfig,
    pub unix_seconds: u64,
}

pub fn totp_run(ctx: &TotpContext) -> Result<bool, String> {
    logging::debug("auto.totp", "totp_run", "enter");
    Ok(verify_totp(&ctx.token, &ctx.config, ctx.unix_seconds))
}
