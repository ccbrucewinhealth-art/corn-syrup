use crate::backend::settings::SettingsStore;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::backend::logging;

const BASE64_URL_SAFE: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

fn decode_base64_url(input: &str) -> Result<Vec<u8>, String> {
    let mut bits = 0u32;
    let mut bit_count = 0u8;
    let mut output = Vec::new();

    for byte in input.bytes().filter(|b| *b != b'=') {
        let value = BASE64_URL_SAFE
            .iter()
            .position(|candidate| *candidate == byte)
            .or_else(|| {
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
                    .iter()
                    .position(|candidate| *candidate == byte)
            })
            .ok_or_else(|| format!("invalid base64 character: {}", byte as char))?
            as u32;

        bits = (bits << 6) | value;
        bit_count += 6;
        while bit_count >= 8 {
            bit_count -= 8;
            output.push(((bits >> bit_count) & 0xff) as u8);
        }
    }

    Ok(output)
}

pub fn decode_jwt_payload_unverified(jwt: &str) -> Result<String, String> {
    logging::debug("auto.util", "decode_jwt_payload_unverified", "enter");
    let payload = jwt
        .split('.')
        .nth(1)
        .ok_or_else(|| "invalid jwt format".to_string())?;
    let decoded = decode_base64_url(payload)?;
    String::from_utf8(decoded).map_err(|e| format!("payload utf8 invalid: {e}"))
}

pub fn shake256_like(input: &str) -> String {
    logging::debug("auto.util", "shake256_like", "enter");
    shake256_hex(input, 16)
}

pub fn shake256_hex(input: &str, output_len: usize) -> String {
    logging::debug("auto.util", "shake256_hex", "enter");
    if input.is_empty() {
        return String::new();
    }

    let mut bytes = Vec::with_capacity(output_len);
    let mut round = 0u64;
    while bytes.len() < output_len {
        let mut hasher = DefaultHasher::new();
        round.hash(&mut hasher);
        input.hash(&mut hasher);
        bytes.extend_from_slice(&hasher.finish().to_be_bytes());
        round += 1;
    }

    bytes
        .into_iter()
        .take(output_len)
        .map(|b| format!("{b:02x}"))
        .collect()
}

pub fn setting<S: SettingsStore>(store: &S, key: &str, default_value: &str) -> String {
    store.get(key).unwrap_or_else(|| default_value.to_string())
}

pub fn init_jwt_secret<S: SettingsStore>(store: &S) -> String {
    if let Some(secret) = store.get("jwtSecret") {
        if !secret.trim().is_empty() {
            return secret;
        }
    }
    let secret = gen_secret(32);
    store.set_typed("jwtSecret", secret.clone(), Some("auth".to_string()));
    secret
}

pub fn gen_secret(output_len: usize) -> String {
    logging::debug("auto.util", "gen_secret", "enter");
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    shake256_hex(&format!("corn-syrup:{nanos}"), output_len)
}

pub fn allow_dev_all_origin_headers() -> Vec<(String, String)> {
    logging::debug("auto.util", "allow_dev_all_origin_headers", "enter");
    vec![
        ("Access-Control-Allow-Origin".to_string(), "*".to_string()),
        ("Access-Control-Allow-Headers".to_string(), "*".to_string()),
        (
            "Access-Control-Allow-Methods".to_string(),
            "GET, POST, PUT, DELETE, OPTIONS".to_string(),
        ),
    ]
}

pub fn check_status_code(status: u16, accepted: &str) -> bool {
    logging::debug("auto.util", "check_status_code", "enter");
    accepted
        .split('-')
        .collect::<Vec<_>>()
        .chunks(2)
        .any(|chunk| match chunk {
            [single] => single.trim().parse::<u16>().map(|v| v == status).unwrap_or(false),
            [start, end] => match (start.trim().parse::<u16>(), end.trim().parse::<u16>()) {
                (Ok(s), Ok(e)) => status >= s && status <= e,
                _ => false,
            },
            _ => false,
        })
}

pub fn accepted_status_ranges(ranges: &[&str]) -> Vec<u16> {
    logging::debug("auto.util", "accepted_status_ranges", "enter");
    let mut statuses = Vec::new();
    for range in ranges {
        let parts: Vec<_> = range.split('-').map(str::trim).collect();
        match parts.as_slice() {
            [single] => {
                if let Ok(v) = single.parse::<u16>() {
                    statuses.push(v);
                }
            }
            [start, end] => {
                if let (Ok(s), Ok(e)) = (start.parse::<u16>(), end.parse::<u16>()) {
                    statuses.extend(s..=e);
                }
            }
            _ => {}
        }
    }
    statuses
}

pub fn encode_base64_url(bytes: &[u8]) -> String {
    logging::debug("auto.util", "encode_base64_url", "enter");
    let table = BASE64_URL_SAFE;
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let b0 = bytes[i] as u32;
        let b1 = bytes.get(i + 1).copied().unwrap_or(0) as u32;
        let b2 = bytes.get(i + 2).copied().unwrap_or(0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(table[((n >> 18) & 63) as usize] as char);
        out.push(table[((n >> 12) & 63) as usize] as char);
        if i + 1 < bytes.len() {
            out.push(table[((n >> 6) & 63) as usize] as char);
        }
        if i + 2 < bytes.len() {
            out.push(table[(n & 63) as usize] as char);
        }
        i += 3;
    }
    out
}

pub fn get_days_remaining(now_epoch_secs: i64, expiry_epoch_secs: i64) -> i64 {
    logging::debug("auto.util", "get_days_remaining", "enter");
    const DAY: i64 = 86_400;
    (expiry_epoch_secs - now_epoch_secs + DAY - 1) / DAY
}

pub fn evaluate_json_query(document: &str, query: &str) -> bool {
    logging::debug("auto.util", "evaluate_json_query", "enter");
    let q = query.trim();
    if q.is_empty() || q == "$" {
        return !document.trim().is_empty();
    }
    document.contains(q.trim_matches('$').trim_matches('.'))
}
