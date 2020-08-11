use tide::{Request, Response};

pub async fn update_done(req: Request<()>) -> tide::Result<Response> {
    match super::model::update_done(req).await {
        Ok(_) => super::view::update_done_ok(),
        Err(error) => super::view::update_done_err(error),
    }
}
