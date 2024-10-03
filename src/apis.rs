use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok()
    .content_type("application/json")
    .body(r#"{"name":"Taro, Tanaka", "age":10}"#)
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

use actix_web::{HttpServer, App};

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
