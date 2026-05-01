use std::time::{Duration, Instant};
use crate::backend::logging;

#[derive(Debug, Clone)]
pub struct KumaRateLimiter {
    pub error_message: String,
    pub tokens_per_interval: f64,
    pub interval: Duration,
    pub tokens: f64,
    pub last_refill: Instant,
}

#[derive(Debug, Clone)]
pub struct RateLimitDecision {
    pub ok: bool,
    pub remaining_requests: f64,
    pub msg: Option<String>,
}

impl KumaRateLimiter {
    pub fn new(
        tokens_per_interval: u32,
        interval: Duration,
        error_message: impl Into<String>,
    ) -> Self {
        logging::debug("auto.rate_limiter", "new", "enter");
        Self {
            error_message: error_message.into(),
            tokens_per_interval: tokens_per_interval as f64,
            interval,
            tokens: tokens_per_interval as f64,
            last_refill: Instant::now(),
        }
    }

    fn refill(&mut self) {
        let elapsed = self.last_refill.elapsed();
        if elapsed >= self.interval {
            let intervals = elapsed.as_secs_f64() / self.interval.as_secs_f64();
            self.tokens =
                (self.tokens + self.tokens_per_interval * intervals).min(self.tokens_per_interval);
            self.last_refill = Instant::now();
        }
    }

    pub fn remove_tokens(&mut self, num: f64) -> f64 {
        logging::debug("auto.rate_limiter", "remove_tokens", "enter");
        // 對齊 limiter.removeTokens(num)：每次 pass 前先補充 interval 內可用 token，再扣除本次成本；
        // 若結果小於 0，代表超過限制，呼叫端應回傳 Too frequently 訊息。
        self.refill();
        self.tokens -= num;
        self.tokens
    }

    pub fn pass(&mut self, num: f64) -> RateLimitDecision {
        logging::debug("auto.rate_limiter", "pass", "enter");
        let remaining = self.remove_tokens(num);
        if remaining < 0.0 {
            RateLimitDecision {
                ok: false,
                remaining_requests: remaining,
                msg: Some(self.error_message.clone()),
            }
        } else {
            RateLimitDecision {
                ok: true,
                remaining_requests: remaining,
                msg: None,
            }
        }
    }
}

pub fn login_rate_limiter() -> KumaRateLimiter {
    logging::debug("auto.rate_limiter", "login_rate_limiter", "enter");
    KumaRateLimiter::new(
        20,
        Duration::from_secs(60),
        "Too frequently, try again later.",
    )
}

pub fn api_rate_limiter() -> KumaRateLimiter {
    logging::debug("auto.rate_limiter", "api_rate_limiter", "enter");
    KumaRateLimiter::new(
        60,
        Duration::from_secs(60),
        "Too frequently, try again later.",
    )
}

pub fn two_fa_rate_limiter() -> KumaRateLimiter {
    logging::debug("auto.rate_limiter", "two_fa_rate_limiter", "enter");
    KumaRateLimiter::new(
        30,
        Duration::from_secs(60),
        "Too frequently, try again later.",
    )
}

#[derive(Debug, Clone)]
pub struct RateLimiterContext {
    pub kind: String,
}

pub fn rate_limiter_run(ctx: &RateLimiterContext) -> Result<KumaRateLimiter, String> {
    logging::debug("auto.rate_limiter", "rate_limiter_run", "enter");
    match ctx.kind.as_str() {
        "login" => Ok(login_rate_limiter()),
        "api" => Ok(api_rate_limiter()),
        "2fa" => Ok(two_fa_rate_limiter()),
        _ => Err(format!("unknown rate limiter kind: {}", ctx.kind)),
    }
}
