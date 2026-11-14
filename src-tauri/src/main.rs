// No console window behind the launcher in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hub_core::achievements::{self, Achievement};
use hub_core::announcements::{self, Announcement};
use hub_core::build;
use hub_core::config::{self, Config};
use hub_core::news::{self, Entry};
use hub_core::project::{self, Script, State};
use hub_core::registry::{Project, Registry};
use hub_core::requirements::{self, Check, Requires};
use hub_core::runner::{self, Cancel};
use hub_core::source;
use hub_core::system::{self, SystemInfo};
use hub_core::update::{self, Outcome};
use hub_core::video::{self, Video};
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_updater::UpdaterExt;

struct Hub {
    config_path: PathBuf,
    config: Mutex<Config>,
    /// Set when the settings file could not be read: shown, and nothing is
    /// saved over the file until the user has seen it.
    config_error: Mutex<Option<String>>,
    registry: Mutex<Registry>,
    remote_news: Mutex<HashMap<String, Vec<Entry>>>,
    launcher_news: Mutex<Vec<Entry>>,
    latest_video: Mutex<Option<Video>>,
    announcements: Mutex<Vec<Announcement>>,
    task: Mutex<Option<Running>>,
}

/// The launcher's license, shown on first run and in Credits.
const LICENSE: &str = include_str!("../../LICENSE");
/// Raised when the terms change, so the agreement is shown again.
const LICENSE_VERSION: u32 = 2;

struct Running {
    project: String,
    cancel: Cancel,
}

#[derive(Serialize)]
struct Snapshot {
    version: &'static str,
    install_root: Option<PathBuf>,
    /// The install folder of the settings, when it is not on the disk.
    missing_root: Option<PathBuf>,
    language: Option<String>,
    /// False until the user accepts the current license agreement.
    license_accepted: bool,
    config_error: Option<String>,
    /// The project a task is running for, if any.
    busy: Option<String>,
    /// The launcher's own announcements, newest first (the home page).
    launcher_news: Vec<Entry>,
    /// The author's newest video on YouTube, when it could be read.
    latest_video: Option<Video>,
    /// The author's announcements this user should see, newest first.
    announcements: Vec<AnnouncementView>,
    /// After the launcher updated itself: what it brought since the version
    /// the user had, until they have seen it. None when there is nothing new
    /// to show; empty while the news are not loaded or say nothing of it.
    launcher_update: Option<LauncherNews>,
    projects: Vec<ProjectView>,
}

#[derive(Serialize)]
struct LauncherNews {
    /// The version the user had, when the launcher recorded it.
    from: Option<String>,
    changes: Vec<news::Release>,
}

#[derive(Serialize)]
struct AnnouncementView {
    #[serde(flatten)]
    announcement: Announcement,
    /// Closed by the user already: it no longer makes the megaphone beat.
    seen: bool,
}

#[derive(Serialize)]
struct ProjectView {
    id: String,
    name: String,
    tagline: String,
    description: String,
    tags: Vec<String>,
    url: String,
    /// Pictures from the project's repository, as GitHub URLs.
    hero: Option<String>,
    logo: Option<String>,
    screens: Vec<String>,
    input_label: Option<String>,
    dir: Option<PathBuf>,
    input: Option<PathBuf>,
    state: State,
    can_update: bool,
    can_rebuild: bool,
    /// A git working copy: uninstalling only takes it off the launcher
    is_clone: bool,
    /// What the installed copy has, newest first.
    news: Vec<Entry>,
    /// Entries on GitHub that the installed copy does not have.
    new_entries: Vec<Entry>,
    /// The news as GitHub has them now (the home page); what the installed
    /// copy has when GitHub was not reached.
    latest_news: Vec<Entry>,
    /// What the installed copy has written of its achievements.
    achievements: Vec<Achievement>,
    /// Which version is installed.
    version: Option<project::Version>,
}

#[derive(Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum TaskEvent {
    Line { project: String, text: String },
    Done { project: String, ok: bool, message: String },
}

