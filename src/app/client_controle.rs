use actix_web::{HttpResponse, Responder, web};
use crate::app::models::AppState;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use serde_json::json;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Personne {
    pub  name: String,
    pub  email: String,
    pub  password: String,
    pub  use_name : String,
    pub  numero: String,
    pub id :  String,
 }

 
pub async fn delete_personne(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let note_id = path.into_inner();
    let result = sqlx::query(r#"DELETE FROM notes WHERE id = $1"#)
        .bind(note_id)
        .execute(&data.db)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Personne supprimée"})),
        Err(_) => {
            println!("erreur  lors de  la supression du  client ");
            HttpResponse::InternalServerError().json(json!({"error": "Erreur serveur"}))}
    }
}

pub async fn update_personne(
    data: web::Data<AppState>,
    personne: web::Json<Personne>,
) -> impl Responder {
    let result = sqlx::query(
        r#"
        UPDATE notes 
        SET name = $1, email = $2, password = $3, use_name = $4, numero = $5 
        WHERE id = $6
        "#
    )
    .bind(&personne.name)
    .bind(&personne.email)
    .bind(&personne.password)
    .bind(&personne.use_name)
    .bind(&personne.numero)
    .bind(&personne.id)
    .execute(&data.db)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({ "message": "Personne mise à jour avec succès" })),
        Err(_) => {
            println!("erreur lors   de  la  mise a  jours du cleint");
            HttpResponse::InternalServerError().json(json!({ "error": "Erreur lors de la mise à jour" }))}
    }
}
