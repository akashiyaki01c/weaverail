use weaverail_model::{
    event::EmitEventType,
    model::{
        DiagramRoot,
        time::Time,
        timetable::TimetableId,
        train::{TemplateSegment, Train, TrainId},
    },
};

use crate::command::{Command, CommandError, EventEmitter};

/// 列車追加操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddTrainCommand {
    timetable_id: TimetableId,
    train: Train,
}

impl AddTrainCommand {
    pub fn new(timetable_id: TimetableId, train: Train) -> Self {
        Self {
            timetable_id,
            train,
        }
    }
}

impl Command for AddTrainCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.add_train(self.train.clone())?;
        emitter.emit(EmitEventType::TrainAdded, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.delete_train(self.train.id)?;
        emitter.emit(EmitEventType::TrainDeleted, "");
        Ok(())
    }
}

/// 列車削除操作。
///
/// 削除した列車を保持し、`undo`で時刻表の順序からも復元する。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RemoveTrainCommand {
    train_id: TrainId,
    train: Option<Train>,
}
impl RemoveTrainCommand {
    pub fn new(train_id: TrainId) -> Self {
        Self {
            train_id,
            train: None,
        }
    }
}
impl Command for RemoveTrainCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        self.train = Some(obj.delete_train(self.train_id)?);
        emitter.emit(EmitEventType::TrainDeleted, "");
        Ok(())
    }
    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.add_train(self.train.clone().ok_or(CommandError::Inconsistent)?)?;
        emitter.emit(EmitEventType::TrainAdded, "");
        Ok(())
    }
}

/// 列車名変更操作。
///
/// 変更前の名前を保持するため、`undo`で元の名前へ戻せる。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RenameTrainCommand {
    train_id: TrainId,
    old_name: Option<String>,
    new_name: String,
}
impl RenameTrainCommand {
    pub fn new(train_id: TrainId, new_name: &str) -> Self {
        Self {
            train_id,
            old_name: None,
            new_name: new_name.to_string(),
        }
    }
}
impl Command for RenameTrainCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let train = obj
            .trains
            .get_mut(&self.train_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        self.old_name = Some(train.name.clone());
        train.name = self.new_name.clone();
        emitter.emit(EmitEventType::TrainAdded, "");
        Ok(())
    }
    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let train = obj
            .trains
            .get_mut(&self.train_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        train.name = self.old_name.clone().ok_or(CommandError::Inconsistent)?;
        emitter.emit(EmitEventType::TrainAdded, "");
        Ok(())
    }
}

/// 列車の始発時刻変更操作。
///
/// 変更前の時刻を保持するため、`undo`で元の時刻へ戻せる。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ChangeDepartureTimeCommand {
    train_id: TrainId,
    old_time: Option<Time>,
    new_time: Time,
}
impl ChangeDepartureTimeCommand {
    pub fn new(train_id: TrainId, new_time: Time) -> Self {
        Self {
            train_id,
            old_time: None,
            new_time,
        }
    }
}
impl Command for ChangeDepartureTimeCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let train = obj
            .trains
            .get_mut(&self.train_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        self.old_time = Some(train.start_departure_time);
        train.start_departure_time = self.new_time;
        emitter.emit(EmitEventType::TrainAdded, "");
        Ok(())
    }
    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.trains
            .get_mut(&self.train_id)
            .ok_or(CommandError::TargetObjectNotFound)?
            .start_departure_time = self.old_time.ok_or(CommandError::Inconsistent)?;
        emitter.emit(EmitEventType::TrainAdded, "");
        Ok(())
    }
}

/// 列車の運行区間を置換する操作。
///
/// 列車が参照するテンプレート区間列を一括置換し、元の列を`undo`で復元する。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ReplaceTrainOperationCommand {
    train_id: TrainId,
    old_segments: Option<Vec<TemplateSegment>>,
    new_segments: Vec<TemplateSegment>,
}
impl ReplaceTrainOperationCommand {
    pub fn new(train_id: TrainId, new_segments: Vec<TemplateSegment>) -> Self {
        Self {
            train_id,
            old_segments: None,
            new_segments,
        }
    }
}
impl Command for ReplaceTrainOperationCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let train = obj
            .trains
            .get_mut(&self.train_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        self.old_segments = Some(std::mem::replace(
            &mut train.template_segments,
            self.new_segments.clone(),
        ));
        emitter.emit(EmitEventType::TrainAdded, "");
        Ok(())
    }
    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.trains
            .get_mut(&self.train_id)
            .ok_or(CommandError::TargetObjectNotFound)?
            .template_segments = self
            .old_segments
            .clone()
            .ok_or(CommandError::Inconsistent)?;
        emitter.emit(EmitEventType::TrainAdded, "");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::{
        id::WeaverailId, station::StationId, template_train::TemplateTrainId,
    };

    #[test]
    fn train_commands_restore_values() {
        let mut root = DiagramRoot::default();
        let train_id = TrainId::new(WeaverailId::new(1));
        let timetable_id = TimetableId::new(WeaverailId::new(2));
        let template_id = TemplateTrainId::new(WeaverailId::new(3));
        let station_id = StationId::new(WeaverailId::new(4));
        let original = TemplateSegment {
            template_train_id: template_id,
            start_station_id: station_id,
            end_station_id: station_id,
        };
        let replacement = TemplateSegment {
            template_train_id: template_id,
            start_station_id: station_id,
            end_station_id: station_id,
        };
        let mut train = Train::new(train_id, timetable_id);
        train.name = "local".to_string();
        train.template_segments = vec![original.clone()];
        root.add_train(train).unwrap();

        let mut rename = RenameTrainCommand::new(train_id, "rapid");
        rename
            .redo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.trains[&train_id].name, "rapid");
        rename
            .undo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.trains[&train_id].name, "local");

        let new_time = Time::new(6, 30, 0);
        let mut departure = ChangeDepartureTimeCommand::new(train_id, new_time);
        departure
            .redo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.trains[&train_id].start_departure_time, new_time);
        departure
            .undo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.trains[&train_id].start_departure_time, Time::default());

        let mut replace = ReplaceTrainOperationCommand::new(train_id, vec![replacement]);
        replace
            .redo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.trains[&train_id].template_segments.len(), 1);
        replace
            .undo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.trains[&train_id].template_segments, vec![original]);

        let mut remove = RemoveTrainCommand::new(train_id);
        remove
            .redo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert!(!root.trains.contains_key(&train_id));
        remove
            .undo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert!(root.trains.contains_key(&train_id));
    }

    #[test]
    fn train_commands_handle_missing_train() {
        let mut root = DiagramRoot::default();
        let invalid_id = TrainId::new(WeaverailId::new(99));

        let mut rename = RenameTrainCommand::new(invalid_id, "new");
        assert_eq!(
            rename.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::TargetObjectNotFound)
        );

        let mut departure = ChangeDepartureTimeCommand::new(invalid_id, Time::new(6, 0, 0));
        assert_eq!(
            departure.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::TargetObjectNotFound)
        );

        let mut replace = ReplaceTrainOperationCommand::new(invalid_id, vec![]);
        assert_eq!(
            replace.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::TargetObjectNotFound)
        );

        let mut remove = RemoveTrainCommand::new(invalid_id);
        assert_eq!(
            remove.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::ModelError(
                weaverail_model::error::ModelError::ObjectNotFound
            ))
        );
    }
}
