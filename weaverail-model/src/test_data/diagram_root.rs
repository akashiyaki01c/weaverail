/// テストデータを取得するクレート群
use crate::{
    command::{
        Command, EmptyEventEmitter,
        command_manager::CommandManager,
        line::AddLineCommand,
        segment::{AddSegmentCommand, PushBackSegmentCommand},
        station::AddStationCommand,
        track::AddTrackCommand,
        train_type::AddTrainTypeCommand,
    },
    model::{
        ExtensionProperty,
        diagram_view_settings::{DiagramViewSegment, DiagramViewSettings, DiagramViewSettingsId},
        line::{Line, LineId, SegmentRef},
        line_segment::{LineSegment, LineSegmentId},
        segment_train_order::SegmentTrainOrder,
        station::{Station, StationId},
        template_train::{
            StopType, TemplateTrain, TemplateTrainId, TemplateTrainSegment, TemplateTrainSegmentId,
            TemplateTrainStation, TemplateTrainStationId,
        },
        time::Time,
        timetable::{Timetable, TimetableId},
        track::{Track, TrackId},
        train::{TemplateSegment, Train, TrainId},
        train_type::{TrainType, TrainTypeId},
    },
};

fn remove_brackets(mut s: &str) -> &str {
    s = s.split("\t").collect::<Vec<&str>>()[0];
    if s.starts_with('(') && s.ends_with(')') {
        s.strip_prefix('(').unwrap().strip_suffix(')').unwrap()
    } else {
        s
    }
}

fn add_stations(manager: &mut CommandManager) {
    let station = include_str!("./station.csv");
    for sta_name in station.split("\n") {
        if sta_name.is_empty() {
            continue;
        }
        if sta_name == "駅名" {
            continue;
        }
        if sta_name.starts_with("(") {
            continue;
        }
        if sta_name.starts_with("#") {
            continue;
        }

        if sta_name.contains("\t") {
            let cell: Vec<&str> = sta_name.split("\t").collect();
            let station = Station::new(StationId::new(manager.root.id_issuer.next()), cell[0]);
            manager.execute(Box::new(AddStationCommand::new(station.clone())));
            for track_name in cell[1].split(",") {
                let track = Track::new(
                    TrackId::new(manager.root.id_issuer.next()),
                    station.id,
                    track_name,
                );
                manager.execute(Box::new(AddTrackCommand::new(track)));
            }
        } else {
            let id = manager.root.id_issuer.next();
            manager.execute(Box::new(AddStationCommand::new(Station::new(
                StationId::new(id),
                sta_name,
            ))));
        }
    }
}

fn add_lines(manager: &mut CommandManager) {
    let station = include_str!("./station.csv");
    for line_str in station.split("\n\n") {
        let id = LineId::new(manager.root.id_issuer.next());
        let mut line = Line::new(id, "", &[]);
        let mut commands: Vec<Box<dyn Command>> = Vec::new();
        let mut before_name = "".to_string();
        for sta_name in line_str.split("\n") {
            if sta_name.is_empty() {
                continue;
            }
            if sta_name.starts_with("#") {
                line.name = sta_name.replace("#", "").to_string();
                continue;
            }
            let sta_name = remove_brackets(sta_name);

            if before_name.is_empty() {
                before_name = sta_name.to_string();
                continue;
            }

            let id = manager.root.id_issuer.next();
            let before_station = manager.root.find_station_by_name(&before_name).unwrap();
            let current_station = manager.root.find_station_by_name(sta_name).unwrap();
            let segment = LineSegment::new(
                LineSegmentId::new(id),
                before_station.id,
                current_station.id,
            );
            commands.push(Box::new(AddSegmentCommand::new(segment.clone())));
            commands.push(Box::new(PushBackSegmentCommand::new(
                line.id, segment.id, false,
            )));
            before_name = sta_name.to_string();
        }
        manager.execute(Box::new(AddLineCommand::new(line)));
        for command in commands {
            manager.execute(command);
        }
    }
}

