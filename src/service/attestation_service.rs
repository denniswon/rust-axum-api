use crate::config::database::{Database, DatabaseTrait};
use crate::dto::attestation_dto::{AttestationReadDto, AttestationRegisterDto};
use crate::entity::attestation::Attestation;
use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;    
use crate::error::attestation_error::AttestationError;
use crate::repository::attestation_repository::{AttestationRepository, AttestationRepositoryTrait};
use sqlx::Error as SqlxError;
use std::sync::Arc;

#[derive(Clone)]
pub struct AttestationService {
    attestation_repo: AttestationRepository,
    db_conn: Arc<Database>,
}

impl AttestationService {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            attestation_repo: AttestationRepository::new(db_conn),
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn create_attestation(&self, payload: AttestationRegisterDto) -> Result<AttestationReadDto, ApiError> {
        let attestation = self.add_attestation(payload).await;

        return match attestation {
            Ok(user) => Ok(AttestationReadDto::from(user)),
            Err(e) => match e {
                SqlxError::Database(e) => match e.code() {
                    Some(code) => {
                        if code == "23000" {
                            Err(DbError::UniqueConstraintViolation(e.to_string()))?
                        } else {
                            Err(DbError::SomethingWentWrong(e.to_string()))?
                        }
                    }
                    _ => Err(DbError::SomethingWentWrong(e.to_string()))?,
                },
                _ => Err(DbError::SomethingWentWrong(e.to_string()))?,
            },
        };
    }

    async fn add_attestation(&self, payload: AttestationRegisterDto) -> Result<Attestation, SqlxError> {
        let insert = sqlx::query_as!(
            Attestation,
            r#"
                INSERT INTO attestations (request_id, attestation_type, attestation_data)
                VALUES ($1, $2, $3)
            "#,
            payload.request_id,
            payload.attestation_type,
            payload.attestation_data
        )
        .execute(self.db_conn.get_pool())
        .await?;

        let attestation = self.attestation_repo.find(insert.last_insert_id()).await?;
        return Ok(attestation);
    }
}
