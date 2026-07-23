use thiserror::Error;
use weaverail_model::error::ModelError;

pub mod project_file;

#[derive(Error, Debug)]
pub enum WeaverailIoError {
    #[error("File is invalid format")]
    InvalidMagicNumber,
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("weaverail model error")]
    ModelError(#[from] ModelError),
    #[error("Metadata text encoding format is invalid")]
    InvalidMetadataEncodingFormat(#[from] std::str::Utf8Error),
    #[error("ron deserializing error")]
    RonDeserializeError(#[from] ron::de::SpannedError),
    #[error("ron serializing error")]
    RonSerializeError(#[from] ron::error::Error),
}

/// Weaverailプロジェクトファイルを読み込む関数
pub fn read_project_file() {}

/// Weaverailプロジェクトファイルを書き込む関数
pub fn write_project_file() {}
