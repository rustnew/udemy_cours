use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::app::models::AppState;
use sqlx::FromRow;
use serde::{Deserialize, Serialize}; 

#[derive(Deserialize, FromRow, Serialize)] 
pub struct Cours {
    pub id: i32,               
    pub title: String,
    pub description: String,
    pub duree: String,
    pub categorie: String,
    pub prix: String,
    pub prix_promo: String,
    pub langue: String,
    pub url: String,
}

pub async fn afficher_cours(pool: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Cours>("SELECT * FROM cours") // Correctement spécifié
        .fetch_all(&pool.db)
        .await
    {
        Ok(cours) => HttpResponse::Ok().json(json!({ "cours": cours })),
        Err(err) => {
            eprintln!("Erreur lors de la récupération des cours: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "error": "Erreur lors de la récupération des cours"
            }))
        }
    }
}