fn add_template_trains(manager: &mut CommandManager, input: &str) {
    let prefix = input.split("\t").next().unwrap();
    let train_type_name: Vec<&str> = input
        .split("\n")
        .next()
        .unwrap()
        .split("\t")
        .skip(2)
        .collect();
    for train_type_index in 0..train_type_name.len() {
        let template_id = TemplateTrainId::new(manager.root.id_issuer.next());
        let train_type = manager
            .root
            .find_train_type_by_name(train_type_name.get(train_type_index).unwrap())
            .unwrap_or_else(|| panic!("{:?}", train_type_name.get(train_type_index)));
        let mut template_train = TemplateTrain {
            id: template_id,
            name: format!("{}-{}", prefix, train_type.name),
            train_type_id: train_type.id,
            start_station: TemplateTrainStation::default(),
            segments: Vec::new(),
            properties: ExtensionProperty::new(),
        };
        for row in input.split("\n").skip(1) {
            let cells: Vec<&str> = row.split("\t").collect();
            let station_between_name = cells.first().unwrap();
            let start_station_name = station_between_name
                .split("→")
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
                .to_string();
            let start_station = manager
                .root
                .find_station_by_name(&start_station_name)
                .unwrap();
            let end_station_name = station_between_name
                .split("→")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .to_string();
            println!("{}", end_station_name);
            let end_station = manager
                .root
                .find_station_by_name(&end_station_name)
                .unwrap();
            let stop_time = str::parse(cells.get(1).unwrap()).unwrap();
            let is_passing = cells.get(2 + train_type_index).unwrap().ends_with("p");
            let running_time =
                str::parse(&cells.get(2 + train_type_index).unwrap().replace("p", "")).unwrap();

            if template_train.segments.is_empty() {
                template_train.start_station = TemplateTrainStation {
                    id: TemplateTrainStationId::new(manager.root.id_issuer.next()),
                    station_id: start_station.id,
                    track_id: manager
                        .root
                        .tracks
                        .values()
                        .find(|track| track.name == "1番線" && track.station_id == start_station.id)
                        .unwrap()
                        .id,
                    stop_time: crate::model::template_train::StopType::Stop(Time::new(0, 0, 0)),
                }
            }
            let segment_id = manager.root.id_issuer.next();
            let station_id = manager.root.id_issuer.next();
            let seg_ref = manager
                .root
                .find_segment_by_name(&start_station_name, &end_station_name)
                .unwrap();
            let segment = seg_ref.segment_id;
            template_train.segments.push((
                TemplateTrainSegment {
                    id: TemplateTrainSegmentId::new(segment_id),
                    segment_id: segment,
                    running_time: Time::new_from_total_second(running_time),
                    is_reversed: seg_ref.is_reversed,
                },
                TemplateTrainStation {
                    id: TemplateTrainStationId::new(station_id),
                    station_id: end_station.id,
                    track_id: manager
                        .root
                        .tracks
                        .values()
                        .find(|track| track.name == "1番線" && track.station_id == end_station.id)
                        .unwrap()
                        .id,
                    stop_time: if is_passing {
                        StopType::Pass
                    } else {
                        StopType::Stop(Time::new(0, 0, stop_time))
                    },
                },
            ));
        }

        // 暫定処置始まり
        manager
            .root
            .template_trains
            .insert(template_train.id, template_train);
        // 暫定処置終わり
    }
}

fn add_train_types(manager: &mut CommandManager) {
    let train_types = include_str!("./traintype.csv");
    for train_type in train_types.split("\n") {
        let id = manager.root.id_issuer.next();
        manager.execute(Box::new(AddTrainTypeCommand::new(TrainType::new(
            TrainTypeId::new(id),
            train_type,
        ))));
    }
}

