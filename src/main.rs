mod api;
mod app;
mod pages;

// use actix_web::middleware::Logger;
// use cfg_if::cfg_if;
// cfg_if! {
// if #[cfg(feature = "ssr")] {

mod appdata;
mod websocket;
const HOME_ROOM_ID: uuid::Uuid = uuid::Uuid::nil();

use crate::appdata::AppData;
use crate::websocket::lobby::Lobby;
use actix::Actor;
use actix_files::Files;
use actix_web::*;
use chatroom_project_v3::app::*;
use leptos::*;
use leptos_actix::handle_server_fns;
use leptos_actix::{generate_route_list, LeptosRoutes};
use sqlx::postgres::PgPoolOptions;
use std::io::Write;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "0");
    init_logger();

    let database_url =
        std::env::var("DATABASE_URL").expect("missing DATABASE_URL environment variable");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let lobby_addr = Lobby::default().start();

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    log::info!(target: "Starting HttpServer", "listening on http://{}", &addr);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;
        // let logger = Logger::default();
        let app_data = web::Data::new(AppData::new(
            leptos_options.to_owned(),
            pool.clone(),
            lobby_addr.clone(),
        ));
        // let app_data = web::Data::new(AppData2::new(
        //     leptos_options.to_owned(),
        //     pool.clone(),
        // ));

        App::new()
            // .wrap(logger)
            .route("/api/{tail:.*}", handle_server_fns())
            .service(actix_web::web::redirect("/", "/login"))
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(actix_web::web::scope("/ws/chatroom").service(api::chatroom::start_connection))
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(app_data)
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

fn init_logger() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let level_style = buf.default_level_style(record.level());
            let mut bracket_style = buf.style();
            bracket_style.set_dimmed(true);
            writeln!(
                buf,
                "{}{} {:5} {}:{}{} {}",
                bracket_style.value("["),
                chrono::Local::now().format("%H:%M:%S"),
                level_style.value(record.level()),
                record.module_path().unwrap_or("null"),
                record.line().unwrap_or(0),
                bracket_style.value("]"),
                record.args()
            )
        })
        .init();
}

// } else {
// fn main() {
//     //        use chatroom_project_v3::app::App;
//     //
//     //        _ = console_log::init_with_level(log::Level::Debug);
//     //        console_error_panic_hook::set_once();
//     //        mount_to_body(App);
// }
// }
// }
