use tide::{Request, Response};

pub async fn insert_one(req: Request<()>) -> tide::Result<Response> {
    match super::model::insert_one(req).await {
        Ok(_) => super::view::insert_one_ok(),
        Err(error) => super::view::insert_one_err(error),
    }
}
