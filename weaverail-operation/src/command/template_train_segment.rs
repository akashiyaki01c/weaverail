use weaverail_model::{
    event::EmitEventType,
    model::{
        DiagramRoot,
        template_train::{
            TemplateTrainId, TemplateTrainSection, TemplateTrainSegment, TemplateTrainStation,
        },
    },
};

use crate::command::{Command, CommandError, EventEmitter};

/// テンプレート列車の駅間追加操作。
///
/// `redo`で末尾へ追加し、`undo`で末尾から削除する。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddTemplateTrainSegmentCommand {
    template_train_id: TemplateTrainId,
    segment: TemplateTrainSegment,
    station: TemplateTrainStation,
}

impl AddTemplateTrainSegmentCommand {
    pub fn new(
        template_train_id: TemplateTrainId,
        segment: TemplateTrainSegment,
        station: TemplateTrainStation,
    ) -> Self {
        Self {
            template_train_id,
            segment,
            station,
        }
    }
}

impl Command for AddTemplateTrainSegmentCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.push_back_template_segment(
            self.template_train_id,
            self.segment.clone(),
            self.station.clone(),
        )?;
        emitter.emit(EmitEventType::TemplateTrainSegmentPushed, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.pop_back_template_segment(self.template_train_id)?;
        emitter.emit(EmitEventType::TemplateTrainSegmentPoped, "");
        Ok(())
    }
}

/// テンプレート列車の末尾に駅間を追加する操作（ドキュメント上の名称）。
pub type PushBackTemplateTrainSegmentCommand = AddTemplateTrainSegmentCommand;

/// テンプレート列車の先頭に駅間を追加する操作。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PushFrontTemplateTrainSegmentCommand {
    template_train_id: TemplateTrainId,
    segment: TemplateTrainSegment,
    station: TemplateTrainStation,
}

impl PushFrontTemplateTrainSegmentCommand {
    pub fn new(
        template_train_id: TemplateTrainId,
        segment: TemplateTrainSegment,
        station: TemplateTrainStation,
    ) -> Self {
        Self {
            template_train_id,
            segment,
            station,
        }
    }
}

impl Command for PushFrontTemplateTrainSegmentCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.push_front_template_segment(
            self.template_train_id,
            self.segment.clone(),
            self.station.clone(),
        )?;
        let train = obj
            .template_trains
            .get_mut(&self.template_train_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        let section = train.segments.pop().ok_or(CommandError::Inconsistent)?;
        train.segments.insert(0, section);
        emitter.emit(EmitEventType::TemplateTrainSegmentPushed, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let train = obj
            .template_trains
            .get_mut(&self.template_train_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        if train.segments.first().map(|section| section.segment.id) != Some(self.segment.id) {
            return Err(CommandError::Inconsistent);
        }
        train.segments.remove(0);
        emitter.emit(EmitEventType::TemplateTrainSegmentPoped, "");
        Ok(())
    }
}

/// テンプレート列車の末尾駅間を削除する操作。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PopBackTemplateTrainSegmentCommand {
    template_train_id: TemplateTrainId,
    section: Option<TemplateTrainSection>,
}

impl PopBackTemplateTrainSegmentCommand {
    pub fn new(template_train_id: TemplateTrainId) -> Self {
        Self {
            template_train_id,
            section: None,
        }
    }
}

impl Command for PopBackTemplateTrainSegmentCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let section = obj
            .template_trains
            .get(&self.template_train_id)
            .and_then(|train| train.segments.last())
            .cloned()
            .ok_or(CommandError::TargetObjectNotFound)?;
        obj.pop_back_template_segment(self.template_train_id)?;
        self.section = Some(section);
        emitter.emit(EmitEventType::TemplateTrainSegmentPoped, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let section = self.section.clone().ok_or(CommandError::Inconsistent)?;
        obj.template_trains
            .get_mut(&self.template_train_id)
            .ok_or(CommandError::TargetObjectNotFound)?
            .segments
            .push(section);
        emitter.emit(EmitEventType::TemplateTrainSegmentPushed, "");
        Ok(())
    }
}

