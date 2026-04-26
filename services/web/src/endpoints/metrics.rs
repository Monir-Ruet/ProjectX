use std::time::Instant;

use axum::{
    Router,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
};
use lazy_static::lazy_static;
use prometheus::*;
use prometheus::{Encoder, TextEncoder};
use sysinfo::System;
use tokio::time::{Duration, sleep};

use crate::endpoints::AppState;

pub fn init() {
    init_metrics();
    spawn_system_metrics();
}

pub fn metrics_router() -> Router<AppState> {
    Router::new()
        .route("/metrics", get(metrics_handler_with_process))
        .layer(middleware::from_fn(metrics_middleware))
}

pub fn spawn_system_metrics() {
    tokio::spawn(async {
        let mut sys = System::new_all();
        let start_time: Instant = Instant::now();
        SYSTEM_MEMORY.set(sys.total_memory() as i64 / 1024 / 1024);
        loop {
            let pid = sysinfo::get_current_pid().unwrap();
            sys.refresh_all();
            if let Some(proc) = sys.process(pid) {
                APP_CPU_USAGE.set(proc.cpu_usage() as f64);
                APP_MEMORY_USAGE.set(proc.memory() as f64 / 1024.0 / 1024.0);
            }
            APP_CPU_SECONDS.set(start_time.elapsed().as_secs() as f64);
            SYSTEM_MEMORY_USAGE.set(sys.used_memory() as f64 / 1024.0 / 1024.0);
            sleep(Duration::from_secs(5)).await;
        }
    });
}

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref HTTP_REQUESTS_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("http_requests_total", "Total HTTP requests"),
        &["method", "route", "status"]
    ).unwrap();
    pub static ref HTTP_ERRORS_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("http_errors_total", "Total HTTP errors"),
        &["method", "route"]
    ).unwrap();
    // Gauges
    pub static ref ACTIVE_CONNECTIONS: IntGauge = IntGauge::new(
        "http_active_connections",
        "Active HTTP connections"
    ).unwrap();
    pub static ref SYSTEM_MEMORY: IntGauge = IntGauge::new(
        "system_total_memory_megabytes",
        "Total system memory"
    ).unwrap();
    pub static ref SYSTEM_MEMORY_USAGE: Gauge = Gauge::new(
        "system_memory_usage_megabytes",
        "Total system memory usage"
    ).unwrap();
    pub static ref APP_MEMORY_USAGE: Gauge = Gauge::new(
        "app_memory_usage_megabytes",
        "Memory usage"
    ).unwrap();
    pub static ref APP_CPU_USAGE: Gauge = Gauge::new(
        "app_cpu_usage",
        "CPU usage"
    ).unwrap();
    pub static ref APP_CPU_SECONDS: Gauge =
        Gauge::new("app_cpu_seconds_total", "CPU time").unwrap();
}

pub fn init_metrics() {
    REGISTRY
        .register(Box::new(HTTP_REQUESTS_TOTAL.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(HTTP_ERRORS_TOTAL.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(ACTIVE_CONNECTIONS.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(APP_MEMORY_USAGE.clone()))
        .unwrap();
    REGISTRY.register(Box::new(APP_CPU_USAGE.clone())).unwrap();
    REGISTRY
        .register(Box::new(APP_CPU_SECONDS.clone()))
        .unwrap();
    REGISTRY.register(Box::new(SYSTEM_MEMORY.clone())).unwrap();
    REGISTRY
        .register(Box::new(SYSTEM_MEMORY_USAGE.clone()))
        .unwrap();
}

async fn metrics_middleware(req: Request<axum::body::Body>, next: Next) -> Response {
    let method = req.method().to_string();
    let route = req.uri().path().to_string();
    ACTIVE_CONNECTIONS.inc();
    let response = next.run(req).await;
    ACTIVE_CONNECTIONS.dec();
    let status = response.status().as_u16();
    record_request(&method, &route, status);
    response
}

pub async fn metrics_handler_with_process() -> Response {
    let all_metrics = REGISTRY.gather();
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();

    encoder.encode(&all_metrics, &mut buffer).unwrap();
    (
        StatusCode::OK,
        [("content-type", "text/plain; version=0.0.4; charset=utf-8")],
        buffer,
    )
        .into_response()
}

pub fn record_request(method: &str, route: &str, status: u16) {
    HTTP_REQUESTS_TOTAL
        .with_label_values(&[method, route, &status.to_string()])
        .inc();
    if status >= 500 {
        HTTP_ERRORS_TOTAL.with_label_values(&[method, route]).inc();
    }
}
