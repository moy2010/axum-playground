use axum::{
    extract::FromRequest,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum DemoError {
    IO {
        cause: Box<dyn std::error::Error + Send + Sync>,
    },
    ResourceNotFound,
    Validation {
        message: String,
    },
}

impl From<sqlx::Error> for DemoError {
    fn from(error: sqlx::Error) -> Self {
        Self::IO {
            cause: error.into(),
        }
    }
}

impl std::fmt::Display for DemoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DemoError::IO { cause } => {
                write!(f, "{}", cause)
            }
            DemoError::ResourceNotFound => {
                write!(f, "Resource not found")
            }
            DemoError::Validation { message } => {
                write!(f, "{}", message)
            }
        }
    }
}

// Create our own JSON extractor by wrapping `axum::Json`. This makes it easy to override the
// rejection and provide our own which formats errors to match our application.
//
// `axum::Json` responds with plain text if the input is invalid.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(DemoError))]
struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

impl IntoResponse for DemoError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        // Create error span
        tracing::error!("{self}");

        // Map domain error to API error
        let (status, message) = match self {
            DemoError::IO { cause: _ } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_owned(),
            ),
            DemoError::ResourceNotFound => (StatusCode::NOT_FOUND, "Resource not found".to_owned()),
            DemoError::Validation { message } => (StatusCode::BAD_REQUEST, message),
        };

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}
