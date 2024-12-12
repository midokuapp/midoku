use std::str::FromStr;

use tauri::http::{Request, Response};
use tauri::{Manager, Runtime, UriSchemeContext, UriSchemeResponder};

use crate::util::{http, image};

pub fn gallery<R: Runtime>(
    app: UriSchemeContext<'_, R>,
    request: Request<Vec<u8>>,
    responder: UriSchemeResponder,
) {
    let pool = app.app_handle().state::<rayon::ThreadPool>();

    fn response(status: u16) -> Response<Vec<u8>> {
        Response::builder().status(status).body(Vec::new()).unwrap()
    }

    let bad_request = response(400);
    let not_found = response(404);
    let internal_server_error = response(500);

    if request.method() != "GET" {
        return responder.respond(not_found);
    }

    let uri = request.uri();
    let query = uri.query().unwrap_or_default();

    fn get_value<'a>(query: &'a str, key: &str) -> Option<&'a str> {
        let start_pos = match query.find(&format!("{}=", key)) {
            Some(pos) => pos + key.len() + 1,
            None => return None,
        };
        let end_pos = match query[start_pos..].find("&") {
            Some(pos) => start_pos + pos,
            None => start_pos + query[start_pos..].len(),
        };
        Some(&query[start_pos..end_pos])
    }

    fn get_decoded(query: &str, key: &str) -> Option<String> {
        get_value(query, key)
            .and_then(|value| urlencoding::decode(value).ok())
            .map(|value| value.to_string())
    }

    fn get_int<T: FromStr>(query: &str, key: &str) -> Option<T> {
        get_value(query, key).and_then(|value| value.parse::<T>().ok())
    }

    let image_url = match get_decoded(query, "url") {
        Some(url) => url.to_string(),
        None => return responder.respond(bad_request),
    };

    let width = match get_int(query, "width") {
        Some(value) => value,
        None => return responder.respond(bad_request),
    };

    let height = match get_int(query, "height") {
        Some(value) => value,
        None => return responder.respond(bad_request),
    };

    pool.spawn(move || {
        let image_bytes = http::download_bytes(image_url);
        let image_bytes = match image_bytes {
            Ok(bytes) => bytes,
            Err(_) => return responder.respond(not_found),
        };

        let image_src = image::Image::try_from(image_bytes);
        let image_src = image_src.and_then(|src| src.resize(width, height));
        let image_src = match image_src {
            Ok(src) => src,
            Err(_) => return responder.respond(internal_server_error),
        };

        let image_bytes: Vec<u8> = match (&image_src).try_into() {
            Ok(bytes) => bytes,
            Err(_) => return responder.respond(internal_server_error),
        };

        responder.respond(
            Response::builder()
                .header("Content-Type", image_src.format().to_mime_type())
                .body(image_bytes)
                .unwrap(),
        );
    });
}
