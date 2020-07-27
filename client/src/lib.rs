use tide::Redirect;

// ! ENTRYPOINT
pub async fn entrypoint() -> Result<(), std::io::Error> {

    tide::log::start();
    let mut app = tide::new();

    // * STATIC FILES
    app.at("/").get(Redirect::new("/static/index.html"));
    app.at("/static").serve_dir("client/static/")?;

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
