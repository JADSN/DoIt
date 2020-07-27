// #[async_std::main]
fn main() {
    // * INIT SERVER
    server::entrypoint().unwrap();

    // * INIT CLIENT
    termcli::entrypoint();

    // Ok(client::entrypoint().await?)
}
