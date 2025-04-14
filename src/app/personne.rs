use crate::app::models::AppState;
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid; 
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

//use serde_json::json;
#[derive(Deserialize, Serialize, FromRow)]
pub struct Personne {
    pub  name: String,
    pub  email: String,
    pub  password: String,
    pub  use_name : String,
    pub  numero: String,
    pub id :  String,
 }

pub async fn add_personne(
    data: web::Data<AppState>,
    personne: web::Json<Personne>,
) -> Result<impl Responder, actix_web::Error> {
    let user_id = Uuid::new_v4().to_string();

    let result = sqlx::query(
        r#"INSERT INTO notes (name, email, password, use_name, numero, id) VALUES ($1, $2, $3, $4, $5, $6)"#
    )
    .bind(personne.name.to_string())
    .bind(personne.email.to_string())
    .bind(personne.password.to_string())
    .bind(personne.use_name.to_string())
    .bind(personne.numero.to_string())
    .bind(user_id.clone())
    .execute(&data.db)
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::Created().json(serde_json::json!({"message": "Personne ajoutée", "id": user_id}))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({"error": "Erreur serveur"}))),
    }
}
