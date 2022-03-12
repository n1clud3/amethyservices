pub mod v0;

use rocket::response::content;

#[get("/")]
pub fn index() -> content::Html<Vec<u8>> {
    let index = std::fs::read("pages/api.html");
    let index =
        match index {
            Ok(file) => file,
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        error!("cannot find \"api.html\".");
                        return content::Html(String::from("<b>api.html</b> is missing! Check server logs for more information.").as_bytes().to_vec());
                    }
                    other_error => panic!("Problem opening file: {:?}", other_error),
                }
            }
        };

    content::Html(index)
}