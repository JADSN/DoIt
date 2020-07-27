#[async_std::main]
async fn main() -> std::io::Result<()> {
    Ok(client::entrypoint().await?)
}
