use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer,Responder,get};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    url: String,
}

#[get("/sharetik")]
async fn ttconv(req: HttpRequest) -> HttpResponse {
    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();
    HttpResponse::Ok().body(format!("{:?}", params.url))
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(ttconv)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}