fn snapshot_of(hub: &Hub) -> Snapshot {
    let mut config = hub.config.lock().unwrap().clone();
    // An install folder that is no longer on the disk (deleted, or on a drive
    // that is not plugged in) is shown as not chosen, and named, so the user
    // picks one again instead of finding every project "not installed"
    let missing_root = config.install_root.take_if(|root| !root.is_dir());
    let registry = hub.registry.lock().unwrap().clone();
    let remote = hub.remote_news.lock().unwrap();
    let projects: Vec<ProjectView> = registry
        .projects
        .iter()
        .map(|p| {
            let dir = config.project_dir(&p.id);
            let input = config.input(&p.id);
            let state = project::state(p, dir.as_deref(), input.as_deref());
            let installed = state != State::NotInstalled;
            let local = match (&dir, installed) {
                (Some(d), true) => project::local_news(p, d),
                _ => Vec::new(),
            };
            let new_entries = match (installed, remote.get(&p.id)) {
                (true, Some(r)) => news::newer_than(r, &local),
                _ => Vec::new(),
            };
            let version = match (&dir, installed) {
                (Some(d), true) => project::installed_version(p, d, &local),
                _ => None,
            };
            ProjectView {
                id: p.id.clone(),
                name: p.name.clone(),
                tagline: p.tagline.clone(),
                description: p.description.clone(),
                tags: p.tags.clone(),
                url: p.url(),
                hero: p.art.hero.as_deref().map(|f| p.raw_url(f)),
                logo: p.art.logo.as_deref().map(|f| p.raw_url(f)),
                screens: p.art.screens.iter().map(|f| p.raw_url(f)).collect(),
                input_label: p.input.as_ref().map(|i| i.label.clone()),
                // Before it is installed, show the news from GitHub instead
                latest_news: remote.get(&p.id).cloned().unwrap_or_else(|| local.clone()),
                news: if installed { local } else { remote.get(&p.id).cloned().unwrap_or_default() },
                new_entries,
                version,
                achievements: match (&dir, installed) {
                    (Some(d), true) => achievements::read(d),
                    _ => Vec::new(),
                },
                can_update: installed && p.scripts.update.is_some(),
                can_rebuild: state == State::Ready && p.scripts.build.is_some(),
                is_clone: dir.as_ref().is_some_and(|d| d.join(".git").exists()),
                dir,
                input,
                state,
            }
        })
        .collect();
    let today = announcements::today();
    let installed = |id: &str| projects.iter().any(|p| p.id == id && p.state != State::NotInstalled);
    let announcements = hub
        .announcements
        .lock()
        .unwrap()
        .iter()
        .filter(|a| announcements::shown(a, &today, installed))
        .map(|a| AnnouncementView { seen: config.seen_announcements.contains(&a.id), announcement: a.clone() })
        .collect();
    Snapshot {
        version: env!("CARGO_PKG_VERSION"),
        install_root: config.install_root.clone(),
        missing_root,
        language: config.language.clone(),
        license_accepted: config.license_accepted.is_some_and(|v| v >= LICENSE_VERSION),
        config_error: hub.config_error.lock().unwrap().clone(),
        busy: hub.task.lock().unwrap().as_ref().map(|t| t.project.clone()),
        launcher_news: hub.launcher_news.lock().unwrap().clone(),
        latest_video: hub.latest_video.lock().unwrap().clone(),
        announcements,
        launcher_update: launcher_update(&config, &hub.launcher_news.lock().unwrap()),
        projects,
    }
}

/// What a launcher update brought, while the user has not seen it. A user
/// with no version recorded who accepted the license came from a launcher
/// older than the record: they see what this version brought.
fn launcher_update(config: &Config, news: &[Entry]) -> Option<LauncherNews> {
    let current = env!("CARGO_PKG_VERSION");
    match config.last_version.as_deref() {
        Some(v) if v == current => None,
        None if config.license_accepted.is_none() => None,
        from => Some(LauncherNews { from: from.map(str::to_string), changes: news::launcher_changes(news, from, current) }),
    }
}

fn change_config(hub: &Hub, change: impl FnOnce(&mut Config)) -> Result<(), String> {
    if let Some(e) = hub.config_error.lock().unwrap().as_ref() {
        return Err(format!("{e} Fix or delete {} and start the launcher again.", hub.config_path.display()));
    }
    let mut config = hub.config.lock().unwrap();
    let mut next = config.clone();
    change(&mut next);
    next.save(&hub.config_path)?;
    *config = next;
    Ok(())
}

fn find_project(hub: &Hub, id: &str) -> Result<Project, String> {
    hub.registry.lock().unwrap().find(id).cloned().ok_or_else(|| format!("Unknown project: {id}"))
}

#[tauri::command]
fn snapshot(hub: tauri::State<Hub>) -> Snapshot {
    snapshot_of(&hub)
}

