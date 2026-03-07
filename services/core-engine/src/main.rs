use axum::{extract::State, response::Json, routing::{get, post, delete}, Router};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

struct AppState { start_time: Instant, stats: Mutex<Stats> }
struct Stats { total_requests_proxied: u64, total_apis_managed: u64, total_rate_limits_hit: u64, total_auth_checks: u64 }

#[derive(Serialize)]
struct Health { status: String, version: String, uptime_secs: u64, total_ops: u64 }

#[derive(Deserialize)]
struct CreateApiRequest { name: String, upstream_url: String, path_prefix: String, rate_limit_per_minute: Option<u32>, auth_required: Option<bool>, cors_enabled: Option<bool> }
#[derive(Serialize)]
struct CreateApiResponse { api_id: String, name: String, upstream_url: String, path_prefix: String, rate_limit_per_minute: u32, auth_required: bool, cors_enabled: bool, status: String, created_at: String }

#[derive(Deserialize)]
struct TrafficAnalysisRequest { api_id: String, time_range_minutes: Option<u32> }
#[derive(Serialize)]
struct TrafficAnalysisResponse { api_id: String, total_requests: u64, avg_latency_ms: f64, p95_latency_ms: f64, p99_latency_ms: f64, error_rate_percent: f64, top_endpoints: Vec<EndpointStats>, status_distribution: serde_json::Value }

#[derive(Serialize)]
struct EndpointStats { path: String, method: String, count: u64, avg_latency_ms: f64 }

#[derive(Deserialize)]
struct RateLimitConfigRequest { api_id: String, strategy: Option<String>, requests_per_minute: u32, burst_size: Option<u32> }
#[derive(Serialize)]
struct RateLimitConfigResponse { config_id: String, api_id: String, strategy: String, requests_per_minute: u32, burst_size: u32, status: String }

#[derive(Deserialize)]
struct OpenApiGenerateRequest { api_id: String, format: Option<String> }
#[derive(Serialize)]
struct OpenApiGenerateResponse { api_id: String, format: String, spec: serde_json::Value, endpoints_documented: u32, generated_at: String }

#[derive(Serialize)]
struct StatsResponse { total_requests_proxied: u64, total_apis_managed: u64, total_rate_limits_hit: u64, total_auth_checks: u64 }

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "alice_api_gateway_engine=info".into())).init();
    let state = Arc::new(AppState { start_time: Instant::now(), stats: Mutex::new(Stats { total_requests_proxied: 0, total_apis_managed: 0, total_rate_limits_hit: 0, total_auth_checks: 0 }) });
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/gateway/apis", post(create_api))
        .route("/api/v1/gateway/apis", get(list_apis))
        .route("/api/v1/gateway/apis/{api_id}", delete(delete_api))
        .route("/api/v1/gateway/traffic", post(traffic_analysis))
        .route("/api/v1/gateway/ratelimit", post(configure_rate_limit))
        .route("/api/v1/gateway/openapi", post(generate_openapi))
        .route("/api/v1/gateway/stats", get(stats))
        .layer(cors).layer(TraceLayer::new_for_http()).with_state(state);
    let addr = std::env::var("GATEWAY_ENGINE_ADDR").unwrap_or_else(|_| "0.0.0.0:8081".into());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("API Gateway Engine on {addr}");
    axum::serve(listener, app).await.unwrap();
}

async fn health(State(s): State<Arc<AppState>>) -> Json<Health> {
    let st = s.stats.lock().unwrap();
    Json(Health { status: "ok".into(), version: env!("CARGO_PKG_VERSION").into(), uptime_secs: s.start_time.elapsed().as_secs(), total_ops: st.total_requests_proxied + st.total_apis_managed })
}

async fn create_api(State(s): State<Arc<AppState>>, Json(req): Json<CreateApiRequest>) -> Json<CreateApiResponse> {
    let mut st = s.stats.lock().unwrap();
    st.total_apis_managed += 1;
    Json(CreateApiResponse {
        api_id: uuid::Uuid::new_v4().to_string(),
        name: req.name,
        upstream_url: req.upstream_url,
        path_prefix: req.path_prefix,
        rate_limit_per_minute: req.rate_limit_per_minute.unwrap_or(1000),
        auth_required: req.auth_required.unwrap_or(true),
        cors_enabled: req.cors_enabled.unwrap_or(true),
        status: "active".into(),
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}

async fn list_apis(State(_s): State<Arc<AppState>>) -> Json<Vec<CreateApiResponse>> {
    Json(vec![])
}

async fn delete_api(State(_s): State<Arc<AppState>>, axum::extract::Path(api_id): axum::extract::Path<String>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "api_id": api_id, "status": "deleted" }))
}

async fn traffic_analysis(State(s): State<Arc<AppState>>, Json(req): Json<TrafficAnalysisRequest>) -> Json<TrafficAnalysisResponse> {
    let st = s.stats.lock().unwrap();
    Json(TrafficAnalysisResponse {
        api_id: req.api_id,
        total_requests: st.total_requests_proxied,
        avg_latency_ms: 12.5,
        p95_latency_ms: 45.2,
        p99_latency_ms: 120.8,
        error_rate_percent: 0.3,
        top_endpoints: vec![
            EndpointStats { path: "/api/v1/users".into(), method: "GET".into(), count: 15000, avg_latency_ms: 8.2 },
            EndpointStats { path: "/api/v1/orders".into(), method: "POST".into(), count: 8500, avg_latency_ms: 22.1 },
        ],
        status_distribution: serde_json::json!({ "2xx": 98.5, "4xx": 1.2, "5xx": 0.3 }),
    })
}

async fn configure_rate_limit(State(s): State<Arc<AppState>>, Json(req): Json<RateLimitConfigRequest>) -> Json<RateLimitConfigResponse> {
    let mut st = s.stats.lock().unwrap();
    st.total_rate_limits_hit += 1;
    Json(RateLimitConfigResponse {
        config_id: uuid::Uuid::new_v4().to_string(),
        api_id: req.api_id,
        strategy: req.strategy.unwrap_or_else(|| "token_bucket".into()),
        requests_per_minute: req.requests_per_minute,
        burst_size: req.burst_size.unwrap_or(req.requests_per_minute / 10),
        status: "active".into(),
    })
}

async fn generate_openapi(State(_s): State<Arc<AppState>>, Json(req): Json<OpenApiGenerateRequest>) -> Json<OpenApiGenerateResponse> {
    Json(OpenApiGenerateResponse {
        api_id: req.api_id,
        format: req.format.unwrap_or_else(|| "json".into()),
        spec: serde_json::json!({
            "openapi": "3.1.0",
            "info": { "title": "Generated API Spec", "version": "1.0.0" },
            "paths": {}
        }),
        endpoints_documented: 0,
        generated_at: chrono::Utc::now().to_rfc3339(),
    })
}

async fn stats(State(s): State<Arc<AppState>>) -> Json<StatsResponse> {
    let st = s.stats.lock().unwrap();
    Json(StatsResponse { total_requests_proxied: st.total_requests_proxied, total_apis_managed: st.total_apis_managed, total_rate_limits_hit: st.total_rate_limits_hit, total_auth_checks: st.total_auth_checks })
}
