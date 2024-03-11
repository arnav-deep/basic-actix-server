mod model;

use crate::model::AppState;
use serde::Deserialize;
use std::sync::Mutex;

use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder, Result};

#[derive(Deserialize)]
struct UserFriendInfo {
    user_name: String,
    friend: String,
}

#[derive(Deserialize)]
struct UserFriendInfoQuery {
    user_name: String,
    friend: String,
}

// #[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    HttpResponse::Ok().body(format!(
        "Hello world! You've visited {} {} times",
        app_name, counter
    ))
}

#[get("/hello_query")]
async fn hello_query(info: web::Query<UserFriendInfoQuery>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Hello Query Operator {}! Your friend {} is coming",
        info.user_name, info.friend
    ))
}

#[post("/hello_json")]
async fn hello_json(info: web::Json<UserFriendInfo>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Hello JSON Operator {}! Your friend {} is coming",
        info.user_name, info.friend
    ))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/who/{user_name}/{friend}")]
async fn who(info: web::Path<UserFriendInfo>) -> Result<String> {
    Ok(format!(
        "Hello {}! Your friend {} is coming",
        info.user_name, info.friend
    ))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn localip_conifg(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .guard(guard::Host("127.0.0.1"))
            .route("", web::get().to(manual_hello)),
    );
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        app_name: String::from("Actix-web server Tutorial"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            // .service(hello)
            .service(echo)
            .service(web::scope("/hey").route("/there.php", web::get().to(manual_hello)))
            .service(
                web::scope("/")
                    .guard(guard::Host("localhost"))
                    .route("", web::get().to(hello)),
            )
            .configure(localip_conifg)
            .service(who)
            .service(hello_query)
            .service(hello_json)
    })
    .bind(("000000000", 8080))?
    .run()
    .await
}
