use actix_web::{web, App, HttpServer, http::header};
use actix_cors::Cors;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use log::{info, error};

mod app;
use crate::app::models::AppState;
use crate::app::client_controle::{delete_personne, update_personne};
use crate::app::cours_controle::{update_cours_by_id, delete_cours};
use crate::app::cours_api::afficher_cours;
use crate::app::personne::add_personne;

// Configuration des routes de gestion des cours et des clients
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/controles")
            .service(
                web::scope("/cours")
                    .route("/{id}", web::put().to(update_cours_by_id))
                    .route("/{id}", web::delete().to(delete_cours)),
            )
            .service(
                web::scope("/client")
                    .route("/{id}", web::delete().to(delete_personne))
                    .route("/{id}", web::put().to(update_personne)),
            ),
    )
    .service(
        web::scope("/cours")
            .route("/add", web::post().to(add_personne))
            .route("/", web::get().to(afficher_cours)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL non trouvée");
    let pool = PgPoolOptions::new()
        .max_connections(30)
        .connect(&database_url)
        .await
        .expect("Erreur de connexion à la base de données");

    // Appliquer les migrations
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => info!("✅ Migrations appliquées avec succès"),
        Err(e) => {
            error!("🔥 Erreur lors de l'application des migrations: {}", e);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE", "UPDATE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .wrap(cors)
            .configure(routes)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
