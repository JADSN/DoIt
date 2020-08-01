use tide::{Request, Response};

pub async fn read_all(req: Request<()>) -> tide::Result<Response> {
    match super::model::read_all(req).await {
        Ok(data) => super::view::read_all_ok(data),
        Err(error) => super::view::read_all_err(error),
    }
}
