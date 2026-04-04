use tauri::AppHandle;

use crate::{
    command::{Command, CommandError},
    model::DiagramRoot,
};

/// モデルに対する操作を管理する構造体
pub struct CommandManager {
    app_handle: Option<AppHandle>,
    pub root: DiagramRoot,
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}
impl CommandManager {
    pub fn new(app_handle: Option<AppHandle>) -> Self {
        Self {
            app_handle,
            root: DiagramRoot::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// 操作を実行する
    pub fn execute(&mut self, mut cmd: Box<dyn Command>) {
        if cmd.redo(&mut self.root, self.app_handle.as_ref()).is_ok() {
            self.undo_stack.push(cmd);
            self.redo_stack.clear();
        }
    }

    /// 元に戻す
    pub fn undo(&mut self) -> Option<Result<(), CommandError>> {
        if let Some(mut cmd) = self.undo_stack.pop() {
            let undo: Result<(), CommandError> = cmd.undo(&mut self.root, self.app_handle.as_ref());
            self.redo_stack.push(cmd);
            return Some(undo);
        }
        None
    }

    /// やり直す
    pub fn redo(&mut self) -> Option<Result<(), CommandError>> {
        if let Some(mut cmd) = self.redo_stack.pop() {
            let redo = cmd.redo(&mut self.root, self.app_handle.as_ref());
            self.undo_stack.push(cmd);
            return Some(redo);
        }
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
