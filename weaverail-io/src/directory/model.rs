use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use weaverail_model::{id_issuer::IdIssuer, metadata::Metadata, model::ExtensionProperty};

/// ディレクトリのバージョン
#[derive(Serialize, Deserialize)]
pub enum DirectoryVersion {
    /// version 0.1
    V01 = 1,
}

/// ディレクトリのプロジェクト設定を表すファイル
#[derive(Serialize, Deserialize)]
pub struct DirectoryProject {
    /// ディレクトリのバージョン
    pub directory_version_format: DirectoryVersion,
    /// パス設定
    pub path: PathSettings,
}
impl Default for DirectoryProject {
    fn default() -> Self {
        Self {
            directory_version_format: DirectoryVersion::V01,
            path: Default::default(),
        }
    }
}

/// パス設定
#[derive(Serialize, Deserialize)]
pub struct PathSettings {
    pub root_path: PathBuf,
    pub stations_path: PathBuf,
    pub tracks_path: PathBuf,
    pub segments_path: PathBuf,
    pub lines_path: PathBuf,
    pub train_types_path: PathBuf,
    pub template_trains_path: PathBuf,
    pub timetables_path: Vec<PathBuf>,
    pub trains_path: Vec<PathBuf>,
}
impl Default for PathSettings {
    fn default() -> Self {
        Self {
            root_path: PathBuf::from("model").join("root.ron"),
            stations_path: PathBuf::from("model").join("stations.ron"),
            tracks_path: PathBuf::from("model").join("tracks.ron"),
            segments_path: PathBuf::from("model").join("segments.ron"),
            lines_path: PathBuf::from("model").join("lines.ron"),
            train_types_path: PathBuf::from("model").join("train_types.ron"),
            template_trains_path: PathBuf::from("model").join("template_train.ron"),
            timetables_path: vec![],
            trains_path: vec![],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RootFile {
    pub properties: ExtensionProperty,
    pub id_issuer: IdIssuer,
    pub version: u32,
    pub metadata: Metadata,
}
