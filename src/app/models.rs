
use sqlx::PgPool;
pub  struct AppState {
   pub  db: PgPool
}

#[derive(serde::Deserialize)]
pub struct UpdateCours {
    pub title: Option<String>,
    pub description: Option<String>,
    pub duree: Option<String>,
    pub categorie: Option<String>,
    pub prix: Option<String>,
    pub prix_promo: Option<String>,
    pub langue: Option<String>,
}


use thiserror::Error;

#[derive(Error, Debug)]
pub enum PersonError {
    #[error("member not found")]
    PersonNotFound,
    #[error("failed to update member")]
    PersonUpdateFailure,
    #[error("failed to create member")]
    PersonCreationFailure,
    #[error("failed to delete member")]
    PersonDeleteFailure,
}

pub type ErrorMessage = String;

pub trait ResponseErrorTrait {
    fn create(person_error: PersonError) -> ErrorMessage;
}

impl ResponseErrorTrait for ErrorMessage {
    fn create(person_error: PersonError) -> ErrorMessage {
        match person_error {
            PersonError::PersonNotFound => ErrorMessage::from("member not found"),
            PersonError::PersonUpdateFailure => ErrorMessage::from("failed to update member"),
            PersonError::PersonCreationFailure => ErrorMessage::from("failed to create member"),
            PersonError::PersonDeleteFailure => ErrorMessage::from("failed to delete member"),
        }
    }
}