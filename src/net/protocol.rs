use serde::{Deserialize, Serialize};

use super::user::Username;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub user: Username,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum RegisterResponse {
    Ok,
    Denied { reason: String },
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum ServerRequest {
    SendMessage { message: String },
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, Eq, PartialEq)]
pub enum ServerResponse {
    Error(ServerResponseError),
}

#[derive(thiserror::Error, Debug, Clone, Hash, Serialize, Deserialize, Eq, PartialEq)]
pub enum ServerResponseError {
    #[error("Unknown Request")]
    UnknownRequest,
}
