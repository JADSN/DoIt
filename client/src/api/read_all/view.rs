use shared::todo::Todo;

use tide::{http::mime, prelude::*, Body, Response};

pub fn read_all_ok(data: Todo) -> tide::Result<Response> {
    Ok(Response::builder(200)
        .body(Body::from_json(&data)?)
        .content_type(mime::JSON)
        .build())
}

pub fn read_all_err(error: String) -> tide::Result<Response> {
    Ok(Response::builder(200)
        .body(json!({ "status": error }))
        .content_type(mime::JSON)
        .build())
}
