use std::sync::Mutex;

use actix_web::{get, web, App, HttpRequest, HttpServer};

async fn baz() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

async fn hoge(rc: &Mutex<i32>) {
    let v = rc.lock().unwrap();
    baz().await;
    print!(" {:?}", v);
}

#[get("/index.html")]
async fn index(req: HttpRequest, rc: web::Data<Mutex<i32>>) -> String {
    println!("REQ: {:?}", req);
    hoge(&rc).await;
    rc.lock().unwrap().to_string()
}

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .app_data(web::Data::new(Mutex::new(42)))
    })
    .bind("0.0.0.0:8080")
    .expect("Failed to bind to 0.0.0.0:8080")
    .run()
    .await
    .expect("Failed to run server");
}
