use actix_web::*;

pub struct Counter {
    value: usize,
}

impl Counter {
    fn new(size: usize) -> Counter {
        Counter { value: size }
    }

    fn incr(&mut self, size: usize) {
        self.value += size
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    use std::sync::{Arc, Mutex};
    let data = Arc::new(Mutex::new(Counter::new(1)));

    let local = tokio::task::LocalSet::new();
    let sys = actix_rt::System::run_in_tokio("server", &local);
    let server_res = HttpServer::new(move || {
        App::new().data(data.clone()).route(
            "/",
            web::get().to(|d: web::Data<Arc<Mutex<Counter>>>| {
                let mut c = d.lock().unwrap();
                c.incr(1);
                HttpResponse::Ok().body(format!("Counter {}", c.value))
            }),
        )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await?;
    sys.await?;
    Ok(server_res)
}
