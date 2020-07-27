use tide::prelude::*;
use tide::Redirect;

// * INTERNAL MODULES
mod api;

// ! ENTRYPOINT
pub async fn entrypoint() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();

    // * STATIC FILES
    app.at("/").get(Redirect::new("/static/index.html"));
    app.at("/static").serve_dir("client/static/")?;

    app.at("/api").nest({
        let mut api = tide::new();
        api.at("/").get(crate::api::read_all::presenter::read_all);
        api.at("/")
            .post(crate::api::insert_one::presenter::insert_one);
        api.at("/:id").put(|_| async {
            Ok(json!({
                "status": "UPDATED",

            }))
        });
        api.at("/:id").delete(|_| async {
            Ok(json!({
                "status": "DELETED",

            }))
        });
        api
    });

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
