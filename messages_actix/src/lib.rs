#[macro_use]
extern crate actix_web;

use actix_web::{
    middleware, web, App, HttpServer, HttpRequest, HttpResponse, Result
};
use actix_web::error::{
    Error, InternalError, JsonPayloadError
};
use serde::{Serialize, Deserialize};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

const LOG_FORMAT: &'static str = r#""%r" %s %b "%{User-Agent}i" %D"#;

struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}

#[derive(Serialize)]
struct IndexResponse {
    server_id: usize,
    request_count: usize,
    messages: Vec<String>,
}

#[derive(Deserialize)]
struct PostInput {
    message: String,
}

#[derive(Serialize)]
struct PostResponse {
    server_id: usize,
    request_count: usize,
    message: String,
}

#[derive(Serialize)]
struct PostError {
    server_id: usize,
    request_count: usize,
    error: String,
}

#[derive(Serialize)]
struct LookupResponse {
    server_id: usize,
    request_count: usize,
    result: Option<String>,
}

#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let messages = state.messages.lock().unwrap();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: messages.clone(),
    }))
}

#[get("/lookup/{index}")]
fn lookup(state: web::Data<AppState>, idx: web::Path<usize>) -> Result<web::Json<LookupResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let messages = state.messages.lock().unwrap();
    let result = messages.get(idx.into_inner()).cloned();

    Ok(web::Json(LookupResponse {
        server_id: state.server_id,
        request_count,
        result
    }))
}

#[post("/clear")]
fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let mut messages = state.messages.lock().unwrap();
    messages.clear();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: vec![],
    }))
}

fn post(payload: web::Json<PostInput>, state: web::Data<AppState>) -> Result<web::Json<PostResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let mut messages = state.messages.lock().unwrap();
    messages.push(payload.message.clone());

    Ok(web::Json(PostResponse {
        server_id: state.server_id,
        request_count,
        message: payload.message.clone(),
    }))
}

fn post_error(err: JsonPayloadError, req: &HttpRequest) -> Error {
    let state = req.get_app_data::<AppState>().unwrap();
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let post_error = PostError {
        server_id: state.server_id,
        request_count,
        error: format!("{}", err),
    };

    InternalError::from_response(
        err,
        HttpResponse::BadRequest().json(post_error)
    ).into()    
}

pub struct MessageApp {
    port: u16,
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    pub fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: 127.0.0.1:{}", self.port);

        let messages = Arc::new(Mutex::new(vec![]));

        HttpServer::new(move || {
            App::new()
                .data(AppState {
                    server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                    request_count: Cell::new(0),
                    messages: messages.clone(),
                })
                .wrap(middleware::Logger::new(LOG_FORMAT))
                .service(index)
                .service(
                    web::resource("/send")
                    .data(
                        web::JsonConfig::default().limit(4096).error_handler(post_error),
                    )
                    .route(web::post().to(post)),
                )
                .service(clear)
                .service(lookup)
        })
        .bind(("127.0.0.1", self.port))?
        .workers(8)
        .run()
    }
}