/// Reads the project list and each project's news from GitHub. Slow (network),
/// so it runs off the main thread; what cannot be reached keeps its old value.
#[tauri::command(async)]
fn refresh(hub: tauri::State<Hub>) -> Snapshot {
    if let Ok(reg) = Registry::fetch() {
        *hub.registry.lock().unwrap() = reg;
    }
    let projects = hub.registry.lock().unwrap().projects.clone();
    for p in projects {
        if let Ok(entries) = news::fetch(&p) {
            hub.remote_news.lock().unwrap().insert(p.id.clone(), entries);
        }
    }
    if let Ok(entries) = news::fetch_launcher() {
        *hub.launcher_news.lock().unwrap() = entries;
    }
    if let Ok(Some(v)) = video::fetch_latest() {
        *hub.latest_video.lock().unwrap() = Some(v);
    }
    if let Ok(list) = announcements::fetch() {
        *hub.announcements.lock().unwrap() = list;
    }
    snapshot_of(&hub)
}

/// Looks on this PC for what the listed projects need to be built. It starts
/// a few programs to ask their version, so it runs off the main thread.
#[tauri::command(async)]
fn check_requirements(hub: tauri::State<Hub>) -> Vec<Check> {
    let projects = hub.registry.lock().unwrap().projects.clone();
    let all = projects.iter().fold(Requires::default(), |all, p| all.merge(&p.requires));
    requirements::check(&all)
}

#[derive(Serialize)]
struct LauncherUpdate {
    version: String,
    notes: Option<String>,
}

/// Asks the launcher's releases on GitHub whether there is a newer version.
/// Only the launcher itself: projects are never downloaded built.
#[tauri::command]
async fn check_launcher_update(app: AppHandle) -> Result<Option<LauncherUpdate>, String> {
    let update = app.updater().map_err(|e| e.to_string())?.check().await.map_err(|e| e.to_string())?;
    Ok(update.map(|u| LauncherUpdate { version: u.version.clone(), notes: u.body.clone() }))
}

/// Downloads the new version, checks its signature against the key built into
/// this exe, installs it and starts it. Refused while a task is running.
#[tauri::command]
async fn install_launcher_update(app: AppHandle) -> Result<(), String> {
    if app.state::<Hub>().task.lock().unwrap().is_some() {
        return Err("A task is still running: wait for it to end first.".into());
    }
    let update = app.updater().map_err(|e| e.to_string())?.check().await.map_err(|e| e.to_string())?;
    let update = update.ok_or("This is already the latest version.")?;
    update.download_and_install(|_, _| {}, || {}).await.map_err(|e| e.to_string())?;
    app.restart()
}

#[tauri::command]
fn set_install_root(hub: tauri::State<Hub>, path: PathBuf) -> Result<Snapshot, String> {
    if !path.is_absolute() {
        return Err("Pick a full folder path.".into());
    }
    if let Some(problem) = config::folder_problem(&path) {
        return Err(problem);
    }
    change_config(&hub, |c| c.install_root = Some(path))?;
    Ok(snapshot_of(&hub))
}

#[tauri::command]
fn set_language(hub: tauri::State<Hub>, language: String) -> Result<Snapshot, String> {
    change_config(&hub, |c| c.language = Some(language))?;
    Ok(snapshot_of(&hub))
}

/// The user closed these announcements: they stop making the megaphone beat
/// until there is one with a new id.
#[tauri::command]
fn mark_announcements_seen(hub: tauri::State<Hub>, ids: Vec<String>) -> Result<Snapshot, String> {
    change_config(&hub, |c| {
        for id in ids {
            if !c.seen_announcements.contains(&id) {
                c.seen_announcements.push(id);
            }
        }
    })?;
    Ok(snapshot_of(&hub))
}

/// The news of this launcher version have been seen (or there were none).
#[tauri::command]
fn mark_launcher_seen(hub: tauri::State<Hub>) -> Result<Snapshot, String> {
    change_config(&hub, |c| c.last_version = Some(env!("CARGO_PKG_VERSION").to_string()))?;
    Ok(snapshot_of(&hub))
}

#[tauri::command]
fn license_text() -> &'static str {
    LICENSE
}

#[tauri::command]
fn accept_license(hub: tauri::State<Hub>) -> Result<Snapshot, String> {
    change_config(&hub, |c| {
        // A new user has nothing to catch up on
        if c.license_accepted.is_none() {
            c.last_version = Some(env!("CARGO_PKG_VERSION").to_string());
        }
        c.license_accepted = Some(LICENSE_VERSION);
    })?;
    Ok(snapshot_of(&hub))
}

/// Closes the launcher: the way out of the agreement for who does not accept it.
#[tauri::command]
fn quit(app: AppHandle) {
    app.exit(0);
}

