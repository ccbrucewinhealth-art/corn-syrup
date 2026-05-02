use crate::backend::config::AppConfig;
use axum::extract::Request;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Instant;
use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn app() -> Router {
    app_with_config(None)
}

pub fn app_with_config(config: Option<&AppConfig>) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/setup-database-info", get(setup_database_info))
        .route("/setup-database", post(setup_database))
        .route("/test-webhook", post(test_webhook))
        .route("/test-x-www-form-urlencoded", post(test_x_www_form_urlencoded))
        .route("/_e2e/take-sqlite-snapshot", get(e2e_take_sqlite_snapshot))
        .route("/_e2e/restore-sqlite-snapshot", get(e2e_restore_sqlite_snapshot))
        .route("/robots.txt", get(robots_txt))
        .route("/metrics", get(metrics))
        .route("/.well-known/change-password", get(change_password))
        .route("/api/entry-page", get(api_entry_page))
        .route("/api/badge/:id/status", get(badge_status))
        .route("/api/badge/:id/uptime", get(badge_uptime))
        .route("/api/badge/:id/uptime/:duration", get(badge_uptime))
        .route("/api/badge/:id/ping", get(badge_ping))
        .route("/api/badge/:id/ping/:duration", get(badge_ping))
        .route("/api/badge/:id/avg-response", get(badge_avg_response))
        .route("/api/badge/:id/avg-response/:duration", get(badge_avg_response))
        .route("/api/badge/:id/cert-exp", get(badge_cert_exp))
        .route("/api/badge/:id/response", get(badge_response))
        .route("/api/monitors", get(api_monitors_list))
        .route("/api/monitor/:id", get(api_monitor_get))
        .route("/api/monitor/:id/heartbeat", get(api_monitor_heartbeat))
        .route("/api/status-page", get(api_status_page_list))
        .route("/api/settings", get(api_settings))
        .route("/api/login", post(api_login))
        .route("/api/register", post(api_register))
        .route("/api/logout", post(api_logout))
        .route("/status", get(status_default))
        .route("/status-page", get(status_default))
        .route("/status/:slug", get(status_slug))
        .route("/status/:slug/rss", get(status_slug_rss))
        .route("/api/status-page/:slug", get(api_status_page_slug))
        .route("/api/status-page/heartbeat/:slug", get(api_status_page_heartbeat))
        .route("/api/status-page/:slug/manifest.json", get(api_status_page_manifest))
        .route("/api/status-page/:slug/incident-history", get(api_status_page_incident_history))
        .route("/api/status-page/:slug/badge", get(api_status_page_badge))
        .route("/migrate-status", get(migrate_status))
        .layer(middleware::from_fn(api_debug_middleware))
        .layer(cors_layer(config))
}

pub async fn serve(config: AppConfig) -> Result<(), String> {
    let host = config.hostname.clone().unwrap_or_else(|| "0.0.0.0".to_string());
    let ip = host.parse::<IpAddr>().unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED));
    let addr = SocketAddr::new(ip, config.port);
    crate::backend::logging::debug("rest.server", "serve", format!("listen host={}, port={}", host, config.port));
    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|err| err.to_string())?;
    axum::serve(listener, app_with_config(Some(&config))).await.map_err(|err| err.to_string())
}

fn cors_layer(config: Option<&AppConfig>) -> CorsLayer {
    let allow_origin = build_allow_origin(config);
    CorsLayer::new()
        .allow_origin(allow_origin)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(tower_http::cors::Any)
}

fn build_allow_origin(config: Option<&AppConfig>) -> AllowOrigin {
    let allowed = config
        .and_then(|cfg| cfg.args.get("cors-origin"))
        .cloned()
        .or_else(|| std::env::var("CORN_SYRUP_BACKEND_CORS_ORIGIN").ok())
        .unwrap_or_else(|| "*".to_string());

    crate::backend::logging::debug("rest.server", "cors_origin", &allowed);

    if allowed.trim() == "*" {
        return AllowOrigin::any();
    }

    let origins = allowed
        .split(',')
        .filter_map(|origin| HeaderValue::from_str(origin.trim()).ok())
        .collect::<Vec<_>>();

    if origins.is_empty() {
        AllowOrigin::any()
    } else {
        AllowOrigin::list(origins)
    }
}

async fn api_debug_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let version = format!("{:?}", req.version());
    let is_api = uri.path().starts_with("/api/") || uri.path() == "/api";
    let started_at = Instant::now();

    if is_api {
        crate::backend::logging::debug(
            "rest.api.trace",
            "request.start",
            format!("method={method}, uri={uri}, version={version}"),
        );
        crate::backend::logging::debug(
            "rest.api.trace",
            "request.headers",
            format_headers(req.headers()),
        );
        crate::backend::logging::debug("rest.api.trace", "request.dispatch", "enter handler");
    }

    let response = next.run(req).await;

    if is_api {
        let elapsed_ms = started_at.elapsed().as_millis();
        crate::backend::logging::debug(
            "rest.api.trace",
            "response.status",
            format!("status={}, elapsed_ms={elapsed_ms}", response.status()),
        );
        crate::backend::logging::debug(
            "rest.api.trace",
            "response.headers",
            format_headers(response.headers()),
        );
        crate::backend::logging::debug("rest.api.trace", "request.end", format!("method={method}, uri={uri}"));
    }

    response
}

fn format_headers(headers: &axum::http::HeaderMap) -> String {
    let mut pairs = headers
        .iter()
        .map(|(key, value)| {
            let value = if is_sensitive_header(key.as_str()) {
                "***".to_string()
            } else {
                value.to_str().unwrap_or("<non-utf8>").to_string()
            };
            format!("{}={}", key.as_str(), value)
        })
        .collect::<Vec<_>>();
    pairs.sort();
    if pairs.is_empty() {
        "<none>".to_string()
    } else {
        pairs.join(", ")
    }
}

