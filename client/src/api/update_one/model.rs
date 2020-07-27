use shared::todo_update::TodoUpdate;
use tide::Request;

pub async fn update_one(mut req: Request<()>) -> Result<(), String> {
    let id: u8 = req.param("id").unwrap_or(0);
    match req.body_json::<TodoUpdate>().await {
        Ok(data) => match server::database::update_one::presenter::update_one(data, id) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        },
        Err(error) => Err(error.to_string()),
    }
}
