use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

use rquickjs::{prelude::*, Context, Runtime};
use serde::{Deserialize, Serialize};
use weaverail_model::model::{
    station::{Station, StationId},
    track::{Track, TrackId},
    DiagramRoot,
};
use weaverail_operation::{
    app::AppState,
    command::{script::ScriptCommand, CommandError},
};

#[derive(Clone, Debug, Serialize, specta::Type)]
pub struct ScriptExecutionResult {
    pub success: bool,
    pub output: String,
    pub changed_objects: usize,
    pub project_version: u32,
    pub elapsed_ms: u64,
    pub error: Option<ScriptError>,
}

#[derive(Clone, Debug, Serialize, specta::Type)]
pub struct ScriptError {
    pub kind: String,
    pub message: String,
}

pub struct ScriptSession {
    runtime: Runtime,
    context: Context,
}

impl ScriptSession {
    pub fn new() -> Result<Self, String> {
        let runtime = Runtime::new().map_err(|error| error.to_string())?;
        runtime.set_max_stack_size(256 * 1024);
        runtime.set_memory_limit(16 * 1024 * 1024);
        let context = Context::full(&runtime).map_err(|error| error.to_string())?;
        Ok(Self { runtime, context })
    }

    pub fn reset(&mut self) -> Result<(), String> {
        let runtime = Runtime::new().map_err(|error| error.to_string())?;
        runtime.set_max_stack_size(256 * 1024);
        runtime.set_memory_limit(16 * 1024 * 1024);
        let context = Context::full(&runtime).map_err(|error| error.to_string())?;
        self.runtime = runtime;
        self.context = context;
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StationCreateInput {
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StationUpdateInput {
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrackCreateInput {
    station_id: String,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrackUpdateInput {
    name: String,
}

fn js_error(message: impl Into<String>) -> rquickjs::Error {
    rquickjs::Error::new_from_js_message("Object", "String", message.into())
}

fn parse_station_id(value: &str) -> Result<StationId, rquickjs::Error> {
    let json = serde_json::to_string(value).unwrap();
    serde_json::from_str(&json)
        .map_err(|error| js_error(format!("Invalid station ID: {error}")))
}

fn parse_track_id(value: &str) -> Result<TrackId, rquickjs::Error> {
    let json = serde_json::to_string(value).unwrap();
    serde_json::from_str(&json)
        .map_err(|error| js_error(format!("Invalid track ID: {error}")))
}

fn execute_quickjs(
    source: &str,
    initial_root: &DiagramRoot,
    timeout: Duration,
    session: &mut ScriptSession,
) -> Result<(DiagramRoot, usize, String), String> {
    let forbidden = [
        "fetch(", "XMLHttpRequest", "WebSocket", "eval(", "Function(", "import(", "require(",
        "process.", "window.", "globalThis.", "tauri.", "__TAURI__", "import ",
    ];
    if forbidden.iter().any(|needle| source.contains(needle)) {
        return Err("Forbidden API detected in script".to_string());
    }

    let started = Instant::now();
    session.runtime.set_interrupt_handler(Some(Box::new(move || started.elapsed() >= timeout)));

    let working_root = Arc::new(Mutex::new(initial_root.clone()));
    let operation_count = Arc::new(AtomicUsize::new(0));

    session.context.with(|ctx| -> Result<(), String> {
        let globals = ctx.globals();

        {
            let root = Arc::clone(&working_root);
            globals.set(
                "_weaverailStationsList",
                Func::from(MutFn::from(move || -> rquickjs::Result<String> {
                    let root = root.lock().map_err(|_| js_error("Model lock failed"))?;
                    let stations: Vec<_> = root.stations.values().cloned().collect();
                    serde_json::to_string(&stations)
                        .map_err(|error| js_error(format!("Failed to serialize stations: {error}")))
                })),
            ).map_err(|error| error.to_string())?;
        }

        {
            let root = Arc::clone(&working_root);
            let operations = Arc::clone(&operation_count);
            globals.set(
                "_weaverailStationsCreate",
                Func::from(MutFn::from(move |_ctx: Ctx, input: String| -> rquickjs::Result<String> {
                    if operations.fetch_add(1, Ordering::SeqCst) >= 256 {
                        return Err(js_error("Script operation limit exceeded"));
                    }
                    let input: StationCreateInput = serde_json::from_str(&input)
                        .map_err(|error| js_error(format!("Invalid station input: {error}")))?;
                    let name = input.name.trim();
                    if name.is_empty() {
                        return Err(js_error("Station name must not be empty"));
                    }
                    let mut root = root.lock().map_err(|_| js_error("Model lock failed"))?;
                    let id = root.id_issuer.next();
                    let station = Station::new(StationId::new(id), name);
                    root.add_station(station.clone())
                        .map_err(|error| js_error(format!("Failed to add station: {error}")))?;
                    serde_json::to_string(&station)
                        .map_err(|error| js_error(format!("Failed to serialize station: {error}")))
                })),
            ).map_err(|error| error.to_string())?;
        }

        {
            let root = Arc::clone(&working_root);
            let operations = Arc::clone(&operation_count);
            globals.set(
                "_weaverailStationsUpdate",
                Func::from(MutFn::from(move |_ctx: Ctx, id: String, input: String| -> rquickjs::Result<String> {
                    if operations.fetch_add(1, Ordering::SeqCst) >= 256 {
                        return Err(js_error("Script operation limit exceeded"));
                    }
                    let input: StationUpdateInput = serde_json::from_str(&input)
                        .map_err(|error| js_error(format!("Invalid station input: {error}")))?;
                    let name = input.name.trim();
                    if name.is_empty() {
                        return Err(js_error("Station name must not be empty"));
                    }
                    let station_id = parse_station_id(&id)?;
                    let mut root = root.lock().map_err(|_| js_error("Model lock failed"))?;
                    let station = root.stations.get_mut(&station_id)
                        .ok_or_else(|| js_error("Station not found"))?;
                    station.name = name.to_string();
                    serde_json::to_string(station)
                        .map_err(|error| js_error(format!("Failed to serialize station: {error}")))
                })),
            ).map_err(|error| error.to_string())?;
        }

        {
            let root = Arc::clone(&working_root);
            let operations = Arc::clone(&operation_count);
            globals.set(
                "_weaverailStationsRemove",
                Func::from(MutFn::from(move |_ctx: Ctx, id: String| -> rquickjs::Result<()> {
                    if operations.fetch_add(1, Ordering::SeqCst) >= 256 {
                        return Err(js_error("Script operation limit exceeded"));
                    }
                    let station_id = parse_station_id(&id)?;
                    let mut root = root.lock().map_err(|_| js_error("Model lock failed"))?;
                    root.delete_station(station_id)
                        .map_err(|error| js_error(format!("Failed to remove station: {error}")))?;
                    Ok(())
                })),
            ).map_err(|error| error.to_string())?;
        }

        {
            let root = Arc::clone(&working_root);
            globals.set(
                "_weaverailTracksList",
                Func::from(MutFn::from(move |station_id: Opt<String>| -> rquickjs::Result<String> {
                    let root = root.lock().map_err(|_| js_error("Model lock failed"))?;
                    let tracks: Vec<_> = match station_id.0 {
                        Some(station_id) => {
                            let station_id = parse_station_id(&station_id)?;
                            root.tracks.values()
                                .filter(|track| track.station_id == station_id)
                                .cloned()
                                .collect()
                        }
                        None => root.tracks.values().cloned().collect(),
                    };
                    serde_json::to_string(&tracks)
                        .map_err(|error| js_error(format!("Failed to serialize tracks: {error}")))
                })),
            ).map_err(|error| error.to_string())?;
        }

        {
            let root = Arc::clone(&working_root);
            let operations = Arc::clone(&operation_count);
            globals.set(
                "_weaverailTracksCreate",
                Func::from(MutFn::from(move |_ctx: Ctx, input: String| -> rquickjs::Result<String> {
                    if operations.fetch_add(1, Ordering::SeqCst) >= 256 {
                        return Err(js_error("Script operation limit exceeded"));
                    }
                    let input: TrackCreateInput = serde_json::from_str(&input)
                        .map_err(|error| js_error(format!("Invalid track input: {error}")))?;
                    let name = input.name.trim();
                    if name.is_empty() {
                        return Err(js_error("Track name must not be empty"));
                    }
                    let station_id = parse_station_id(&input.station_id)?;
                    let mut root = root.lock().map_err(|_| js_error("Model lock failed"))?;
                    if !root.stations.contains_key(&station_id) {
                        return Err(js_error("Station not found"));
                    }
                    let id = root.id_issuer.next();
                    let track = Track::new(TrackId::new(id), station_id, name);
                    root.add_track(track.clone())
                        .map_err(|error| js_error(format!("Failed to add track: {error}")))?;
                    serde_json::to_string(&track)
                        .map_err(|error| js_error(format!("Failed to serialize track: {error}")))
                })),
            ).map_err(|error| error.to_string())?;
        }

        {
            let root = Arc::clone(&working_root);
            let operations = Arc::clone(&operation_count);
            globals.set(
                "_weaverailTracksUpdate",
                Func::from(MutFn::from(move |_ctx: Ctx, id: String, input: String| -> rquickjs::Result<String> {
                    if operations.fetch_add(1, Ordering::SeqCst) >= 256 {
                        return Err(js_error("Script operation limit exceeded"));
                    }
                    let input: TrackUpdateInput = serde_json::from_str(&input)
                        .map_err(|error| js_error(format!("Invalid track input: {error}")))?;
                    let name = input.name.trim();
                    if name.is_empty() {
                        return Err(js_error("Track name must not be empty"));
                    }
                    let track_id = parse_track_id(&id)?;
                    let mut root = root.lock().map_err(|_| js_error("Model lock failed"))?;
                    let track = root.tracks.get_mut(&track_id)
                        .ok_or_else(|| js_error("Track not found"))?;
                    track.name = name.to_string();
                    serde_json::to_string(track)
                        .map_err(|error| js_error(format!("Failed to serialize track: {error}")))
                })),
            ).map_err(|error| error.to_string())?;
        }

        {
            let root = Arc::clone(&working_root);
            let operations = Arc::clone(&operation_count);
            globals.set(
                "_weaverailTracksRemove",
                Func::from(MutFn::from(move |_ctx: Ctx, id: String| -> rquickjs::Result<()> {
                    if operations.fetch_add(1, Ordering::SeqCst) >= 256 {
                        return Err(js_error("Script operation limit exceeded"));
                    }
                    let track_id = parse_track_id(&id)?;
                    let mut root = root.lock().map_err(|_| js_error("Model lock failed"))?;
                    root.delete_track(track_id)
                        .map_err(|error| js_error(format!("Failed to remove track: {error}")))?;
                    Ok(())
                })),
            ).map_err(|error| error.to_string())?;
        }

        let api = r#"
if (typeof _scriptOutput === "undefined") {
  var _scriptOutput = [];
}
_scriptOutput.length = 0;
var console = {
  log(...args) {
    _scriptOutput.push(args.map(value => {
      if (typeof value === "string") return value;
      try { return JSON.stringify(value); } catch (_) { return String(value); }
    }).join(" "));
  }
};
var weaverail = {
  project: {
    stations: {
      list: () => JSON.parse(_weaverailStationsList()),
      get: id => JSON.parse(_weaverailStationsList()).find(station => station.id === id),
      create: input => JSON.parse(_weaverailStationsCreate(JSON.stringify(input))),
      update: (id, input) => JSON.parse(_weaverailStationsUpdate(id, JSON.stringify(input))),
      remove: id => _weaverailStationsRemove(id)
    },
    tracks: {
      list: stationId => JSON.parse(_weaverailTracksList(stationId)),
      get: id => JSON.parse(_weaverailTracksList()).find(track => track.id === id),
      create: input => JSON.parse(_weaverailTracksCreate(JSON.stringify(input))),
      update: (id, input) => JSON.parse(_weaverailTracksUpdate(id, JSON.stringify(input))),
      remove: id => _weaverailTracksRemove(id)
    }
  }
};
"#;

        let wrapped = format!("{api}\n{source}\n;_scriptOutput.join('\\n')");
        let output: String = match ctx.eval(wrapped) {
            Ok(output) => output,
            Err(error) => {
                let caught = ctx.catch();
                let detail = caught
                    .as_object()
                    .and_then(|object| rquickjs::Exception::from_object(object.clone()))
                    .and_then(|exception| exception.message())
                    .unwrap_or_else(|| error.to_string());
                let stack = caught
                    .as_object()
                    .and_then(|object| rquickjs::Exception::from_object(object.clone()))
                    .and_then(|exception| exception.stack());
                let message = match stack {
                    Some(stack) => format!("{detail}\n{stack}"),
                    None => detail,
                };
                return Err(message);
            }
        };
        ctx.globals().set("__script_output__", output).map_err(|error| error.to_string())?;
        Ok(())
    })?;

    let final_root = working_root
        .lock()
        .map_err(|_| "Model lock failed".to_string())?
        .clone();
    let final_root = final_root;
    let change_count = operation_count.load(Ordering::SeqCst);
    let output = session.context.with(|ctx| {
        ctx.globals().get("__script_output__").unwrap_or_default()
    });
    let mut final_root = final_root;
    final_root.validate().map_err(|error| format!("Transaction validation failed: {error}"))?;
    final_root.version = initial_root.version;
    Ok((final_root, change_count, output))
}

#[tauri::command]
#[specta::specta]
pub async fn run_script(
    state: tauri::State<'_, Mutex<AppState>>,
    session_state: tauri::State<'_, Mutex<ScriptSession>>,
    source: String,
) -> Result<ScriptExecutionResult, CommandError> {
    if source.len() > 256 * 1024 {
        let state = state.lock().map_err(|_| CommandError::MutexLockError)?;
        return Ok(ScriptExecutionResult {
            success: false,
            output: String::new(),
            changed_objects: 0,
            project_version: state.command_manager.root.version,
            elapsed_ms: 0,
            error: Some(ScriptError {
                kind: "source-too-large".to_string(),
                message: "Script exceeds 256 KB".to_string(),
            }),
        });
    }

    let started = Instant::now();
    let mut state = state.lock().map_err(|_| CommandError::MutexLockError)?;
    let mut session = session_state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?;
    let old_root = state.command_manager.root.clone();

    match execute_quickjs(&source, &old_root, Duration::from_secs(30), &mut session) {
        Ok((new_root, changed_objects, output)) => {
            state
                .command_manager
                .execute(Box::new(ScriptCommand::new(old_root, new_root)));
            Ok(ScriptExecutionResult {
                success: true,
                output,
                changed_objects,
                project_version: state.command_manager.root.version,
                elapsed_ms: started.elapsed().as_millis().min(u64::MAX as u128) as u64,
                error: None,
            })
        }
        Err(error) => Ok(ScriptExecutionResult {
            success: false,
            output: String::new(),
            changed_objects: 0,
            project_version: old_root.version,
            elapsed_ms: started.elapsed().as_millis().min(u64::MAX as u128) as u64,
            error: Some(ScriptError {
                kind: "execution-error".to_string(),
                message: error,
            }),
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn reset_script_session(
    session_state: tauri::State<'_, Mutex<ScriptSession>>,
) -> Result<(), CommandError> {
    session_state
        .lock()
        .map_err(|_| CommandError::MutexLockError)?
        .reset()
        .map_err(|_| CommandError::Inconsistent)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn station_create_commits_in_working_root() {
        let initial = DiagramRoot::default();
        let source = r#"
const station = weaverail.project.stations.create({ name: "Tokyo" });
console.log(station.name);
"#;

        let mut session = ScriptSession::new().unwrap();
        let (root, changed, output) =
            execute_quickjs(source, &initial, Duration::from_secs(1), &mut session).unwrap();
        assert_eq!(root.stations.len(), 1);
        assert_eq!(changed, 1);
        assert_eq!(root.stations.values().next().unwrap().name, "Tokyo");
        assert_eq!(output, "Tokyo");
    }

    #[test]
    fn rollback_is_represented_by_execution_error() {
        let initial = DiagramRoot::default();
        let source = r#"
weaverail.project.stations.create({ name: "First" });
throw new Error("abort");
"#;

        assert!(
            execute_quickjs(source, &initial, Duration::from_secs(1), &mut ScriptSession::new().unwrap()).is_err()
        );
    }

    #[test]
    fn forbidden_api_is_rejected_before_execution() {
        let initial = DiagramRoot::default();
        assert!(
            execute_quickjs("fetch('https://example.com')", &initial, Duration::from_secs(1), &mut ScriptSession::new().unwrap())
                .is_err()
        );
    }

    #[test]
    fn variables_persist_between_interactive_blocks() {
        let initial = DiagramRoot::default();
        let mut session = ScriptSession::new().unwrap();
        execute_quickjs("const answer = 42;", &initial, Duration::from_secs(1), &mut session)
            .unwrap();
        let (_, _, output) = execute_quickjs(
            "console.log(answer);",
            &initial,
            Duration::from_secs(1),
            &mut session,
        )
        .unwrap();
        assert_eq!(output, "42");
    }
}
