pub mod service;

use self::service::settings::{
    update_settings,
};

use repository::storage::*;
use std::sync::{ Arc, Mutex };

use actix;

use actix_web::{
    http,
    middleware,
    server,
    App,
    HttpRequest,
    HttpResponse,
    Json,
    FromRequest,
    ResponseError,
};

use domain::{
    Settings,
};

struct AppState {
    db: Arc<Store>,
}

fn get_settings<'r>(req: &'r HttpRequest<AppState>) -> HttpResponse {
    let ref db = req.state().db;

    HttpResponse::Ok().json(db.get_settings().unwrap())
}

fn set_settings((settings, req): (Json<Settings>, HttpRequest<AppState>)) -> HttpResponse {
    let ref db = *req.state().db;

    let response = update_settings(db, &settings);

    match response {
        Ok(settings) => HttpResponse::Ok().json(settings),
        Err(error) => error.error_response(),
    }
}

pub fn server() {
    let mut db = PgStore::new();

    let foo = Arc::new(db);

    server::new(move || {
        App::with_state(AppState { db: foo.clone() })
            .middleware(middleware::Logger::default())
            .resource("/settings", |r| {
                r.method(http::Method::GET).f(get_settings);
                r.method(http::Method::POST)
                    .with_config(set_settings, |cfg| {
                        cfg.0.limit(10000000);
                    });
            })
    }).bind("127.0.0.1:8081")
    .unwrap()
    .start();
}
