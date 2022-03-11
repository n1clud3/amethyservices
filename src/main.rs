#[macro_use]
extern crate rocket;

use rocket::{response::content, fs::FileServer};

#[get("/")]
fn index() -> content::Html<Vec<u8>> {
    let index = std::fs::read("static/index.html");
    let index = match index {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                warn!("cannot find \"index.html\" in \"static\" directory");
                return content::Html(String::from(
                    "<b>index.html</b> is missing! Check server logs for more information.",
                ).as_bytes().to_vec());
            }
            other_error => panic!("Problem opening file: {:?}", other_error),
        },
    };

    content::Html(index)
}

// #[get("/assets/<file>")]
// fn assets_css(file: String) -> content::Css<Vec<u8>> {
//     let requested_file = std::fs::read(format!("frontend/assets/{}", file));
//     let requested_file = match requested_file {
//         Ok(file) => file,
//         Err(e) => match e.kind() {
//             std::io::ErrorKind::NotFound => {
//                 warn!("cannot find requested file");
//                 return content::Css(String::from(
//                     "/* Requested file not found */"
//                 ).as_bytes().to_vec())
//             },
//             other_error => panic!("Problem opening file: {:?}", other_error),
//         },
//     };

//     content::Css(requested_file)
// }

// #[get("/assets/<file>.js", rank=1)]
// fn assets_js(file: String) -> content::JavaScript<Vec<u8>> {
//     let requested_file = std::fs::read(format!("frontend/assets/{}", file));
//     let requested_file = match requested_file {
//         Ok(file) => file,
//         Err(e) => match e.kind() {
//             std::io::ErrorKind::NotFound => {
//                 warn!("cannot find requested file");
//                 return content::JavaScript(String::from(
//                     "// Requested file not found"
//                 ).as_bytes().to_vec())
//             },
//             other_error => panic!("Problem opening file: {:?}", other_error),
//         },
//     };

//     content::JavaScript(requested_file)
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/assets", FileServer::from("static/assets")) // loads resources required by the frontend
}
