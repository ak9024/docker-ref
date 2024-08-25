use actix_web::{
    body::BoxBody, get, http::header::ContentType, middleware::Logger, App, HttpResponse,
    HttpServer, Responder,
};
use env_logger::Env;
use serde::Serialize;

#[derive(Serialize)]
struct Data {
    name: &'static str,
}

impl Responder for Data {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/")]
async fn index() -> impl Responder {
    Data {
        name: "Hello World",
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(index)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