/// The user's own file for a project: only its path is kept. Nothing is copied,
/// uploaded or downloaded.
#[tauri::command]
fn set_input(hub: tauri::State<Hub>, id: String, path: PathBuf) -> Result<Snapshot, String> {
    find_project(&hub, &id)?;
    if !path.is_file() {
        return Err(format!("{} is not a file.", path.display()));
    }
    change_config(&hub, |c| c.projects.entry(id).or_default().input = Some(path))?;
    Ok(snapshot_of(&hub))
}

/// Points a project at a folder the user already has it in.
#[tauri::command]
fn set_project_dir(hub: tauri::State<Hub>, id: String, path: PathBuf) -> Result<Snapshot, String> {
    let p = find_project(&hub, &id)?;
    if !project::is_installed(&p, &path) {
        return Err(format!("{} does not look like a folder of {}.", path.display(), p.name));
    }
    if let Some(problem) = config::folder_problem(&path) {
        return Err(problem);
    }
    change_config(&hub, |c| c.projects.entry(id).or_default().dir = Some(path))?;
    Ok(snapshot_of(&hub))
}

#[tauri::command]
fn launch(hub: tauri::State<Hub>, id: String) -> Result<(), String> {
    let p = find_project(&hub, &id)?;
    let dir = hub.config.lock().unwrap().project_dir(&id).ok_or("Not installed.")?;
    project::launch(&p, &dir)
}

/// A shortcut to the built game on the desktop.
#[tauri::command]
fn desktop_shortcut(hub: tauri::State<Hub>, id: String) -> Result<(), String> {
    let p = find_project(&hub, &id)?;
    let dir = hub.config.lock().unwrap().project_dir(&id).ok_or("Not installed.")?;
    project::desktop_shortcut(&p, &dir)
}

/// Windows version and graphics card, for the Feedback page's reports
#[tauri::command(async)]
fn system_info() -> SystemInfo {
    system::info()
}

#[tauri::command]
fn open_folder(hub: tauri::State<Hub>, id: String) -> Result<(), String> {
    let dir = hub.config.lock().unwrap().project_dir(&id).ok_or("Not installed.")?;
    if !dir.is_dir() {
        return Err("Not installed.".into());
    }
    let opener = if cfg!(windows) { "explorer" } else if cfg!(target_os = "macos") { "open" } else { "xdg-open" };
    std::process::Command::new(opener).arg(&dir).spawn().map(|_| ()).map_err(|e| e.to_string())
}

/// Deletes a project's folder (see `project::uninstall`) and forgets a folder
/// of the user's own that it pointed at. The path of the user's file is kept
/// for a later install, unless the file went with the folder.
#[tauri::command(async)]
fn uninstall(hub: tauri::State<Hub>, id: String) -> Result<Snapshot, String> {
    let p = find_project(&hub, &id)?;
    if hub.task.lock().unwrap().is_some() {
        return Err("A task is still running: wait for it to end first.".into());
    }
    let dir = hub.config.lock().unwrap().project_dir(&id).ok_or("Not installed.")?;
    if project::is_running(&p, &dir) {
        return Err(format!("{} is running: close it first.", p.name));
    }
    let result = project::uninstall(&p, &dir);
    // Also after a failure halfway: what is left is shown as it is
    if !matches!(result, Err(_) if dir.is_dir()) {
        change_config(&hub, |c| {
            if let Some(pc) = c.projects.get_mut(&id) {
                pc.dir = None;
                pc.input.take_if(|file| !file.is_file());
            }
        })?;
    }
    result?;
    Ok(snapshot_of(&hub))
}

#[tauri::command]
fn cancel_task(hub: tauri::State<Hub>) {
    if let Some(t) = hub.task.lock().unwrap().as_ref() {
        t.cancel.cancel();
    }
}

