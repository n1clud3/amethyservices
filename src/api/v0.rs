use rocket::response::content;

#[derive(serde::Deserialize)]
struct ServingValues {
    latest_version: String,
}

#[get("/")]
pub fn api_docs() -> content::Html<Vec<u8>> {
    let index = std::fs::read("pages/api_v0.html");
    let index =
        match index {
            Ok(file) => file,
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        error!("cannot find \"api_v0.html\".");
                        return content::Html(String::from("<b>api_v0.html</b> is missing! Check server logs for more information.").as_bytes().to_vec());
                    }
                    other_error => panic!("Problem opening file: {:?}", other_error),
                }
            }
        };

    content::Html(index)
}

#[get("/checkversion/<user_version>")]
pub fn check_latest_version(user_version: String) -> String {
    // Gather values from values.toml
    let values_file = std::fs::read_to_string("values.toml");
    let values_file = match values_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                error!("Could not find \"values.toml\". Have you created it?");
                return String::from("unknown. Error occured on server.");
            }
            other_error => panic!("Error occured whilst opening file: {:?}", other_error),
        },
    };

    let values: ServingValues = toml::from_str(&values_file).unwrap();

    if user_version == values.latest_version {
        return format!("{}", values.latest_version);
    } else {
        return format!("{}. Consider updating client!", values.latest_version);
    }
}