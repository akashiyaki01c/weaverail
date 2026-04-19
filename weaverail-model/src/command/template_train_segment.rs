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
        obj.push_back_template_segment(
            self.template_train_id,
            self.template_segment.clone(),
            self.template_station.clone(),
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
