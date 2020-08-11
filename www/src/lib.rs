use async_std::task;
use tide::Redirect;
// * INTERNAL MODULES
mod api;

// ! ENTRYPOINT
pub fn entrypoint() -> Result<(), std::io::Error> {
    tide::log::start();

    let mut app = tide::new();

    // * STATIC FILES
    app.at("/").get(Redirect::new("/static/index.html"));
    app.at("/static").serve_dir("static/")?;

    // * API
    app.at("/api/todos").nest({
        let mut api = tide::new();
        api.at("/")
            .get(crate::api::read_all::presenter::read_all)
            .post(crate::api::insert_one::presenter::insert_one)
            .patch(crate::api::update_one::presenter::update_done);

        api.at("/:id")
            .delete(crate::api::delete_one::presenter::delete_one);
        api
    });

    task::block_on(async {
        app.listen("0.0.0.0:8080").await?;
        Ok(())
    })
}
