use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
if #[cfg(feature = "ssr")] {

use actix_files::Files;
use actix_web::*;
use leptos_actix::{generate_route_list, LeptosRoutes};
use chatroom_project_v3::app::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .service(actix_web::web::redirect("/", "/login"))
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}
} else {
    fn main() {
        use chatroom_project_v3::app::App;

        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();
        mount_to_body(App);
    }
}
}
