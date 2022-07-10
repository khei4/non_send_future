use std::cell::RefCell;

use actix_web::{get, web, App, HttpRequest, HttpServer};

async fn baz() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

async fn hoge(rc: &RefCell<i32>) {
    let mut v = rc.borrow_mut();
    *v += 1;
    baz().await;
    print!("incremented, {}", v);
}

#[get("/index.html")]
async fn index(req: HttpRequest, rc: web::Data<RefCell<i32>>) -> String {
    println!("REQ: {:?}", req);
    hoge(&rc).await;
    rc.borrow().to_string()
}

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .app_data(web::Data::new(RefCell::new(42)))
    })
    .bind("0.0.0.0:8080")
    .expect("Failed to bind to 0.0.0.0:8080")
    .run()
    .await
    .expect("Failed to run server");
}
