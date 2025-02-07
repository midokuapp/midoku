use std::str::FromStr;

#[cfg(target_os = "android")]
use dioxus::mobile::use_asset_handler;

#[cfg(not(target_os = "android"))]
use dioxus::desktop::use_asset_handler;

#[doc(hidden)]
macro_rules! response {
    ($status:expr) => {{
        #[cfg(target_os = "android")]
        use dioxus::mobile::wry::http::Response;

        #[cfg(not(target_os = "android"))]
        use dioxus::desktop::wry::http::Response;

        Response::builder()
            .status($status)
            .body(Vec::new())
            .unwrap()
    }};
    ($image_src:expr, $image_bytes:expr) => {{
        #[cfg(target_os = "android")]
        use dioxus::mobile::wry::http::Response;

        #[cfg(not(target_os = "android"))]
        use dioxus::desktop::wry::http::Response;

        Response::builder()
            .header("Content-Type", $image_src.format().to_mime_type())
            .body($image_bytes)
            .unwrap()
    }};
}

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

pub fn use_gallery_handler() {
    use_asset_handler("gallery", move |request, responder| {
        if request.method() != "GET" {
            return responder.respond(response!(404));
        }

        let uri = request.uri();
        let query = uri.query().unwrap_or_default();

        let image_url: String = match get_decoded(query, "url") {
            Some(url) => url.to_string(),
            None => return responder.respond(response!(400)),
        };

        let width: u32 = match get_int(query, "width") {
            Some(value) => value,
            None => return responder.respond(response!(400)),
        };

        let height: u32 = match get_int(query, "height") {
            Some(value) => value,
            None => return responder.respond(response!(400)),
        };

        crate::util::thread::spawn!(async move {
            let image_bytes = crate::util::http::download_bytes(image_url).await;
            let Ok(image_bytes) = image_bytes else {
                return responder.respond(response!(404));
            };

            let image_src = crate::util::image::Image::try_from(image_bytes);
            let image_src = image_src.and_then(|src| src.resize(width, height));
            let Ok(image_src) = image_src else {
                return responder.respond(response!(500));
            };

            let Ok(image_bytes) = TryInto::<Vec<u8>>::try_into(&image_src) else {
                return responder.respond(response!(500));
            };

            responder.respond(response!(image_src, image_bytes));
        });
    });
}
