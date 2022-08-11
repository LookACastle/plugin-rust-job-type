use actix_web::{web};
use serde::{Serialize};
use std::env;

#[derive(Serialize)]
pub struct HealthResponse {
    service: String,
    fatman_name: String,
    fatman_version: String,
    git_version: String,
    deployed_by_racetrack_version: String,
    status: String,
    deployment_timestamp: u64,
}

#[derive(Serialize)]
pub struct LiveResponse {
    status: String,
    deployment_timestamp: u64,
}

#[derive(Serialize)]
pub struct ReadyResponse {
    status: String,
}

pub async fn health_handler() -> web::Json<HealthResponse> {
    let deployment_timestamp: u64 = env::var("FATMAN_DEPLOYMENT_TIMESTAMP").unwrap().parse().unwrap();
    let fatman_name = env::var("FATMAN_NAME").unwrap();
    let fatman_version = env::var("FATMAN_VERSION").unwrap();
    let git_version = env::var("GIT_VERSION").unwrap();
    let racetrack_version = env::var("DEPLOYED_BY_RACETRACK_VERSION").unwrap();
    web::Json(HealthResponse {
        service: String::from("fatman"),
        fatman_name: fatman_name,
        fatman_version: fatman_version,
        git_version: git_version,
        deployed_by_racetrack_version: racetrack_version,
        status: String::from("pass"),
        deployment_timestamp: deployment_timestamp,
    })
}

pub async fn live_handler() -> web::Json<LiveResponse> {
    let deployment_timestamp: u64 = env::var("FATMAN_DEPLOYMENT_TIMESTAMP").unwrap().parse().unwrap();
    web::Json(LiveResponse {
        status: String::from("live"),
        deployment_timestamp: deployment_timestamp,
    })
}

pub async fn ready_handler() -> web::Json<ReadyResponse> {
    web::Json(ReadyResponse {
        status: String::from("ready"),
    })
}