use weaverail_model::{
    event::EmitEventType,
    model::{
        DiagramRoot,
        template_train::{TemplateTrainId, TemplateTrainSegment, TemplateTrainSection, TemplateTrainStation},
    },
};

use crate::command::{Command, CommandError, EventEmitter};

/// テンプレート列車の駅間追加操作
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
