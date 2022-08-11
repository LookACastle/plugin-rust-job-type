use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use log::{info, LevelFilter};
use env_logger::{Builder, Target};
use std::collections::HashMap;
use serde_json::{Value};
use crate::handler::perform;
use crate::health::{health_handler, live_handler, ready_handler};
use std::env;

#[actix_web::main]
pub async fn serve() -> std::io::Result<()> {
    Builder::new()
        .target(Target::Stdout)
        .filter(None, LevelFilter::Debug)
        .init();
    info!("I wish only to serve...");

    let fatman_name = env::var("FATMAN_NAME").unwrap();
	// Serve endpoints at raw path (to facilitate debugging) and prefixed path (when accessed through PUB).
	// Accept any version so that fatman can be called by its many version names ("latest", "1.x").
	let base_urls: Vec<String> = vec![
		format!("/pub/fatman/{name}/{{version}}", name=fatman_name),
		String::new(),
    ];

    HttpServer::new(move || {
        let mut app = App::new();

        for base_url in base_urls.iter() {
            app = app
                .route(&*format!("{}", base_url), web::get().to(homepage))
                .route(&*format!("{}/", base_url), web::get().to(homepage))
                .route(&*format!("{}/health", base_url), web::get().to(health_handler))
                .route(&*format!("{}/live", base_url), web::get().to(live_handler))
                .route(&*format!("{}/ready", base_url), web::get().to(ready_handler))
                .route(&*format!("{}/api/v1/perform", base_url), web::post().to(perform_handler))
        }

        app.wrap(Logger::default())
    })
    .bind(("0.0.0.0", 7000))?
    .run()
    .await
}

async fn homepage() -> impl Responder {
    HttpResponse::Ok().body("This is a fatman built with Rust language wrapper")
}

async fn perform_handler(input: web::Json<HashMap<String, Value>>) -> web::Json<Value> {
    let input_map: HashMap<String, Value> = input.into_inner();
    let output: Value = perform(input_map);
    web::Json(output)
}
