#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() {
    use actix_files::Files;
    use actix_web::*;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use monofolio::app::{shell, App};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone();

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // Serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // Serve other assets from the `assets` directory
            .service(Files::new("/assets", &*site_root))
            // Serve the favicon from /favicon.ico
            .service(Files::new("/", "public"))
            .leptos_routes(routes.to_owned(), {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
    })
    .bind(&addr)
    .unwrap()
    .run()
    .await
    .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless using hydrate feature
}