fn add_test_train(manager: &mut CommandManager) {
    let mut timetable = Timetable::new(TimetableId::new(manager.root.id_issuer.next()), "1号表");

    // 12:00発湊川発姫路行特急
    let mut train_ltd1 = Train::new(TrainId::new(manager.root.id_issuer.next()), timetable.id);
    train_ltd1.start_departure_time = Time::new(12, 0, 0);
    train_ltd1.template_segments.push(TemplateSegment {
        template_train_id: manager
            .root
            .find_template_train_by_name("神姫線下り-特急")
            .unwrap()
            .id,
        start_station_id: manager.root.find_station_by_name("湊川").unwrap().id,
        end_station_id: manager.root.find_station_by_name("姫路").unwrap().id,
    });
    manager
        .root
        .trains
        .insert(train_ltd1.id, train_ltd1.clone());

    // 12:10発湊川発姫路行特急
    let mut train_ltd2 = Train::new(TrainId::new(manager.root.id_issuer.next()), timetable.id);
    train_ltd2.start_departure_time = Time::new(12, 10, 0);
    train_ltd2.template_segments.push(TemplateSegment {
        template_train_id: manager
            .root
            .find_template_train_by_name("神姫線下り-特急")
            .unwrap()
            .id,
        start_station_id: manager.root.find_station_by_name("湊川").unwrap().id,
        end_station_id: manager.root.find_station_by_name("姫路").unwrap().id,
    });
    manager
        .root
        .trains
        .insert(train_ltd2.id, train_ltd2.clone());

    // 12:20発湊川発姫路行急行
    let mut train_ltd3 = Train::new(TrainId::new(manager.root.id_issuer.next()), timetable.id);
    train_ltd3.start_departure_time = Time::new(12, 20, 0);
    train_ltd3.template_segments.push(TemplateSegment {
        template_train_id: manager
            .root
            .find_template_train_by_name("神姫線下り-急行")
            .unwrap()
            .id,
        start_station_id: manager.root.find_station_by_name("湊川").unwrap().id,
        end_station_id: manager.root.find_station_by_name("姫路").unwrap().id,
    });
    manager
        .root
        .trains
        .insert(train_ltd3.id, train_ltd3.clone());

    // 12:02発湊川発姫路行普通
    let mut train_local = Train::new(TrainId::new(manager.root.id_issuer.next()), timetable.id);
    train_local.start_departure_time = Time::new(12, 2, 0);
    train_local.template_segments.push(TemplateSegment {
        template_train_id: manager
            .root
            .find_template_train_by_name("神姫線下り-普通")
            .unwrap()
            .id,
        start_station_id: manager.root.find_station_by_name("湊川").unwrap().id,
        end_station_id: manager.root.find_station_by_name("姫路").unwrap().id,
    });
    manager
        .root
        .trains
        .insert(train_local.id, train_local.clone());

    // 11:30発姫路発湊川行普通
    let mut train_local_up = Train::new(TrainId::new(manager.root.id_issuer.next()), timetable.id);
    train_local_up.start_departure_time = Time::new(11, 30, 0);
    train_local_up.template_segments.push(TemplateSegment {
        template_train_id: manager
            .root
            .find_template_train_by_name("神姫線上り-普通")
            .unwrap()
            .id,
        start_station_id: manager.root.find_station_by_name("姫路").unwrap().id,
        end_station_id: manager.root.find_station_by_name("湊川").unwrap().id,
    });
    manager
        .root
        .trains
        .insert(train_local_up.id, train_local_up.clone());

    {
        let stations = vec![
            "湊川",
            "会下山",
            "長田",
            "板宿",
            "妙法寺",
            "昴麓園",
            "奥畑",
            "玉子温泉",
            "陣屋",
            "大沢",
            "多聞",
            "明舞",
            "大蔵谷",
            "明石",
            "西新町",
            "林崎",
            "藤江",
            "中八木",
            "江井ヶ島",
            "西江井ヶ島",
            "魚住",
            "東二見",
            "西二見",
            "阿閇",
            "別府",
            "浜の宮",
            "尾上の松",
            "高砂",
            "荒井",
            "伊保",
            "曽根",
            "大塩",
            "的形",
            "八家",
            "白浜の宮",
            "妻鹿",
            "飾磨",
            "亀山",
            "手柄",
            "姫路",
        ];
        for window in stations.windows(2) {
            let start = window[0];
            let end = window[1];

            let segment = manager.root.find_segment_by_name(start, end).unwrap();
            let segment_id = segment.segment_id;

            timetable.segment_train_orders.insert(
                segment_id,
                (
                    SegmentTrainOrder {
                        order: vec![],
                        segment_id,
                        is_reversed: false,
                    },
                    SegmentTrainOrder {
                        order: vec![],
                        segment_id,
                        is_reversed: true,
                    },
                ),
            );
        }
    }

    // 列車順序情報
    {
        // 湊川→明石
        // train_ltd1 → train_local → train_ltd2 → train_ltd3
        let stations = vec![
            "湊川",
            "会下山",
            "長田",
            "板宿",
            "妙法寺",
            "昴麓園",
            "奥畑",
            "玉子温泉",
            "陣屋",
            "大沢",
            "多聞",
            "明舞",
            "大蔵谷",
            "明石",
        ];
        for window in stations.windows(2) {
            let start = window[0];
            let end = window[1];

            let segment = manager.root.find_segment_by_name(start, end).unwrap();
            let segment_id = segment.segment_id;

            timetable
                .segment_train_orders
                .get_mut(&segment_id)
                .unwrap()
                .0
                .order = vec![train_ltd1.id, train_local.id, train_ltd2.id, train_ltd3.id];
        }
    }
    {
        // 明石→高砂
        // train_ltd1 → train_ltd2 → train_local → train_ltd3
        let stations = vec![
            "明石",
            "西新町",
            "林崎",
            "藤江",
            "中八木",
            "江井ヶ島",
            "西江井ヶ島",
            "魚住",
            "東二見",
            "西二見",
            "阿閇",
            "別府",
            "浜の宮",
            "尾上の松",
            "高砂",
        ];
        for window in stations.windows(2) {
            let start = window[0];
            let end = window[1];

            let segment = manager.root.find_segment_by_name(start, end).unwrap();
            let segment_id = segment.segment_id;

            timetable
                .segment_train_orders
                .get_mut(&segment_id)
                .unwrap()
                .0
                .order = vec![train_ltd1.id, train_ltd2.id, train_local.id, train_ltd3.id];
        }
    }
    {
        // 高砂→姫路
        // train_ltd1 → train_ltd2 → train_ltd3 → train_local
        let stations = vec![
            "高砂",
            "荒井",
            "伊保",
            "曽根",
            "大塩",
            "的形",
            "八家",
            "白浜の宮",
            "妻鹿",
            "飾磨",
            "亀山",
            "手柄",
            "姫路",
        ];
        for window in stations.windows(2) {
            let start = window[0];
            let end = window[1];

            let segment = manager.root.find_segment_by_name(start, end).unwrap();
            let segment_id = segment.segment_id;

            timetable
                .segment_train_orders
                .get_mut(&segment_id)
                .unwrap()
                .0
                .order = vec![train_ltd1.id, train_ltd2.id, train_ltd3.id, train_local.id];
        }
    }
    {
        // 湊川→明石
        // train_ltd1 → train_local → train_ltd2 → train_ltd3
        let mut stations = vec![
            "湊川",
            "会下山",
            "長田",
            "板宿",
            "妙法寺",
            "昴麓園",
            "奥畑",
            "玉子温泉",
            "陣屋",
            "大沢",
            "多聞",
            "明舞",
            "大蔵谷",
            "明石",
            "西新町",
            "林崎",
            "藤江",
            "中八木",
            "江井ヶ島",
            "西江井ヶ島",
            "魚住",
            "東二見",
            "西二見",
            "阿閇",
            "別府",
            "浜の宮",
            "尾上の松",
            "高砂",
            "荒井",
            "伊保",
            "曽根",
            "大塩",
            "的形",
            "八家",
            "白浜の宮",
            "妻鹿",
            "飾磨",
            "亀山",
            "手柄",
            "姫路",
        ];
        stations.reverse();

        for window in stations.windows(2) {
            let start = window[0];
            let end = window[1];

            let segment = manager.root.find_segment_by_name(start, end).unwrap();
            let segment_id = segment.segment_id;

            timetable
                .segment_train_orders
                .get_mut(&segment_id)
                .unwrap()
                .1
                .order = vec![train_local_up.id];
        }
    }

    {
        // 12:30から2分ごとに列車を追加
        let stations = vec![
            "湊川",
            "会下山",
            "長田",
            "板宿",
            "妙法寺",
            "昴麓園",
            "奥畑",
            "玉子温泉",
            "陣屋",
            "大沢",
            "多聞",
            "明舞",
            "大蔵谷",
            "明石",
            "西新町",
            "林崎",
            "藤江",
            "中八木",
            "江井ヶ島",
            "西江井ヶ島",
            "魚住",
            "東二見",
            "西二見",
            "阿閇",
            "別府",
            "浜の宮",
            "尾上の松",
            "高砂",
            "荒井",
            "伊保",
            "曽根",
            "大塩",
            "的形",
            "八家",
            "白浜の宮",
            "妻鹿",
            "飾磨",
            "亀山",
            "手柄",
            "姫路",
        ];

        for i in 0..1080 * 2 {
            let mut train_local =
                Train::new(TrainId::new(manager.root.id_issuer.next()), timetable.id);
            train_local.start_departure_time = Time::new(12, 30 + i * 2, 0);
            train_local.template_segments.push(TemplateSegment {
                template_train_id: manager
                    .root
                    .find_template_train_by_name("神姫線下り-普通")
                    .unwrap()
                    .id,
                start_station_id: manager.root.find_station_by_name("湊川").unwrap().id,
                end_station_id: manager.root.find_station_by_name("姫路").unwrap().id,
            });
            manager
                .root
                .trains
                .insert(train_local.id, train_local.clone());

            for window in stations.windows(2) {
                let start = window[0];
                let end = window[1];

                let segment = manager.root.find_segment_by_name(start, end).unwrap();
                let segment_id = segment.segment_id;

                let order = timetable.segment_train_orders.get_mut(&segment_id).unwrap();
                order.0.order.push(train_local.id);
            }
        }
    }
    {
        // 12:30から2分ごとに列車を追加
        let mut stations = vec![
            "湊川",
            "会下山",
            "長田",
            "板宿",
            "妙法寺",
            "昴麓園",
            "奥畑",
            "玉子温泉",
            "陣屋",
            "大沢",
            "多聞",
            "明舞",
            "大蔵谷",
            "明石",
            "西新町",
            "林崎",
            "藤江",
            "中八木",
            "江井ヶ島",
            "西江井ヶ島",
            "魚住",
            "東二見",
            "西二見",
            "阿閇",
            "別府",
            "浜の宮",
            "尾上の松",
            "高砂",
            "荒井",
            "伊保",
            "曽根",
            "大塩",
            "的形",
            "八家",
            "白浜の宮",
            "妻鹿",
            "飾磨",
            "亀山",
            "手柄",
            "姫路",
        ];
        stations.reverse();
        let stations = stations;

        for i in 0..1080 * 2 {
            let mut train_local =
                Train::new(TrainId::new(manager.root.id_issuer.next()), timetable.id);
            train_local.start_departure_time = Time::new(12, 30 + i * 2, 0);
            train_local.template_segments.push(TemplateSegment {
                template_train_id: manager
                    .root
                    .find_template_train_by_name("神姫線上り-普通")
                    .unwrap()
                    .id,
                start_station_id: manager.root.find_station_by_name("姫路").unwrap().id,
                end_station_id: manager.root.find_station_by_name("湊川").unwrap().id,
            });
            manager
                .root
                .trains
                .insert(train_local.id, train_local.clone());

            for window in stations.windows(2) {
                let start = window[0];
                let end = window[1];

                let segment = manager.root.find_segment_by_name(start, end).unwrap();
                let segment_id = segment.segment_id;

                let order = timetable.segment_train_orders.get_mut(&segment_id).unwrap();
                order.1.order.push(train_local.id);
            }
        }
    }

    manager.root.timetables.insert(timetable.id, timetable);
}

