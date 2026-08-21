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

/// 時刻表名変更操作。
///
/// 変更前の名前を保持するため、`undo`で元の名前へ戻せる。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RenameTimetableCommand {
    timetable_id: TimetableId,
    old_name: Option<String>,
    new_name: String,
}

impl RenameTimetableCommand {
    pub fn new(timetable_id: TimetableId, new_name: &str) -> Self {
        Self {
            timetable_id,
            old_name: None,
            new_name: new_name.to_string(),
        }
    }
}

impl Command for RenameTimetableCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let timetable = obj
            .timetables
            .get_mut(&self.timetable_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        self.old_name = Some(timetable.name.clone());
        timetable.name = self.new_name.clone();
        emitter.emit(EmitEventType::TimetableAdded, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let timetable = obj
            .timetables
            .get_mut(&self.timetable_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        if let Some(name) = &self.old_name {
            timetable.name = name.clone();
        }
        emitter.emit(EmitEventType::TimetableAdded, "");
        Ok(())
    }
}

/// 時刻表上の列車順序を移動する操作。
///
/// 指定方向の列車順序で、`from_index`の列車を`to_index`へ移動する。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ChangeTrainOrderCommand {
    timetable_id: TimetableId,
    segment_id: weaverail_model::model::line_segment::LineSegmentId,
    is_reversed: bool,
    from_index: usize,
    to_index: usize,
}

impl ChangeTrainOrderCommand {
    pub fn new(
        timetable_id: TimetableId,
        segment_id: weaverail_model::model::line_segment::LineSegmentId,
        is_reversed: bool,
        from_index: usize,
        to_index: usize,
    ) -> Self {
        Self {
            timetable_id,
            segment_id,
            is_reversed,
            from_index,
            to_index,
        }
    }
}

impl Command for ChangeTrainOrderCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let timetable = obj
            .timetables
            .get_mut(&self.timetable_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        let orders = timetable
            .segment_train_orders
            .get_mut(&self.segment_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        let order = if self.is_reversed {
            &mut orders.retrograde.order
        } else {
            &mut orders.prograde.order
        };
        if self.from_index >= order.len() || self.to_index >= order.len() {
            return Err(CommandError::IndexOutOfBounds);
        }
        let train = order.remove(self.from_index);
        order.insert(self.to_index, train);
        emitter.emit(EmitEventType::TimetableAdded, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        std::mem::swap(&mut self.from_index, &mut self.to_index);
        let result = self.redo(obj, emitter);
        std::mem::swap(&mut self.from_index, &mut self.to_index);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::{
        SegmentTrainOrders, id::WeaverailId, line_segment::LineSegmentId,
        segment_train_order::SegmentTrainOrder, train::TrainId,
    };

    #[test]
    fn timetable_commands_restore_name_and_train_order() {
        let mut root = DiagramRoot::default();
        let timetable_id = TimetableId::new(WeaverailId::new(1));
        let segment_id = LineSegmentId::new(WeaverailId::new(2));
        let first = TrainId::new(WeaverailId::new(3));
        let second = TrainId::new(WeaverailId::new(4));
        let orders = SegmentTrainOrders {
            prograde: SegmentTrainOrder {
                segment_id,
                order: vec![first, second],
                ..Default::default()
            },
            ..Default::default()
        };
        let mut timetable = Timetable::new(timetable_id, "weekday");
        timetable.segment_train_orders.insert(segment_id, orders);
        root.add_timetable(timetable).unwrap();

        let mut rename = RenameTimetableCommand::new(timetable_id, "holiday");
        rename
            .redo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.timetables[&timetable_id].name, "holiday");
        rename
            .undo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.timetables[&timetable_id].name, "weekday");

        let mut order = ChangeTrainOrderCommand::new(timetable_id, segment_id, false, 0, 1);
        order
            .redo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(
            root.timetables[&timetable_id].segment_train_orders[&segment_id]
                .prograde
                .order,
            vec![second, first]
        );
        order
            .undo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(
            root.timetables[&timetable_id].segment_train_orders[&segment_id]
                .prograde
                .order,
            vec![first, second]
        );

        let mut invalid = ChangeTrainOrderCommand::new(timetable_id, segment_id, false, 0, 2);
        assert_eq!(
            invalid.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::IndexOutOfBounds)
        );
    }

    #[test]
    fn timetable_commands_handle_missing_entities() {
        let mut root = DiagramRoot::default();
        let invalid_timetable = TimetableId::new(WeaverailId::new(99));
        let segment_id = LineSegmentId::new(WeaverailId::new(100));

        let mut rename = RenameTimetableCommand::new(invalid_timetable, "new");
        assert_eq!(
            rename.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::TargetObjectNotFound)
        );

        let mut order = ChangeTrainOrderCommand::new(invalid_timetable, segment_id, false, 0, 1);
        assert_eq!(
            order.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::TargetObjectNotFound)
        );
    }
}
