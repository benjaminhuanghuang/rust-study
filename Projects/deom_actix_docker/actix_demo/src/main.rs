use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
  HttpResponse::Ok().body("Hey there")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
  log::info!("Starting HTTP server: go to http://localhost:8964");

  HttpServer::new(|| {
    App::new()
      .wrap(Logger::default())
      .service(hello)
      .service(echo)
      .route("/hey", web::get().to(manual_hello))
  })
  .bind(("0.0.0.0", 8964))?
  .run()
  .await
}
