use weaverail_model::{
    event::EmitEventType,
    model::{
        DiagramRoot, TemplateTrainSegment, TemplateTrainStation,
        template_train::{TemplateTrain, TemplateTrainId, TemplateTrainSection},
    },
};

use crate::command::{Command, CommandError, EventEmitter};

/// テンプレート列車追加操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddTemplateTrainCommand {
    template_train: TemplateTrain,
}

impl AddTemplateTrainCommand {
    pub fn new(template_train: TemplateTrain) -> Self {
        Self { template_train }
    }
}

impl Command for AddTemplateTrainCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.add_template_train(self.template_train.clone())?;
        emitter.emit(EmitEventType::TemplateTrainAdded, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.delete_template_train(self.template_train.id)?;
        emitter.emit(EmitEventType::TemplateTrainDeleted, "");
        Ok(())
    }
}

/// テンプレート列車削除操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RemoveTemplateTrainCommand {
    template_train_id: TemplateTrainId,
    template_train: Option<TemplateTrain>,
}

impl RemoveTemplateTrainCommand {
    pub fn new(template_train_id: TemplateTrainId) -> Self {
        Self {
            template_train_id,
            template_train: None,
        }
    }
}

impl Command for RemoveTemplateTrainCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let template_train = obj.delete_template_train(self.template_train_id)?;
        self.template_train = Some(template_train);
        emitter.emit(EmitEventType::TemplateTrainDeleted, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        if let Some(template_train) = self.template_train.clone() {
            obj.add_template_train(template_train)?;
        }
        emitter.emit(EmitEventType::TemplateTrainAdded, "");
        Ok(())
    }
}

/// テンプレート列車名変更操作。
///
/// 変更前の名前を保持するため、`undo`で元の名前へ戻せる。
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RenameTemplateTrainCommand {
    template_train_id: TemplateTrainId,
    old_name: Option<String>,
    new_name: String,
}

impl RenameTemplateTrainCommand {
    pub fn new(template_train_id: TemplateTrainId, new_name: &str) -> Self {
        Self {
            template_train_id,
            old_name: None,
            new_name: new_name.to_string(),
        }
    }
}

impl Command for RenameTemplateTrainCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let train = obj
            .template_trains
            .get_mut(&self.template_train_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        self.old_name = Some(train.name.clone());
        train.name = self.new_name.clone();
        emitter.emit(EmitEventType::TemplateTrainAdded, "");
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
        if let Some(name) = &self.old_name {
            train.name = name.clone();
        }
        emitter.emit(EmitEventType::TemplateTrainAdded, "");
        Ok(())
    }
}

/// テンプレート列車区間の先頭追加操作。
///
/// モデルの接続検証を通した後、先頭へ区間を配置する。
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

/// テンプレート列車の末尾区間削除操作。
///
/// 削除した区間全体を保持し、`undo`で末尾へ復元する。
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
        let train = obj
            .template_trains
            .get_mut(&self.template_train_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        train.segments.push(section);
        emitter.emit(EmitEventType::TemplateTrainSegmentPushed, "");
        Ok(())
    }
}

/// テンプレート列車の先頭区間削除操作。
///
/// 削除した区間全体を保持し、`undo`で先頭へ復元する。
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
        let train = obj
            .template_trains
            .get_mut(&self.template_train_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        train.segments.insert(0, section);
        emitter.emit(EmitEventType::TemplateTrainSegmentPushed, "");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaverail_model::model::id::WeaverailId;

    #[test]
    fn rename_template_train_command_restores_name() {
        let mut root = DiagramRoot::default();
        let id = TemplateTrainId::new(WeaverailId::new(1));
        root.add_template_train(TemplateTrain {
            id,
            name: "up".to_string(),
            ..Default::default()
        })
        .unwrap();
        let mut command = RenameTemplateTrainCommand::new(id, "down");
        command
            .redo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.template_trains[&id].name, "down");
        command
            .undo(&mut root, &crate::command::EmptyEventEmitter)
            .unwrap();
        assert_eq!(root.template_trains[&id].name, "up");
    }

    #[test]
    fn rename_template_train_command_handles_missing_train() {
        let mut root = DiagramRoot::default();
        let invalid_id = TemplateTrainId::new(WeaverailId::new(99));
        let mut command = RenameTemplateTrainCommand::new(invalid_id, "new");
        assert_eq!(
            command.redo(&mut root, &crate::command::EmptyEventEmitter),
            Err(CommandError::TargetObjectNotFound)
        );
    }
}
