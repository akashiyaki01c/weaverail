use crate::{command::{Command, EventEmitter}, model::{DiagramRoot, timetable::Timetable}};

/// プロジェクトに時刻表を追加する操作
/// ID重複時にエラーを返す
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddTimetableCommand {
    timetable: Timetable,
}
impl AddTimetableCommand {
    pub fn new(line: Timetable) -> Self {
        Self { timetable: line }
    }
}
impl Command for AddTimetableCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), super::CommandError> {
        obj.add_timetable(self.timetable.clone())?;
        emitter.emit("timetable::added", "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), super::CommandError> {
        obj.delete_timetable(self.timetable.id)?;
        emitter.emit("timetable::removed", "");

        Ok(())
    }
}