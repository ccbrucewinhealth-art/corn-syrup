use std::collections::BTreeMap;
use crate::backend::logging;

pub const UP: i32 = 1;
pub const DOWN: i32 = 0;
pub const PENDING: i32 = 2;
pub const MAINTENANCE: i32 = 3;

#[derive(Debug, Clone, Default)]
pub struct UptimeBucket {
    pub up: u64,
    pub down: u64,
    pub avg_ping: f64,
    pub min_ping: f64,
    pub max_ping: f64,
}

#[derive(Debug, Clone)]
pub struct UptimeCalculator {
    pub monitor_id: u64,
    pub minutely: BTreeMap<i64, UptimeBucket>,
    pub hourly: BTreeMap<i64, UptimeBucket>,
    pub daily: BTreeMap<i64, UptimeBucket>,
    pub migration_mode: bool,
    pub stat_minutely_keep_hour: i64,
    pub stat_hourly_keep_day: i64,
}

impl UptimeCalculator {
    pub fn new(monitor_id: u64) -> Result<Self, String> {
        logging::debug("auto.uptime", "new", "enter");
        if monitor_id == 0 {
            return Err("Monitor ID is required".to_string());
        }
        Ok(Self {
            monitor_id,
            minutely: BTreeMap::new(),
            hourly: BTreeMap::new(),
            daily: BTreeMap::new(),
            migration_mode: false,
            stat_minutely_keep_hour: 24,
            stat_hourly_keep_day: 30,
        })
    }

    pub fn get_minutely_key(timestamp_seconds: i64) -> i64 {
        logging::debug("auto.uptime", "get_minutely_key", "enter");
        timestamp_seconds - timestamp_seconds.rem_euclid(60)
    }

    pub fn get_hourly_key(timestamp_seconds: i64) -> i64 {
        logging::debug("auto.uptime", "get_hourly_key", "enter");
        timestamp_seconds - timestamp_seconds.rem_euclid(60 * 60)
    }

    pub fn get_daily_key(timestamp_seconds: i64) -> i64 {
        logging::debug("auto.uptime", "get_daily_key", "enter");
        timestamp_seconds - timestamp_seconds.rem_euclid(24 * 60 * 60)
    }

    pub fn flat_status(status: i32) -> Result<i32, String> {
        logging::debug("auto.uptime", "flat_status", "enter");
        match status {
            UP | MAINTENANCE => Ok(UP),
            DOWN | PENDING => Ok(DOWN),
            _ => Err("Invalid status".to_string()),
        }
    }

    fn apply_bucket(bucket: &mut UptimeBucket, flat_status: i32, ping: f64) {
        if flat_status == UP {
            bucket.up += 1;
            if ping > 0.0 {
                // 對齊 uptime-calculator.js：第一筆 UP ping 直接成為 avg/min/max；
                // 後續以「舊平均 * (up - 1) + 新 ping」除以 up 做累進平均，不需保存所有 heartbeat。
                if bucket.up == 1 {
                    bucket.avg_ping = ping;
                    bucket.min_ping = ping;
                    bucket.max_ping = ping;
                } else {
                    bucket.avg_ping =
                        (bucket.avg_ping * (bucket.up - 1) as f64 + ping) / bucket.up as f64;
                    bucket.min_ping = bucket.min_ping.min(ping);
                    bucket.max_ping = bucket.max_ping.max(ping);
                }
            }
        } else {
            bucket.down += 1;
        }
    }

    pub fn update(
        &mut self,
        status: i32,
        ping: f64,
        timestamp_seconds: i64,
    ) -> Result<i64, String> {
        logging::debug("auto.uptime", "update", "enter");
        let flat_status = Self::flat_status(status)?;
        let minute_key = Self::get_minutely_key(timestamp_seconds);
        let hour_key = Self::get_hourly_key(timestamp_seconds);
        let day_key = Self::get_daily_key(timestamp_seconds);

        if status == MAINTENANCE {
            return Ok(timestamp_seconds);
        }

        Self::apply_bucket(
            self.minutely.entry(minute_key).or_default(),
            flat_status,
            ping,
        );
        Self::apply_bucket(self.hourly.entry(hour_key).or_default(), flat_status, ping);
        Self::apply_bucket(self.daily.entry(day_key).or_default(), flat_status, ping);

        Ok(timestamp_seconds)
    }

    pub fn uptime_ratio(bucket: &UptimeBucket) -> Option<f64> {
        logging::debug("auto.uptime", "uptime_ratio", "enter");
        let total = bucket.up + bucket.down;
        if total == 0 {
            None
        } else {
            Some(bucket.up as f64 / total as f64)
        }
    }
}

#[derive(Debug, Clone)]
pub struct UptimeContext {
    pub monitor_id: u64,
    pub status: i32,
    pub ping: f64,
    pub timestamp_seconds: i64,
}

pub fn uptime_run(ctx: &UptimeContext) -> Result<UptimeCalculator, String> {
    logging::debug("auto.uptime", "uptime_run", "enter");
    let mut calculator = UptimeCalculator::new(ctx.monitor_id)?;
    calculator.update(ctx.status, ctx.ping, ctx.timestamp_seconds)?;
    Ok(calculator)
}
