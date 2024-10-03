#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
  println!("launching server...");
  rust_api::apis::create_app("0.0.0.0", 8080).await
}