/// `setup`: download the source if it is not here, then run the project's
/// install script (requirements + build). `build`: its build script.
/// `update`: the latest source, then a rebuild. One task at a time; its output arrives as
/// `task` events.
#[tauri::command]
fn start_task(app: AppHandle, hub: tauri::State<Hub>, id: String, action: String) -> Result<(), String> {
    let p = find_project(&hub, &id)?;
    if !matches!(action.as_str(), "setup" | "build" | "update") {
        return Err(format!("Unknown action: {action}"));
    }
    let (dir, input) = {
        let config = hub.config.lock().unwrap();
        (config.project_dir(&id).ok_or("Choose where to install projects first (Settings).")?, config.input(&id))
    };
    // Chosen before the launcher checked it: nothing would build there
    if let Some(problem) = config::folder_problem(&dir) {
        return Err(problem);
    }
    if action != "setup" && project::is_running(&p, &dir) {
        return Err(format!("{} is running: close it first.", p.name));
    }
    let cancel = Cancel::new();
    {
        let mut task = hub.task.lock().unwrap();
        if task.is_some() {
            return Err("Another task is still running.".into());
        }
        *task = Some(Running { project: id.clone(), cancel: cancel.clone() });
    }
    std::thread::spawn(move || {
        let mut log = |text: &str| {
            let _ = app.emit("task", TaskEvent::Line { project: id.clone(), text: text.to_string() });
        };
        let result = run_task(&p, &dir, input.as_deref(), &action, &cancel, &mut log);
        let (ok, message) = match result {
            _ if cancel.was_asked() => (false, "Cancelled.".to_string()),
            Ok(()) => (true, String::new()),
            Err(e) => (false, e),
        };
        *app.state::<Hub>().task.lock().unwrap() = None;
        let _ = app.emit("task", TaskEvent::Done { project: id.clone(), ok, message });
    });
    Ok(())
}

fn run_task(
    p: &Project,
    dir: &std::path::Path,
    input: Option<&std::path::Path>,
    action: &str,
    cancel: &Cancel,
    log: &mut dyn FnMut(&str),
) -> Result<(), String> {
    let script = match action {
        "setup" => {
            if !project::is_installed(p, dir) {
                source::install(p, dir, log)?;
            }
            if cancel.was_asked() {
                return Ok(());
            }
            match project::state(p, Some(dir), input) {
                // Its own file comes next: the interface asks for it
                State::NeedsInput | State::Ready => return Ok(()),
                State::NotInstalled => return Err("The download did not bring the project's scripts.".into()),
                State::NeedsBuild => Script::Install,
            }
        }
        "build" => Script::Build,
        // A git clone is still updated by the project's own script (git pull)
        _ if p.update_state.is_none() || dir.join(".git").exists() => Script::Update,
        _ => {
            let outcome = update::update(p, dir, false, log)?;
            if cancel.was_asked() || (matches!(outcome, Outcome::UpToDate { .. }) && project::is_built(p, dir)) {
                return Ok(());
            }
            log("Rebuilding...");
            Script::Build
        }
    };
    // A project that brings a hub.toml is built by the launcher itself. When a
    // tool is missing, a first install still goes to the project's install
    // script, which installs the tools and then builds.
    if script != Script::Update {
        if let Some(manifest) = build::manifest_in(dir)? {
            // Its tools from the build kit first: downloaded once, into the
            // launcher's own folder
            let kit_root = hub_core::kit::default_root();
            build::install_kit(&manifest, &kit_root, cancel, log)?;
            match build::Tools::find(&manifest, input, &kit_root) {
                Ok(tools) => {
                    let warnings = build::run(&manifest, dir, &tools, cancel, log)?;
                    for w in &warnings {
                        log(&format!("Warning: {w}"));
                    }
                    log(&format!("Done: {}", manifest.exe));
                    return Ok(());
                }
                Err(e) if script == Script::Install && p.scripts.install.is_some() => {
                    log(&e);
                    log("Running the project's installer for them.");
                }
                Err(e) => return Err(e),
            }
        }
    }
    let cmd = project::script_command(p, dir, script, input)?;
    log(&format!("> {}", cmd.get_program().to_string_lossy()));
    let code = runner::run(cmd, cancel, log).map_err(|e| format!("Could not start the script: {e}"))?;
    if code != 0 {
        return Err(format!("The script ended with an error (code {code}). See the log above."));
    }
    Ok(())
}

fn main() {
    let config_path = Config::default_path();
    let (config, config_error) = match Config::load(&config_path) {
        Ok(c) => (c, None),
        Err(e) => (Config::default(), Some(e)),
    };
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(Hub {
            config_path,
            config: Mutex::new(config),
            config_error: Mutex::new(config_error),
            registry: Mutex::new(Registry::bundled()),
            remote_news: Mutex::new(HashMap::new()),
            launcher_news: Mutex::new(Vec::new()),
            latest_video: Mutex::new(None),
            announcements: Mutex::new(Vec::new()),
            task: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            snapshot,
            refresh,
            check_requirements,
            check_launcher_update,
            install_launcher_update,
            set_install_root,
            set_language,
            license_text,
            accept_license,
            mark_announcements_seen,
            mark_launcher_seen,
            quit,
            set_input,
            set_project_dir,
            launch,
            open_folder,
            system_info,
            desktop_shortcut,
            uninstall,
            cancel_task,
            start_task
        ])
        .run(tauri::generate_context!())
        .expect("could not start NicoLauncher");
}
