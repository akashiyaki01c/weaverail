use weaverail_model::{
    metadata::Metadata,
    model::DiagramRoot,
};

use super::{Command, CommandError, EventEmitter};

/// モデルに対する操作を管理する構造体
pub struct CommandManager {
    pub root: DiagramRoot,
    pub metadata: Metadata,
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
    pub emitter: Box<dyn EventEmitter>,
}

impl CommandManager {
    pub fn new(emitter: Box<dyn EventEmitter>) -> Self {
        Self {
            emitter,
            metadata: Metadata::default(),
            root: DiagramRoot::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// 操作を実行する
    pub fn execute(&mut self, mut cmd: Box<dyn Command>) {
        if cmd.redo(&mut self.root, self.emitter.as_ref()).is_ok() {
            self.undo_stack.push(cmd);
            self.redo_stack.clear();
        }
        self.root.version += 1;
    }

    /// 元に戻す
    pub fn undo(&mut self) -> Option<Result<(), CommandError>> {
        if let Some(mut cmd) = self.undo_stack.pop() {
            let undo: Result<(), CommandError> = cmd.undo(&mut self.root, self.emitter.as_ref());
            self.redo_stack.push(cmd);
            return Some(undo);
        }
        self.root.version -= 1;
        None
    }

    /// やり直す
    pub fn redo(&mut self) -> Option<Result<(), CommandError>> {
        if let Some(mut cmd) = self.redo_stack.pop() {
            let redo = cmd.redo(&mut self.root, self.emitter.as_ref());
            self.undo_stack.push(cmd);
            return Some(redo);
        }
        self.root.version += 1;
        None
    }

    /// もとに戻すことが可能か
    pub fn undoable(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// やり直すことが可能か
    pub fn redoable(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
