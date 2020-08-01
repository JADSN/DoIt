use tide::{http::mime, prelude::*, Response};

pub fn update_one_ok() -> tide::Result<Response> {
    Ok(Response::builder(200).content_type(mime::JSON).build())
}

pub fn update_one_err(error: String) -> tide::Result<Response> {
    Ok(Response::builder(200)
        .body(json!({ "status": error }))
        .content_type(mime::JSON)
        .build())
}
