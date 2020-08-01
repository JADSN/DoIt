use shared::todo_insert::TodoInsert;
use tide::Request;

pub async fn insert_one(mut req: Request<()>) -> Result<(), String> {
    match req.body_json::<TodoInsert>().await {
        Ok(data) => match server::database::insert_one::presenter::insert_one(data) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        },
        Err(error) => Err(error.to_string()),
    }
}
