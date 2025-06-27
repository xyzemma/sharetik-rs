use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer,Responder,get};
use serde_derive::Deserialize;
use actix_cors::Cors;
use reqwest::header::USER_AGENT;
use reqwest::Client;

#[derive(Debug, Deserialize)]
pub struct Params {
    url: String,
}

#[get("/sharetik")]
async fn ttconv(req: HttpRequest) -> HttpResponse {
    let client: Client = reqwest::Client::new();
    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();
    println!("{}",params.url);
    let res = match client
        .get(params.url.clone())
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.7103.48 Safari/537.36")
        .send()
        .await {
            Ok(res) => res,
            Err(err) => panic!("Error: {}", err)
        };
    let respath = res.url().path();
    HttpResponse::Ok().body(format!("{:?}", respath))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin();
        App::new().wrap(cors).service(ttconv)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}