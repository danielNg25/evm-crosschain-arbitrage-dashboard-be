use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;

mod bot;
mod config;
mod database;
mod errors;
mod handlers;
mod routes;
mod services;
use clap::Parser;
use env_logger::Env;
use log::LevelFilter;

use config::Config;
use database::service::MongoDbService;
use routes::configure_routes;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Parse command line arguments and setup logging
    let args = Args::parse();
    let log_level = match args.log_level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level.to_string())).init();
    // Load configuration
    let config = Config::load().expect("Failed to load configuration");

    info!("Starting Arbitrage Bot API...");
    info!("Configuration loaded: {:?}", config);

    // Initialize database service
    let db_service = MongoDbService::new(&config.database)
        .await
        .expect("Failed to initialize database service");

    let db = db_service.get_client().database();

    // Start the background indexer in a separate task
    // let mut indexer = SimpleIndexer::new();
    // tokio::spawn(async move {
    //     indexer.start().await;
    // });

    // Build bind address from config
    let bind_addr = format!("{}:{}", config.server.host, config.server.port);

    info!("Server will be available at http://{}", bind_addr);

    HttpServer::new(move || {
        // Configure CORS from config
        let allowed_origins = config.cors.allowed_origins.clone();

        // Use allowed_origin_fn for more flexible origin matching
        let cors = Cors::default().allowed_origin_fn(move |origin, _req_head| {
            let origin_str = match origin.to_str() {
                Ok(s) => s,
                Err(_) => return false,
            };
            allowed_origins.iter().any(|allowed| origin_str == allowed)
        });

        // Convert string methods to HTTP methods
        let methods: Vec<actix_web::http::Method> = config
            .cors
            .allowed_methods
            .iter()
            .filter_map(|m| m.parse().ok())
            .collect();

        // Add WebSocket-specific headers and methods
        let mut all_headers = config.cors.allowed_headers.clone();
        all_headers.extend_from_slice(&[
            "Upgrade".to_string(),
            "Connection".to_string(),
            "Sec-WebSocket-Key".to_string(),
            "Sec-WebSocket-Version".to_string(),
            "Sec-WebSocket-Protocol".to_string(),
        ]);

        let mut all_methods = methods;
        all_methods.push(actix_web::http::Method::from_bytes(b"OPTIONS").unwrap());

        let cors = cors
            .allowed_methods(all_methods)
            .allowed_headers(all_headers)
            .expose_headers(vec![
                "Upgrade".to_string(),
                "Connection".to_string(),
                "Sec-WebSocket-Accept".to_string(),
            ])
            .max_age(3600);

        let cors = if config.cors.supports_credentials {
            cors.supports_credentials()
        } else {
            cors
        };

        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(std::sync::Arc::new(config.clone())))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    .bind(&bind_addr)?
    .run()
    .await
}
