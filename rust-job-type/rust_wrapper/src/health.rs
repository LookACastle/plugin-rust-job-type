use actix_web::{web};
use serde::{Serialize};
use std::env;

#[derive(Serialize)]
pub struct HealthResponse {
    service: String,
    job_name: String,
    job_version: String,
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
    let deployment_timestamp: u64 = env::var("JOB_DEPLOYMENT_TIMESTAMP").unwrap().parse().unwrap();
    let job_name = env::var("JOB_NAME").unwrap();
    let job_version = env::var("JOB_VERSION").unwrap();
    let git_version = env::var("GIT_VERSION").unwrap();
    let racetrack_version = env::var("DEPLOYED_BY_RACETRACK_VERSION").unwrap();
    web::Json(HealthResponse {
        service: String::from("job"),
        job_name: job_name,
        job_version: job_version,
        git_version: git_version,
        deployed_by_racetrack_version: racetrack_version,
        status: String::from("pass"),
        deployment_timestamp: deployment_timestamp,
    })
}

pub async fn live_handler() -> web::Json<LiveResponse> {
    let deployment_timestamp: u64 = env::var("JOB_DEPLOYMENT_TIMESTAMP").unwrap().parse().unwrap();
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
