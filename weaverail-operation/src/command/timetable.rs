use weaverail_model::{
    event::EmitEventType,
    model::{
        DiagramRoot,
        timetable::{Timetable, TimetableId},
    },
};

use crate::command::{Command, CommandError, EventEmitter};

/// 時刻表追加操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddTimetableCommand {
    timetable: Timetable,
}

impl AddTimetableCommand {
    pub fn new(timetable: Timetable) -> Self {
        Self { timetable }
    }
}

impl Command for AddTimetableCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.add_timetable(self.timetable.clone())?;
        emitter.emit(EmitEventType::TimetableAdded, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.delete_timetable(self.timetable.id)?;
        emitter.emit(EmitEventType::TimetableDeleted, "");
        Ok(())
    }
}

/// 時刻表削除操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RemoveTimetableCommand {
    timetable_id: TimetableId,
    timetable: Option<Timetable>,
}

impl RemoveTimetableCommand {
    pub fn new(timetable_id: TimetableId) -> Self {
        Self {
            timetable_id,
            timetable: None,
        }
    }
}

impl Command for RemoveTimetableCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let timetable = obj.delete_timetable(self.timetable_id)?;
        self.timetable = Some(timetable);
        emitter.emit(EmitEventType::TimetableDeleted, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        if let Some(timetable) = self.timetable.clone() {
            obj.add_timetable(timetable)?;
        }
        emitter.emit(EmitEventType::TimetableAdded, "");
        Ok(())
    }
}
