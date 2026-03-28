use std::error::Error;

use feather::{App, State, middleware_fn as route, next};
use nue_model::card::NfcCardBuilder;
use nue_storage::{Storage, sqlite::SqliteStorage};

const INDEX: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>undefined</title>
</head>
<body>
    <h1>undefined</h1>
</body>
</html>
"#;

#[route]
fn index() {
    res.send_html(INDEX);
    next!()
}

#[route]
fn cards() {
    let state = ctx.get_state::<State<SqliteStorage>>();
    let db = state.lock();
    let bytes = db.list()?;

    // Release the lock before sending the response.
    drop(db);

    res.send_bytes(postcard::to_allocvec(&bytes)?);
    next!()
}

pub fn app() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    #[cfg(test)]
    let mut db = SqliteStorage::in_memory()?;
    #[cfg(not(test))]
    let mut db = SqliteStorage::open("/srv/db/db.sqlite")?;

    db.put(
        0,
        NfcCardBuilder::new()
            .membership_id(0)
            .username("subscriber-0")
            .uid([0; 10].into())
            .finish(),
    )?;
    // States.
    app.context().set_state(State::new(db));

    // Routes.
    app.get("/", index);
    app.get("/cards", cards);

    // Run the app.
    app.listen("0.0.0.0:9090");
    Ok(())
}
