use tide::Request;

pub async fn delete_one(req: Request<()>) -> Result<(), String> {
    let id: u8 = req.param("id").unwrap_or(0);
    match server::database::delete_one::presenter::delete_one(id) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}
