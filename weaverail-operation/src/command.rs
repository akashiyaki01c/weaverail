use weaverail_model::{
    error::ModelError,
    event::EmitEventType,
    model::DiagramRoot,
};

pub mod command_manager;
pub mod line;
pub mod segment;
pub mod station;
pub mod template_train;
pub mod template_train_segment;
pub mod timetable;
pub mod track;
pub mod train;
pub mod train_type;

/// フロントエンド側へのイベント通知を行う構造体
pub trait EventEmitter: Send + Sync {
    /// フロントエンド側にイベント通知を行なう
    fn emit(&self, event: EmitEventType, payload: &str);
}

/// フロントエンド側へのイベント通知を行わない空モジュール
pub struct EmptyEventEmitter;
impl EventEmitter for EmptyEventEmitter {
    fn emit(&self, _event: EmitEventType, _payload: &str) {
        // no-op
    }
}

/// モデルに対する「操作」を表すトレイト
pub trait Command: Send + Sync {
    /// やり直す動作
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError>;

    /// 元に戻す動作
    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError>;
}

/// コマンドのエラー一覧
#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize, thiserror::Error, strum::Display)]
pub enum CommandError {
    /// モデルのエラー
    #[error(transparent)]
    ModelError(#[from] ModelError),
    /// 対象オブジェクトが見つからない
    TargetObjectNotFound,
    /// オブジェクトのキーが重複している
    DuplicateKey,
    /// 外部参照されている
    ExternalReference,
    /// 範囲外の要素を参照した
    IndexOutOfBounds,
    /// 整合性がない
    Inconsistent,
    /// データをロックできなかった
    MutexLockError,
}

pub use command_manager::CommandManager;
