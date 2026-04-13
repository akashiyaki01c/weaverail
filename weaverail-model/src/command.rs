pub mod command_manager;
pub mod line;
pub mod station;
pub mod train_type;

use serde::{Deserialize, Serialize};

use crate::model::DiagramRoot;

pub trait EventEmitter: Send + Sync {
    fn emit(&self, event: &str, payload: &str);
}
pub struct EmptyEventEmitter;
impl EventEmitter for EmptyEventEmitter {
    fn emit(&self, _event: &str, _payload: &str) {
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
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum CommandError {
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
}

#[test]
fn check_env() {
    println!("TS_RS_EXPORT_DIR: {:?}", std::env::var("TS_RS_EXPORT_DIR"));
}
