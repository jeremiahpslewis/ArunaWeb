pub mod server;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_session::{storage::CookieSessionStore, SessionMiddleware};
    use actix_web::web::Data;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_start::app::*;
    use server::oidc::Authorizer;
    use std::sync::Mutex;
    //use server::actix_routes;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <EntryPoint/> });

    let secret_key = cookie::Key::generate();

    let data = Data::new(Mutex::new(Authorizer::new().await));

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(Data::clone(&data))
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .service(server::actix_routes::login)
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                |cx| view! { cx, <EntryPoint/> },
            )
            .service(Files::new("/", site_root))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}