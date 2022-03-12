#[macro_use]
extern crate rocket;

mod api;
use api::v0;

#[get("/")]
fn index() -> &'static str {
    "Hello, if you see this, then Amethyservices is running!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/", routes![api::index])
        .mount("/api/v0/", routes![v0::api_docs, v0::check_latest_version])
}
