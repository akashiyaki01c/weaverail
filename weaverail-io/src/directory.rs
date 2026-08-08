use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use ron::ser::PrettyConfig;
use weaverail_model::{metadata::Metadata, model::*};

use crate::{
    WeaverailIoError,
    directory::model::{DirectoryProject, RootFile},
};

mod model;

/// Weaverailプロジェクトファイルを読み込む関数
pub fn read_file(path: &Path) -> Result<(DiagramRoot, Metadata), WeaverailIoError> {
    let mut diagram_root = DiagramRoot::default();

    let project_file: DirectoryProject =
        ron::de::from_str(&std::fs::read_to_string(path.join("project.ron"))?)?;
    let root_file: RootFile = ron::de::from_str(&std::fs::read_to_string(
        path.join(project_file.path.root_path),
    )?)?;
    diagram_root.properties = root_file.properties;
    diagram_root.id_issuer = root_file.id_issuer;
    diagram_root.version = root_file.version;
    let metadata = root_file.metadata;

    let stations: IndexMap<StationId, Station> = ron::de::from_str(&std::fs::read_to_string(
        path.join(project_file.path.stations_path),
    )?)?;
    diagram_root.stations = stations;

    let tracks: IndexMap<TrackId, Track> = ron::de::from_str(&std::fs::read_to_string(
        path.join(project_file.path.tracks_path),
    )?)?;
    diagram_root.tracks = tracks;

    let segments: IndexMap<LineSegmentId, LineSegment> = ron::de::from_str(
        &std::fs::read_to_string(path.join(project_file.path.segments_path))?,
    )?;
    diagram_root.segments = segments;

    let train_types: IndexMap<TrainTypeId, TrainType> = ron::de::from_str(
        &std::fs::read_to_string(path.join(project_file.path.train_types_path))?,
    )?;
    diagram_root.train_types = train_types;

    let template_trains: IndexMap<TemplateTrainId, TemplateTrain> = ron::de::from_str(
        &std::fs::read_to_string(path.join(project_file.path.template_trains_path))?,
    )?;
    diagram_root.template_trains = template_trains;

    for timetable_path in project_file.path.timetables_path {
        let timetable: Timetable =
            ron::de::from_str(&std::fs::read_to_string(path.join(timetable_path))?)?;
        diagram_root.timetables.insert(timetable.id, timetable);
    }

    for train_path in project_file.path.trains_path {
        let trains: Vec<Train> =
            ron::de::from_str(&std::fs::read_to_string(path.join(train_path))?)?;
        for train in trains {
            diagram_root.trains.insert(train.id, train);
        }
    }

    Ok((diagram_root, metadata))
}

/// Weaverailプロジェクトファイルを書き込む関数
pub fn write_file(
    path: &PathBuf,
    root: &DiagramRoot,
    metadata: &Metadata,
) -> Result<(), WeaverailIoError> {
    if !std::fs::exists(path)? {
        std::fs::create_dir(path)?;
    }

    let _ = &root.validate()?;
    let pretty_config = PrettyConfig::new()
        .new_line("\n")
        .indentor("\t")
        .number_suffixes(true)
        .escape_strings(true)
        .struct_names(true);

    let mut project = DirectoryProject::default();
    for timetable in root.timetables.keys() {
        project.path.timetables_path.push(
            PathBuf::from("model")
                .join("timetables")
                .join(format!("{}.ron", timetable.to_string())),
        );
    }
    for timetable in root.timetables.keys() {
        project.path.trains_path.push(
            PathBuf::from("model")
                .join("trains")
                .join(format!("{}.ron", timetable.to_string())),
        );
    }
    std::fs::write(
        path.join("project.ron"),
        ron::ser::to_string_pretty(&project, pretty_config.clone())
            .map_err(WeaverailIoError::RonSerializeError)?,
    )
    .map_err(WeaverailIoError::Io)?;

    if !std::fs::exists(path.join("model"))? {
        std::fs::create_dir(path.join("model"))?;
    }

    let root_file = RootFile {
        properties: root.properties.clone(),
        id_issuer: root.id_issuer.clone(),
        version: root.version,
        metadata: metadata.clone(),
    };
    std::fs::write(
        path.join(project.path.root_path),
        ron::ser::to_string_pretty(&root_file, pretty_config.clone())?,
    )?;

    std::fs::write(
        path.join(project.path.stations_path),
        ron::ser::to_string_pretty(&root.stations, pretty_config.clone())?,
    )?;
    std::fs::write(
        path.join(project.path.tracks_path),
        ron::ser::to_string_pretty(&root.tracks, pretty_config.clone())?,
    )?;
    std::fs::write(
        path.join(project.path.segments_path),
        ron::ser::to_string_pretty(&root.segments, pretty_config.clone())?,
    )?;
    std::fs::write(
        path.join(project.path.lines_path),
        ron::ser::to_string_pretty(&root.lines, pretty_config.clone())?,
    )?;
    std::fs::write(
        path.join(project.path.train_types_path),
        ron::ser::to_string_pretty(&root.train_types, pretty_config.clone())?,
    )?;
    std::fs::write(
        path.join(project.path.template_trains_path),
        ron::ser::to_string_pretty(&root.template_trains, pretty_config.clone())?,
    )?;
    if !std::fs::exists(path.join("model").join("timetables"))? {
        std::fs::create_dir(path.join("model").join("timetables"))?;
    }
    for timetable in root.timetables.keys() {
        std::fs::write(
            path.join("model")
                .join("timetables")
                .join(PathBuf::from(format!("{}.ron", timetable.to_string()))),
            ron::ser::to_string_pretty(
                &root.timetables.get(timetable).unwrap(),
                pretty_config.clone(),
            )?,
        )?;
    }

    if !std::fs::exists(path.join("model").join("trains"))? {
        std::fs::create_dir(path.join("model").join("trains"))?;
    }
    for timetable in root.timetables.keys() {
        std::fs::write(
            path.join("model")
                .join("trains")
                .join(PathBuf::from(format!("{}.ron", timetable.to_string()))),
            ron::ser::to_string_pretty(
                &root
                    .trains
                    .values()
                    .filter(|train| train.timetable_id == *timetable)
                    .collect::<Vec<_>>(),
                pretty_config.clone(),
            )?,
        )?;
    }

    Ok(())
}

#[test]
fn write_read() {
    let test_data = weaverail_model::test_data::diagram_root::get_test_data();
    let start = std::time::Instant::now();
    let path = PathBuf::from("./test.wvd");
    let result = write_file(&path, &test_data.root, &test_data.metadata);
    println!("{:?}", result);
    let duration = start.elapsed();
    println!("write_time: {}us", duration.as_micros());
    let start = std::time::Instant::now();
    let data = read_file(&path);
    let duration = start.elapsed();
    println!("read_time: {}us", duration.as_micros());
}
