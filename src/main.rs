use actix_web::{get, web, App, HttpServer, Responder};
use crossbeam_queue::ArrayQueue;
use std::sync::Arc;
use std::thread::sleep;

struct MyData {
    queue: ArrayQueue<i32>,
}

fn manip_queue(data: &MyData) {
    loop {
        let random_number = rand::random::<i32>();
        println!("Pushing {} to queue", random_number);
        let _ = data.queue.push(random_number);
        sleep(std::time::Duration::from_millis(1000));
    }
}

#[get("/")]
async fn display_queue(data: web::Data<MyData>) -> String {
    println!("Displaying queue");
    let mut result = String::new();
    for i in 0..data.queue.len() {
        result.push_str(&data.queue.pop().unwrap().to_string());
        result.push_str(" ");
    }
    result
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

// todo: perhaps don't use the actix_web::main: https://github.com/actix/actix-web/issues/1283
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(MyData {
        queue: ArrayQueue::new(10),
    });

    std::thread::spawn({
        let inner_config = Arc::clone(&data);
        move || {
            manip_queue(inner_config.as_ref());
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(greet)
            .service(display_queue)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
