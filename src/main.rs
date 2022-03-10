#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/")]
fn index() -> content::Html<String> {
    let index = std::fs::read_to_string("pages/index.html");
    let index = match index {
        Ok(r) => r, // r means result
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                warn!("cannot find \"index.html\" in \"pages\" directory");
                return content::Html(String::from(
                    "<b>index.html</b> is missing! Check server logs for more information.",
                ));
            }
            other_error => panic!("Problem opening pages directory: {:?}", other_error),
        },
    };

    content::Html(String::from(index))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
