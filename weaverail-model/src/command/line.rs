use std::sync::Mutex;

use crate::{
    app::AppState,
    command::{Command, CommandError},
    model::{
        DiagramRoot,
        line::{Line, LineSegment},
    },
};

use tauri::{AppHandle, Emitter};
use uuid::Uuid;

/// 路線の追加
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddLineCommand {
    line: Line,
}
impl AddLineCommand {
    pub fn new(line: Line) -> Self {
        Self { line }
    }
}
impl Command for AddLineCommand {
    fn redo(
        &mut self,
        obj: &mut crate::model::DiagramRoot,
        app: Option<&tauri::AppHandle>,
    ) -> Result<(), super::CommandError> {
        obj.add_line(self.line.clone())?;
        if let Some(app) = app {
            let _ = app.emit_filter("line_changed", &obj, |_| true);
        }
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut crate::model::DiagramRoot,
        app: Option<&tauri::AppHandle>,
    ) -> Result<(), super::CommandError> {
        obj.delete_line(self.line.id)?;
        if let Some(app) = app {
            let _ = app.emit_filter("line_changed", &obj, |_| true);
        }

        Ok(())
    }
}

#[tauri::command]
pub async fn add_line(state: tauri::State<'_, Mutex<AppState>>, line: Line) -> Result<(), String> {
    let command = AddLineCommand::new(line.clone());
    let mut state = state.lock().expect("mutex lock error");

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}

/// 路線を削除する
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RemoveLineCommand {
    line_id: Uuid,
    line: Option<Line>,
}
impl RemoveLineCommand {
    pub fn new(line_id: Uuid) -> Self {
        Self {
            line_id,
            line: None,
        }
    }
}

impl Command for RemoveLineCommand {
    fn redo(&mut self, obj: &mut DiagramRoot, app: Option<&AppHandle>) -> Result<(), CommandError> {
        let line = obj.delete_line(self.line_id)?;
        self.line = Some(line);
        if let Some(app) = app {
            let _ = app.emit_filter("line_changed", &obj, |_| true);
        }
        Ok(())
    }

    fn undo(&mut self, obj: &mut DiagramRoot, app: Option<&AppHandle>) -> Result<(), CommandError> {
        if let Some(line) = self.line.clone() {
            obj.add_line(line)?;
        }

        if let Some(app) = app {
            let _ = app.emit_filter("line_changed", &obj, |_| true);
        }

        Ok(())
    }
}
#[tauri::command]
pub async fn remove_line(
    state: tauri::State<'_, Mutex<AppState>>,
    line_id: Uuid,
) -> Result<(), String> {
    let command = RemoveLineCommand::new(line_id);
    let mut state = state.lock().expect("mutex lock error");

    let command_manager = &mut state.command_manager;
    command_manager.execute(Box::new(command));

    Ok(())
}

/// 路線に駅間を追加する
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AppendSegmentToLine {
    line_id: Uuid,
    segment: LineSegment,
}
impl AppendSegmentToLine {
    pub fn new(line_id: Uuid, segment: LineSegment) -> Self {
        Self { line_id, segment }
    }
}

impl Command for AppendSegmentToLine {
    fn redo(&mut self, obj: &mut DiagramRoot, app: Option<&AppHandle>) -> Result<(), CommandError> {
        obj.append_segment(self.line_id, self.segment.clone())?;
        if let Some(app) = app {
            let _ = app.emit_filter("line_changed", &obj, |_| true);
        }
        Ok(())
    }

    fn undo(&mut self, _obj: &mut DiagramRoot, _app: Option<&AppHandle>) -> Result<(), CommandError> {
        todo!();
    }
}
