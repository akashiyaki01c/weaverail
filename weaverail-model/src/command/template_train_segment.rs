use crate::{
    command::{Command, CommandError, EventEmitter},
    event::EmitEventType,
    model::{
        DiagramRoot,
        template_train::{TemplateTrainId, TemplateTrainSegment, TemplateTrainStation},
    },
};

/// テンプレート列車の末尾に駅間を追加する操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct PushBackTemplateTrainSegmentCommand {
    template_train_id: TemplateTrainId,
    template_segment: TemplateTrainSegment,
    template_station: TemplateTrainStation,
}
impl PushBackTemplateTrainSegmentCommand {
    pub fn new(
        template_train_id: TemplateTrainId,
        template_segment: TemplateTrainSegment,
        template_station: TemplateTrainStation,
    ) -> Self {
        Self {
            template_train_id,
            template_segment,
            template_station,
        }
    }
}
impl Command for PushBackTemplateTrainSegmentCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let template_train = obj
            .template_trains
            .get_mut(&self.template_train_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        template_train
            .segments
            .push((self.template_segment.clone(), self.template_station.clone()));
        emitter.emit(EmitEventType::TemplateTrainSegmentPushed, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let template_train = obj
            .template_trains
            .get_mut(&self.template_train_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        if !template_train.segments.is_empty()
            && template_train.segments.last().unwrap().0.id != self.template_segment.id
            && template_train.segments.last().unwrap().1.id != self.template_station.id
        {
            return Err(CommandError::Inconsistent);
        }
        template_train.segments.pop();
        emitter.emit(EmitEventType::TemplateTrainSegmentPoped, "");
        Ok(())
    }
}
