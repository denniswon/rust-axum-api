use crate::dto::attestation_dto::{AttestationReadDto, AttestationRegisterDto};
use crate::entity::attestation::Attestation;
use crate::entity::dcap::DcapVerifiedOutput;
use crate::error::db_error::DbError;
use crate::error::{api_error::ApiError, api_request_error::ValidatedRequest};
use crate::repository::attestation_repository::AttestationRepositoryTrait;
use crate::response::api_response::ApiSuccessResponse;
use crate::sp1::prove::DcapProof;
use crate::state::attestation_state::AttestationState;
use axum::{
    Json,
    extract::{Extension, Path, State},
};
use dcap_qvl::verify::VerifiedReport;

pub async fn get(
    Extension(attestation): Extension<Attestation>,
) -> Json<ApiSuccessResponse<AttestationReadDto>> {
    Json(ApiSuccessResponse::send(AttestationReadDto::from(
        attestation,
    )))
}

pub async fn query(
    State(state): State<AttestationState>,
    Path(id): Path<i32>,
) -> Result<Json<AttestationReadDto>, ApiError> {
    let attestation: Result<Attestation, DbError> =
        state.attestation_repo.find(id.try_into().unwrap()).await;
    match attestation {
        Ok(attestation) => Ok(Json(AttestationReadDto::from(attestation))),
        Err(e) => Err(ApiError::DbError(e)),
    }
}

pub async fn register(
    State(state): State<AttestationState>,
    ValidatedRequest(payload): ValidatedRequest<AttestationRegisterDto>,
) -> Result<Json<AttestationReadDto>, ApiError> {
    let attestation = state
        .attestation_service
        .create_attestation(payload)
        .await?;
    Ok(Json(attestation))
}

pub async fn verify_dcap_qvl(
    State(state): State<AttestationState>,
    Path(id): Path<i32>,
) -> Result<Json<VerifiedReport>, ApiError> {
    let attestation: Result<Attestation , DbError> =
        state.attestation_repo.find(id.try_into().unwrap()).await;
    match attestation {
        Ok(attestation) => {
            let tcb = state.attestation_service.verify_dcap_qvl(attestation).await;
            match tcb {
                Ok(tcb) => Ok(Json(tcb)),
                Err(e) => Err(ApiError::AttestationError(e)),
            }
        },
        Err(e) => Err(ApiError::DbError(e)),
    }
}

pub async fn verify_dcap(
    State(state): State<AttestationState>,
    Path(id): Path<i32>,
) -> Result<Json<DcapVerifiedOutput>, ApiError> {
    let attestation: Result<Attestation , DbError> =
        state.attestation_repo.find(id.try_into().unwrap()).await;
    match attestation {
        Ok(attestation) => {
            let tcb = state.attestation_service.verify_dcap(attestation);
            match tcb {
                Ok(tcb) => Ok(Json(DcapVerifiedOutput::from_output(tcb))),
                Err(e) => Err(ApiError::AttestationError(e)),
            }
        },
        Err(e) => Err(ApiError::DbError(e)),
    }
}

pub async fn prove(
    State(state): State<AttestationState>,
    Path(id): Path<i32>,
) -> Result<Json<DcapProof>, ApiError> {
    let proof = state
        .attestation_service
        .prove(id.try_into().unwrap())
        .await;
    match proof {
        Ok(proof) => Ok(Json(proof)),
        Err(e) => Err(ApiError::AttestationError(e)),
    }
}

pub async fn verify(
    State(state): State<AttestationState>,
    ValidatedRequest(payload): ValidatedRequest<DcapProof>,
) -> Result<Json<DcapVerifiedOutput>, ApiError> {
    let output = state
        .attestation_service
        .verify(payload)
        .await;
    match output {
        Ok(output) => Ok(Json(DcapVerifiedOutput::from_output(output))),
        Err(e) => Err(ApiError::AttestationError(e)),
    }
}

pub async fn submit_proof(
    State(state): State<AttestationState>,
    ValidatedRequest(payload): ValidatedRequest<DcapProof>,
) -> Result<Json<DcapVerifiedOutput>, ApiError> {
    let output = state
        .attestation_service
        .submit_proof(payload)
        .await;
    match output {
        Ok(output) => Ok(Json(DcapVerifiedOutput::from_output(output))),
        Err(e) => Err(ApiError::AttestationError(e)),
    }
}