/// テンプレート列車の先頭駅間を削除する操作。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PopFrontTemplateTrainSegmentCommand {
    template_train_id: TemplateTrainId,
    section: Option<TemplateTrainSection>,
}

impl PopFrontTemplateTrainSegmentCommand {
    pub fn new(template_train_id: TemplateTrainId) -> Self {
        Self {
            template_train_id,
            section: None,
        }
    }
}

impl Command for PopFrontTemplateTrainSegmentCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let section = obj
            .template_trains
            .get(&self.template_train_id)
            .and_then(|train| train.segments.first())
            .cloned()
            .ok_or(CommandError::TargetObjectNotFound)?;
        obj.pop_front_template_segment(self.template_train_id)?;
        self.section = Some(section);
        emitter.emit(EmitEventType::TemplateTrainSegmentPoped, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let section = self.section.clone().ok_or(CommandError::Inconsistent)?;
        obj.template_trains
            .get_mut(&self.template_train_id)
            .ok_or(CommandError::TargetObjectNotFound)?
            .segments
            .insert(0, section);
        emitter.emit(EmitEventType::TemplateTrainSegmentPushed, "");
        Ok(())
    }
}

/// テンプレート列車の駅間を置換する操作。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ReplaceTemplateTrainSegmentCommand {
    template_train_id: TemplateTrainId,
    target_segment_id: weaverail_model::model::template_train::TemplateTrainSegmentId,
    replacement: TemplateTrainSection,
    original: Option<TemplateTrainSection>,
}

impl ReplaceTemplateTrainSegmentCommand {
    pub fn new(
        template_train_id: TemplateTrainId,
        target_segment_id: weaverail_model::model::template_train::TemplateTrainSegmentId,
        replacement: TemplateTrainSection,
    ) -> Self {
        Self {
            template_train_id,
            target_segment_id,
            replacement,
            original: None,
        }
    }
}

impl Command for ReplaceTemplateTrainSegmentCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let train = obj
            .template_trains
            .get_mut(&self.template_train_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        let section = train
            .segments
            .iter_mut()
            .find(|section| section.segment.id == self.target_segment_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        self.original = Some(std::mem::replace(section, self.replacement.clone()));
        emitter.emit(EmitEventType::TemplateTrainSegmentPushed, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let train = obj
            .template_trains
            .get_mut(&self.template_train_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        let section = train
            .segments
            .iter_mut()
            .find(|section| section.segment.id == self.replacement.segment.id)
            .ok_or(CommandError::Inconsistent)?;
        *section = self.original.clone().ok_or(CommandError::Inconsistent)?;
        emitter.emit(EmitEventType::TemplateTrainSegmentPoped, "");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::{
        id::WeaverailId,
        line_segment::{LineSegment, LineSegmentId},
        station::{Station, StationId},
        template_train::{TemplateTrain, TemplateTrainSegmentId, TemplateTrainStationId},
    };

    fn fixture() -> (
        DiagramRoot,
        TemplateTrainId,
        TemplateTrainSegment,
        TemplateTrainStation,
    ) {
        let mut root = DiagramRoot::default();
        let start = StationId::new(WeaverailId::new(1));
        let end = StationId::new(WeaverailId::new(2));
        let segment_id = LineSegmentId::new(WeaverailId::new(3));
        let train_id = TemplateTrainId::new(WeaverailId::new(4));
        root.add_station(Station::new(start, "A")).unwrap();
        root.add_station(Station::new(end, "B")).unwrap();
        root.add_segment(LineSegment::new(segment_id, start, end))
            .unwrap();
        root.add_template_train(TemplateTrain {
            id: train_id,
            start_station: TemplateTrainStation {
                id: TemplateTrainStationId::new(WeaverailId::new(5)),
                station_id: start,
                ..Default::default()
            },
            ..Default::default()
        })
        .unwrap();
        let segment = TemplateTrainSegment {
            id: TemplateTrainSegmentId::new(WeaverailId::new(6)),
            segment_id,
            is_reversed: false,
            ..Default::default()
        };
        let station = TemplateTrainStation {
            id: TemplateTrainStationId::new(WeaverailId::new(7)),
            station_id: end,
            ..Default::default()
        };
        (root, train_id, segment, station)
    }

    #[test]
    fn template_segment_commands_restore_sections() {
        let (mut root, train_id, segment, station) = fixture();
        let emitter = crate::command::EmptyEventEmitter;

        let mut add =
            PushBackTemplateTrainSegmentCommand::new(train_id, segment.clone(), station.clone());
        add.redo(&mut root, &emitter).unwrap();
        assert_eq!(root.template_trains[&train_id].segments.len(), 1);
        add.undo(&mut root, &emitter).unwrap();
        assert!(root.template_trains[&train_id].segments.is_empty());

        add.redo(&mut root, &emitter).unwrap();
        let front = TemplateTrainSegment {
            id: TemplateTrainSegmentId::new(WeaverailId::new(9)),
            is_reversed: true,
            ..segment.clone()
        };
        let mut push_front =
            PushFrontTemplateTrainSegmentCommand::new(train_id, front.clone(), station.clone());
        push_front.redo(&mut root, &emitter).unwrap();
        assert_eq!(
            root.template_trains[&train_id].segments[0].segment.id,
            front.id
        );
        push_front.undo(&mut root, &emitter).unwrap();
        assert_eq!(root.template_trains[&train_id].segments.len(), 1);

        let mut pop_back = PopBackTemplateTrainSegmentCommand::new(train_id);
        pop_back.redo(&mut root, &emitter).unwrap();
        assert!(root.template_trains[&train_id].segments.is_empty());
        pop_back.undo(&mut root, &emitter).unwrap();
        assert_eq!(
            root.template_trains[&train_id].segments[0].segment.id,
            segment.id
        );

        let mut replace = ReplaceTemplateTrainSegmentCommand::new(
            train_id,
            segment.id,
            TemplateTrainSection {
                segment: TemplateTrainSegment {
                    id: TemplateTrainSegmentId::new(WeaverailId::new(8)),
                    ..segment.clone()
                },
                station: station.clone(),
            },
        );
        replace.redo(&mut root, &emitter).unwrap();
        assert_ne!(
            root.template_trains[&train_id].segments[0].segment.id,
            segment.id
        );
        replace.undo(&mut root, &emitter).unwrap();
        assert_eq!(
            root.template_trains[&train_id].segments[0].segment.id,
            segment.id
        );

        let mut pop_front = PopFrontTemplateTrainSegmentCommand::new(train_id);
        pop_front.redo(&mut root, &emitter).unwrap();
        assert!(root.template_trains[&train_id].segments.is_empty());
        pop_front.undo(&mut root, &emitter).unwrap();
        assert_eq!(root.template_trains[&train_id].segments.len(), 1);
    }

    #[test]
    fn template_segment_commands_handle_missing_entities() {
        let mut root = DiagramRoot::default();
        let invalid_train = TemplateTrainId::new(WeaverailId::new(99));

        let mut pop_back = PopBackTemplateTrainSegmentCommand::new(invalid_train);
        assert_eq!(
            pop_back.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::TargetObjectNotFound)
        );

        let mut pop_front = PopFrontTemplateTrainSegmentCommand::new(invalid_train);
        assert_eq!(
            pop_front.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::TargetObjectNotFound)
        );
    }

    #[test]
    fn template_segment_commands_handle_empty_collections() {
        let mut root = DiagramRoot::default();
        let train_id = TemplateTrainId::new(WeaverailId::new(1));
        root.add_template_train(TemplateTrain {
            id: train_id,
            ..Default::default()
        })
        .unwrap();

        let mut pop_back = PopBackTemplateTrainSegmentCommand::new(train_id);
        assert_eq!(
            pop_back.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::TargetObjectNotFound)
        );

        let mut pop_front = PopFrontTemplateTrainSegmentCommand::new(train_id);
        assert_eq!(
            pop_front.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::TargetObjectNotFound)
        );
    }
}
