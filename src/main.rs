#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{sleep, Duration};
use std::io;

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking_task")]
// async fn blocking_task() -> io::Result<Vec<u8>> {
async fn blocking_task() -> String {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read_to_string("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e));

    println!("{:?}", vec);
    // Ok(vec)
    match vec {
        Ok(Ok(s)) => format!("Content: {:?}", s),
        _ => "Could not read any data".to_owned(),
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![delay])
        .mount("/", routes![blocking_task])
}
