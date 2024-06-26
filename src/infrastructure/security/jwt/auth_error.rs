use axum::response::{ IntoResponse, Response };
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;

pub enum AuthError {
    InvalidToken,
    WrongCredentials,
    TokenCreation,
    MissingCredentials,
}

/// ## Descripción
/// Implementación del trait `IntoResponse` para la estructura `AuthError`, que convierte un `AuthError` en una respuesta HTTP.

/// ## Precondición
/// - Se proporciona una instancia válida de `AuthError`.

/// ## Poscondición
/// - Se devuelve una respuesta HTTP que corresponde al tipo de error en `AuthError`.


impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
