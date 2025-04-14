use actix_web::{HttpResponse, Responder, web, Error};
//use crate::app::models::PersonError::{PersonCreationFailure, PersonDeleteFailure, PersonUpdateFailure ,};
use crate::app::models::{AppState , UpdateCours};
use sqlx::FromRow;
use serde_json::json;
use serde::{Deserialize, Serialize};
// Fonction pour supprimer un cours par ID

#[derive( Deserialize, FromRow, Serialize)] 
pub struct Cours {
    pub id : String,
    pub title: String,
    pub description :String,
    pub duree : String,
    pub categorie : String,
    pub prix : String,
    pub prix_promo : String,
    pub langue : String,
    pub url :  String,
}

pub async fn delete_cours(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> impl Responder {
    let result = sqlx::query(
        r#"
        DELETE FROM cours WHERE id = $1
        "#
    )
    .bind(id.into_inner())
    .execute(&data.db)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({ "message": "Cours supprimé avec succès" })),
        Err(err) => {
            eprintln!("Erreur lors de la suppression du cours: {:?}", err);
            HttpResponse::InternalServerError().json(json!({ "error": "Erreur lors de la suppression du cours" }))
        }
    }
}


pub async fn update_cours_by_id(
    state: web::Data<AppState>, // Récupérer l'état de l'application (connexion DB)
    id: web::Path<String>, // Paramètre Path : ID du cours
    json: web::Json<UpdateCours>, // Données JSON à mettre à jour
) -> Result<HttpResponse, Error> {
    // Requête SQL pour mettre à jour les informations du cours
    let result = sqlx::query(
        "UPDATE cours SET
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            duree = COALESCE($3, duree),
            categorie = COALESCE($4, categorie),
            prix = COALESCE($5, prix),
            prix_promo = COALESCE($6, prix_promo),
            langue = COALESCE($7, langue)
        WHERE id = $8",
    )
    .bind(&json.title)          // Optionnel, sera ignoré si None
    .bind(&json.description)    // Optionnel, sera ignoré si None
    .bind(&json.duree)          // Optionnel, sera ignoré si None
    .bind(&json.categorie)      // Optionnel, sera ignoré si None
    .bind(&json.prix)           // Optionnel, sera ignoré si None
    .bind(&json.prix_promo)     // Optionnel, sera ignoré si None
    .bind(&json.langue)         // Optionnel, sera ignoré si None                // L'ID du cours (ne doit pas être modifié)
    .execute(&state.db)         // Exécution de la requête sur la base de données
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json("Cours mis à jour avec succès")), // Réponse en cas de succès
        Err(e) => {
            // En cas d'erreur, renvoyer une erreur avec le message
            Ok(HttpResponse::InternalServerError().json(format!("Erreur lors de la mise à jour: {}", e)))
        }
    }
}

