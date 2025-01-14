use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::adapters::http::AppState;
use crate::domain::game_manager::ports::GameManagerService;

#[derive(Debug, Clone)]
pub struct ApiSuccess<T: Serialize + PartialEq>(StatusCode, Json<ApiResponseBody<T>>);

impl<T> PartialEq for ApiSuccess<T>
where
    T: Serialize + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 .0 == other.1 .0
    }
}

impl<T: Serialize + PartialEq> ApiSuccess<T> {
    fn new(status: StatusCode, data: T) -> Self {
        ApiSuccess(status, Json(ApiResponseBody::new(status, data)))
    }
}

impl<T: Serialize + PartialEq> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InternalServerError(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.to_string())
    }
}

impl From<()> for ApiError {
    fn from(_: ()) -> Self {
        Self::InternalServerError("(TEMPORAL ERROR TYPE)".to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!("API ERROR");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponseBody::new_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )),
        )
            .into_response()
    }
}

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiResponseBody<T: Serialize + PartialEq> {
    status_code: u16,
    data: T,
}

impl<T: Serialize + PartialEq> ApiResponseBody<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data,
        }
    }
}

impl ApiResponseBody<ApiErrorData> {
    pub fn new_error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: ApiErrorData { message },
        }
    }
}

/// The response data format for all error responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorData {
    pub message: String,
}


/// The body of an [Author] creation request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateGameRequestBody {
}

/// The response body data field for successful [Author] creation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateGameResponseData {
}

/// The body of an [Author] creation request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateGameHttpRequestBody {
}

#[allow(dead_code)]
#[derive(Debug, Clone, Error)]
enum ParseCreateGameHttpRequestError {
}

pub async fn create_game<GMS: GameManagerService>(
    State(state): State<AppState<GMS>>,
    Json(_body): Json<CreateGameHttpRequestBody>,
) -> Result<ApiSuccess<CreateGameResponseData>, ApiError> {
    let _ = state
        .game_manager_service
        .new_game()
        .await
    //    .map_err(ApiError::from)
    //    .map(|_| ApiSuccess::new(StatusCode::CREATED, CreateGameResponseData { }));
    ;

    Ok(ApiSuccess::new(StatusCode::CREATED, CreateGameResponseData { }))
}
