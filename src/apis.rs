use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Person {
  name: String,
  age: u8,
  hobbies: Vec<String>,
}

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok()
    .json(Person {
      name: "Alice".to_string(),
      age: 20,
      hobbies: vec!["programming".to_string(), "music".to_string()],
    })
}

#[get("/technologies/{tech_name}")]
async fn get_technologiy_page(
  tech_name: web::Path<String>) -> impl Responder {

  //TODO: DBへアクセスして技術ページに表示する情報を取得する

  //TODO: HTML形式のレスポンスボディを生成する

  HttpResponse::Ok()
    .content_type("text/html")
    .body(tech_name.to_string())  // pathに指定した技術名をそのまま返す
}

pub async fn create_app(addr: &str, port: u16) -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(hello)
      .service(get_technologiy_page)
    })
    .bind((addr, port))?
    .run()
    .await
}
