use tide::{Request, Response};

pub async fn update_one(req: Request<()>) -> tide::Result<Response> {
    match super::model::update_one(req).await {
        Ok(_) => super::view::update_one_ok(),
        Err(error) => super::view::update_one_err(error),
    }
}
