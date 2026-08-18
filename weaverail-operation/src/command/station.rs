use weaverail_model::{
    event::EmitEventType,
    model::{
        DiagramRoot,
        station::{Station, StationId},
    },
};

use crate::command::{Command, CommandError, EventEmitter};

/// プロジェクトに駅を追加する操作
/// ID重複時にエラーを返す
#[derive(Clone, PartialEq, Debug, Default)]
pub struct AddStationCommand {
    station: Station,
}

impl AddStationCommand {
    pub fn new(station: Station) -> Self {
        Self { station }
    }
}

impl Command for AddStationCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.add_station(self.station.clone())?;
        emitter.emit(EmitEventType::StationAdded, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        obj.delete_station(self.station.id)?;
        emitter.emit(EmitEventType::StationDeleted, "");
        Ok(())
    }
}

/// プロジェクトから駅を削除する操作
/// 対象駅が存在しない場合はエラーを返す
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RemoveStationCommand {
    station_id: StationId,
    station: Option<Station>,
}

impl RemoveStationCommand {
    pub fn new(station_id: StationId) -> Self {
        Self {
            station_id,
            station: None,
        }
    }
}

impl Command for RemoveStationCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let station = obj.delete_station(self.station_id)?;
        self.station = Some(station);
        emitter.emit(EmitEventType::StationDeleted, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        if let Some(station) = self.station.clone() {
            obj.add_station(station)?;
        }
        emitter.emit(EmitEventType::StationAdded, "");
        Ok(())
    }
}

/// 指定駅の駅名を変更する操作
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RenameStationCommand {
    station_id: StationId,
    old_name: Option<String>,
    new_name: String,
}

impl RenameStationCommand {
    pub fn new(station_id: StationId, new_name: &str) -> Self {
        Self {
            station_id,
            old_name: None,
            new_name: new_name.to_string(),
        }
    }
}

impl Command for RenameStationCommand {
    fn redo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let station = obj
            .stations
            .get_mut(&self.station_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        self.old_name = Some(station.name.to_string());
        station.name = self.new_name.clone();
        emitter.emit(EmitEventType::StationRenamed, "");
        Ok(())
    }

    fn undo(
        &mut self,
        obj: &mut DiagramRoot,
        emitter: &dyn EventEmitter,
    ) -> Result<(), CommandError> {
        let station = obj
            .stations
            .get_mut(&self.station_id)
            .ok_or(CommandError::TargetObjectNotFound)?;
        if let Some(name) = &self.old_name {
            station.name = name.clone();
        }
        emitter.emit(EmitEventType::StationRenamed, "");
        Ok(())
    }
}
