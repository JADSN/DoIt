#[async_std::main]
async fn main() -> std::io::Result<()> {
    // * INIT SERVER
    server::entrypoint().unwrap();

    // * INIT CLIENT
    Ok(client::entrypoint().await?)
}