fn add_diagram_view_setting(manager: &mut CommandManager) {
    let stations1 = vec![
        "湊川",
        "会下山",
        "長田",
        "板宿",
        "妙法寺",
        "昴麓園",
        "奥畑",
        "玉子温泉",
        "陣屋",
        "大沢",
        "多聞",
        "明舞",
        "大蔵谷",
        "明石",
    ];
    let stations2: Vec<_> = vec![
        "明石",
        "西新町",
        "林崎",
        "藤江",
        "中八木",
        "江井ヶ島",
        "西江井ヶ島",
        "魚住",
        "東二見",
        "西二見",
        "阿閇",
        "別府",
        "浜の宮",
        "尾上の松",
        "高砂",
        "荒井",
        "伊保",
        "曽根",
        "大塩",
        "的形",
        "八家",
        "白浜の宮",
        "妻鹿",
        "飾磨",
        "亀山",
        "手柄",
        "姫路",
    ]
    .into_iter()
    .rev()
    .collect();

    let id = manager.root.id_issuer.next();
    let mut segments = vec![];

    for window in stations1.windows(2) {
        let start = window[0];
        let end = window[1];

        let segment = manager.root.find_segment_by_name(start, end).unwrap();
        let segment_id = segment.segment_id;

        segments.push(DiagramViewSegment::StationBetween {
            segment: SegmentRef {
                segment_id,
                is_reversed: false,
            },
        })
    }

    segments.push(DiagramViewSegment::Black { scale: 1.0 });

    for window in stations2.windows(2) {
        let start = window[0];
        let end = window[1];

        let segment = manager.root.find_segment_by_name(start, end).unwrap();
        let segment_id = segment.segment_id;

        segments.push(DiagramViewSegment::StationBetween {
            segment: SegmentRef {
                segment_id,
                is_reversed: true,
            },
        })
    }

    let settings = DiagramViewSettings {
        id: DiagramViewSettingsId::new(id),
        name: "てすと".to_string(),
        segments,
    };

    manager
        .root
        .diagram_view_settings
        .insert(settings.id, settings);
}

pub fn get_test_data() -> CommandManager {
    let mut manager = CommandManager::new(Box::new(EmptyEventEmitter));
    add_stations(&mut manager);
    add_lines(&mut manager);
    add_train_types(&mut manager);
    add_template_trains(&mut manager, include_str!("./shinki_down.tsv"));
    add_template_trains(&mut manager, include_str!("./shinki_up.tsv"));
    add_template_trains(&mut manager, include_str!("./hanshin_down.tsv"));
    add_template_trains(&mut manager, include_str!("./hanshin_up.tsv"));
    add_test_train(&mut manager);
    add_diagram_view_setting(&mut manager);
    manager
}

#[test]
fn test() {
    let start = std::time::Instant::now();
    let manager = get_test_data();
    let duration = start.elapsed();
    println!("モデル初期化時間: {}ms", duration.as_millis());

    let mut file = std::fs::File::create("./src/test_data/test.ron").unwrap();
    let serialized = ron::to_string(&manager.root).unwrap();
    let bytes = serialized.as_bytes();
    let bytes = zstd::encode_all(bytes, 0).unwrap();
    std::io::Write::write(&mut file, bytes.as_slice()).unwrap();
}
