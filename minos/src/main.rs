mod auth;
mod database;
mod error;
mod routes;
mod util;

use crate::util::env::parse_var;
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use log::{error, info, warn};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup environment variables & check sanity
    dotenvy::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    if check_env_vars() {
        error!("Some environment variables are missing!");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Missing required environment variables",
        ));
    }

    // Set up Ory configurations
    let basic_configuration = util::ory::generate_basic_configuration();
    let admin_configuration = util::ory::generate_admin_configuration();

    // Set up Sentry watching for errors
    let sentry = sentry::init(sentry::ClientOptions {
        release: sentry::release_name!(),
        traces_sample_rate: 0.1,
        enable_profiling: true,
        profiles_sample_rate: 0.1,
        ..Default::default()
    });
    if sentry.is_enabled() {
        info!("Enabled Sentry integration");
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    // Database Connector
    let pool = database::connect()
        .await
        .expect("Database connection failed");

    // Start server
    info!("Starting Actix HTTP server!");
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::COOKIE,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(basic_configuration.clone()))
            .app_data(web::Data::new(admin_configuration.clone()))
            .configure(routes::user_config)
            .configure(routes::admin_config)
            .wrap(sentry_actix::Sentry::new())
            .default_service(web::get().to(routes::not_found))
    })
    .bind(dotenvy::var("BIND_ADDR").unwrap())?
    .run()
    .await
}

// This is so that env vars not used immediately don't panic at runtime
fn check_env_vars() -> bool {
    let mut failed = false;

    // Check existence of a single .env variable
    fn check_var<T: std::str::FromStr>(var: &'static str) -> bool {
        let check = parse_var::<T>(var).is_none();
        if check {
            warn!(
                "Variable `{}` missing in dotenv or not of type `{}`",
                var,
                std::any::type_name::<T>()
            );
        }
        check
    }

    failed |= check_var::<String>("BIND_ADDR");
    failed |= check_var::<String>("ORY_URL");
    failed |= check_var::<String>("ORY_ADMIN_URL");
    failed |= check_var::<String>("ORY_AUTH_BEARER");

    failed |= check_var::<String>("LABRINTH_API_URL");
    failed |= check_var::<String>("LABRINTH_ADMIN_KEY");
    failed |= check_var::<String>("RATE_LIMIT_IGNORE_KEY");

    failed
}
