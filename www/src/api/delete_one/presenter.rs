use tide::{Request, Response};

pub async fn delete_one(req: Request<()>) -> tide::Result<Response> {
    match super::model::delete_one(req).await {
        Ok(_) => super::view::delete_one_ok(),
        Err(error) => super::view::delete_one_err(error),
    }
}