fn is_sensitive_header(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "authorization" | "cookie" | "set-cookie" | "x-api-key" | "x-auth-token"
    )
}

fn log_api_step(handler: &str, step: &str, detail: impl Into<String>) {
    crate::backend::logging::debug("rest.api.step", &format!("{handler}.{step}"), detail.into());
}

async fn index() -> Html<&'static str> { Html("<div id=\"root\"></div>") }
async fn setup_database_info() -> Response { json("{\"runningSetup\":false,\"needSetup\":false}") }
async fn setup_database() -> Response { json("{\"ok\":true}") }
async fn test_webhook() -> Response { json("{\"ok\":true}") }
async fn test_x_www_form_urlencoded() -> Response { json("{\"ok\":true}") }
async fn e2e_take_sqlite_snapshot() -> Response { json("{\"ok\":true,\"snapshot\":null}") }
async fn e2e_restore_sqlite_snapshot() -> Response { json("{\"ok\":true}") }
async fn robots_txt() -> Response { text("User-agent: *\nDisallow: /\n") }
async fn metrics() -> Response { text("# HELP corn_syrup_up Backend health\n# TYPE corn_syrup_up gauge\ncorn_syrup_up 1\n") }
async fn change_password() -> Response { redirect("/") }
async fn api_entry_page() -> Response { api_json("api_entry_page", "{\"type\":\"entryPage\",\"entryPage\":\"dashboard\"}") }
async fn badge_status() -> Response { api_svg_badge("badge_status", "status", "unknown") }
async fn badge_uptime() -> Response { api_svg_badge("badge_uptime", "uptime", "unknown") }
async fn badge_ping() -> Response { api_svg_badge("badge_ping", "ping", "unknown") }
async fn badge_avg_response() -> Response { api_svg_badge("badge_avg_response", "avg-response", "unknown") }
async fn badge_cert_exp() -> Response { api_svg_badge("badge_cert_exp", "cert-exp", "unknown") }
async fn badge_response() -> Response { api_svg_badge("badge_response", "response", "unknown") }
async fn api_monitors_list() -> Response {
    log_api_step("api_monitors_list", "load", "list monitors from repository placeholder");
    api_json("api_monitors_list", "{\"ok\":true,\"monitors\":[]}")
}
async fn api_monitor_get() -> Response { api_json("api_monitor_get", "{\"ok\":true,\"monitor\":null}") }
async fn api_monitor_heartbeat() -> Response { api_json("api_monitor_heartbeat", "{\"ok\":true,\"heartbeatList\":[]}") }
async fn api_status_page_list() -> Response { api_json("api_status_page_list", "{\"ok\":true,\"statusPageList\":[]}") }
async fn api_settings() -> Response { api_json("api_settings", "{\"ok\":true,\"settings\":{\"timezone\":\"Asia/Taipei\"}}") }
async fn api_login() -> Response { api_json("api_login", "{\"ok\":true,\"token\":null}") }
async fn api_register() -> Response { api_json("api_register", "{\"ok\":true,\"msg\":\"register ok\"}") }
async fn api_logout() -> Response { api_json("api_logout", "{\"ok\":true}") }
async fn status_default() -> Response { Html("<div id=\"root\"></div>").into_response() }
async fn status_slug() -> Response { Html("<div id=\"root\"></div>").into_response() }
async fn status_slug_rss() -> Response { text("<?xml version=\"1.0\" encoding=\"UTF-8\"?><rss version=\"2.0\"><channel></channel></rss>") }
async fn api_status_page_slug() -> Response { api_json("api_status_page_slug", "{\"ok\":true,\"slug\":null}") }
async fn api_status_page_heartbeat() -> Response { api_json("api_status_page_heartbeat", "{\"ok\":true,\"heartbeatList\":[]}") }
async fn api_status_page_manifest() -> Response { api_json("api_status_page_manifest", "{\"name\":\"corn-syrup-backend\",\"short_name\":\"corn-syrup-backend\"}") }
async fn api_status_page_incident_history() -> Response { api_json("api_status_page_incident_history", "{\"ok\":true,\"incidentList\":[]}") }
async fn api_status_page_badge() -> Response { api_svg_badge("api_status_page_badge", "status", "unknown") }
async fn migrate_status() -> Response { json("{\"ok\":true,\"status\":\"idle\"}") }

fn api_json(handler: &str, body: &'static str) -> Response {
    log_api_step(handler, "prepare_response", "content_type=application/json; charset=utf-8");
    log_api_step(handler, "response_body", body);
    json(body)
}

fn api_svg_badge(handler: &str, label: &'static str, value: &'static str) -> Response {
    log_api_step(handler, "prepare_badge", format!("label={label}, value={value}"));
    let response = svg_badge(label, value);
    log_api_step(handler, "return", "content_type=image/svg+xml; charset=utf-8");
    response
}

fn json(body: &'static str) -> Response {
    ([("content-type", "application/json; charset=utf-8")], body).into_response()
}

fn text(body: &'static str) -> Response {
    ([("content-type", "text/plain; charset=utf-8")], body).into_response()
}

fn redirect(location: &'static str) -> Response {
    (StatusCode::FOUND, [("location", location)], "").into_response()
}

fn svg_badge(label: &'static str, value: &'static str) -> Response {
    let body = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"140\" height=\"20\"><text x=\"4\" y=\"14\">{}: {}</text></svg>", label, value);
    ([("content-type", "image/svg+xml; charset=utf-8")], body).into_response()
}
