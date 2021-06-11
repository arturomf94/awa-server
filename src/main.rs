#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::tokio::task::spawn_blocking;
use std::io;

#[get("/logs")]
async fn logs() -> String {
    let vec = spawn_blocking(|| std::fs::read_to_string("/tmp/awa_data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e));
    match vec {
        Ok(Ok(s)) => s,
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
        .mount("/", routes![logs])
}
