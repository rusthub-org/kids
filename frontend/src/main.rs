mod util;
mod routes;
mod models;

use crate::util::constant::CFG;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let app_state = State {};
    let mut app = tide::with_state(app_state);
    // app = push_res(app).await;
    routes::push_res(&mut app).await;

    let log_level = CFG.get("LOG_LEVEL").unwrap();
    use std::str::FromStr;
    femme::with_level(femme::LevelFilter::from_str(log_level).unwrap());
    app.with(tide::log::LogMiddleware::new());

    app.listen(format!(
        "{}:{}",
        CFG.get("ADDR").unwrap(),
        CFG.get("PORT").unwrap()
    ))
    .await?;

    Ok(())
}

#[derive(Clone, Debug)]
pub struct State {}
