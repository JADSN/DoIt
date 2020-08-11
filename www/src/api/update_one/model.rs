use shared::todo_update::Update;

use tide::Request;

pub async fn update_done(mut req: Request<()>) -> Result<(), String> {
    match req.body_json::<Update>().await {
        Ok(data) => {
            if data.col_name.eq(&"done") && data.new_value.is_boolean() {
                match server::database::update_one::done::presenter::update(data) {
                    Ok(_) => Ok(()),
                    Err(error) => Err(error.to_string()),
                }
            } else if data.col_name.eq(&"description") && data.new_value.is_string() {
                match server::database::update_one::description::presenter::update(data) {
                    Ok(_) => Ok(()),
                    Err(error) => Err(error.to_string()),
                }
            } else {
                Ok(())
            }
        }

        Err(error) => Err(error.to_string()),
    }
}
