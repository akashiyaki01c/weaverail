use serde::{Deserialize, Serialize};
use thiserror::Error;

/// モデル操作上のエラーを表す
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Error)]
pub enum ModelError {
    #[error("object is not found.")]
    ObjectNotFound,
    #[error("key is duplicated.")]
    DuplicateKey,
    #[error("external referenced.")]
    ExternalReferenced,
    #[error("external referenced.")]
    Error,
    #[error("empty")]
    Empty,
}
