use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[get("/")] // RustのAttribute. Pythonでいうデコレーター
async fn hello() -> impl Responder {　// impl Responder トレイト. インターフェースのようなもの
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl  Responder {
    HttpResponse::Ok().body("Manual Hello!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { // || はクロージャー. 引数がある場合は |引数| { ... } となる
        App::new()
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
