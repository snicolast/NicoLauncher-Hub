// No console window behind the launcher in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hub_core::achievements::{self, Achievement};
use hub_core::announcements::{self, Announcement};
use hub_core::art::{self, Art};
use hub_core::build;
use hub_core::config::{self, Config, Sort};
use hub_core::diagnose::{self, Diagnosis, Fix};
use hub_core::feedback;
use hub_core::news::{self, Entry};
use hub_core::portable::{self, Places};
use hub_core::project::{self, Script, State};
use hub_core::registry::{Project, Registry};
use hub_core::requirements::{self, Check, Requires};
use hub_core::runner::{self, Cancel};
use hub_core::source;
use hub_core::system::{self, SystemInfo};
use hub_core::update::{self, Outcome};
use hub_core::video::{self, Video};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::webview::NewWindowResponse;
use tauri::{AppHandle, Emitter, Manager, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_updater::UpdaterExt;

struct Hub {
    /// Where the settings and the build kit are: AppData, or the launcher's
    /// own folder when it runs portable.
    places: Places,
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
    /// The launcher's folder, when it runs portable.
    portable: Option<PathBuf>,
    /// Where the build kit is: what an antivirus should leave alone.
    kit_dir: PathBuf,
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
    /// How the library lists the projects: `projects` come in that order.
    sort: Sort,
    projects: Vec<ProjectView>,
    /// Projects on the way, shown greyed out after the others.
    coming_soon: Vec<TeaserView>,
    /// The author's mailbox is known: ideas and messages can be sent.
    feedback_inbox: bool,
    /// Messages kept until the mailbox answers again.
    feedback_outbox: usize,
}

#[derive(Serialize)]
struct TeaserView {
    id: String,
    name: String,
    tagline: String,
    description: String,
    tags: Vec<String>,
    when: Option<String>,
    /// Pictures in the Hub, as GitHub URLs.
    hero: Option<String>,
    logo: Option<String>,
    hub_icon: Option<String>,
    screens: Vec<String>,
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
struct PlatformView {
    os: String,
    guide: Option<String>,
    soon: bool,
}

#[derive(Serialize)]
struct CreditView {
    github: String,
    role: String,
    os: Option<String>,
    url: String,
}

#[derive(Serialize)]
struct ProjectView {
    id: String,
    /// The name the user gave it, or the registry's.
    name: String,
    /// The registry's name: what a rename falls back to, and what reports
    /// on GitHub call it.
    default_name: String,
    /// Its name shown as lettering instead of its logo, by the user's choice.
    text_title: bool,
    tagline: String,
    description: String,
    tags: Vec<String>,
    url: String,
    /// Pictures from the project's repository, as GitHub URLs.
    hero: Option<String>,
    logo: Option<String>,
    screens: Vec<String>,
    /// The user's own pictures, when they picked one and its file is there:
    /// shown instead of `hero`. The window loads them from the `art:` protocol.
    cover: Option<Art>,
    background: Option<Art>,
    /// The user's own logo, shown instead of `logo` (the registry's).
    logo_art: Option<Art>,
    /// The registry's icon (a URL), and the user's own, shown instead.
    hub_icon: Option<String>,
    icon: Option<Art>,
    input_label: Option<String>,
    /// Where it runs (Windows when the registry does not say), with each
    /// guide as a GitHub page.
    platforms: Vec<PlatformView>,
    /// Its contributors, for the Credits page.
    credits: Vec<CreditView>,
    dir: Option<PathBuf>,
    input: Option<PathBuf>,
    state: State,
    can_update: bool,
    /// The last update's new source is here but the game was not rebuilt
    /// from it (cancelled, or the build failed): Play starts the old version.
    rebuild_pending: bool,
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
    /// Kept out of the library, the sidebar and Home by the user.
    hidden: bool,
    /// When it was last played (Unix seconds): started from the launcher, or
    /// its achievements written by the game, whichever is later.
    last_played: Option<u64>,
}

#[derive(Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum TaskEvent {
    Line { project: String, text: String },
    /// `diagnosis`: what went wrong, when it is a problem the launcher knows.
    Done { project: String, ok: bool, message: String, diagnosis: Option<Diagnosis> },
}

fn snapshot_of(hub: &Hub) -> Snapshot {
    let mut config = hub.config.lock().unwrap().clone();
    // An install folder that is no longer on the disk (deleted, or on a drive
    // that is not plugged in) is shown as not chosen, and named, so the user
    // picks one again instead of finding every project "not installed"
    let missing_root = config.install_root().filter(|root| !root.is_dir());
    if missing_root.is_some() {
        config.install_root = None;
    }
    let registry = hub.registry.lock().unwrap().clone();
    let remote = hub.remote_news.lock().unwrap();
    let art_dir = config.art_dir();
    let own_art = |id: &str, kind| config.art(id, kind).filter(|a| art_dir.join(&a.file).is_file()).cloned();
    let mut projects: Vec<ProjectView> = registry
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
                name: config.name(&p.id).unwrap_or(&p.name).to_string(),
                default_name: p.name.clone(),
                text_title: config.text_title(&p.id),
                tagline: p.tagline.clone(),
                description: p.description.clone(),
                tags: p.tags.clone(),
                url: p.url(),
                hero: p.art.hero.as_deref().map(|f| p.raw_url(f)),
                logo: p.logo_url(),
                screens: p.art.screens.iter().map(|f| p.raw_url(f)).collect(),
                cover: own_art(&p.id, art::Kind::Cover),
                background: own_art(&p.id, art::Kind::Background),
                logo_art: own_art(&p.id, art::Kind::Logo),
                hub_icon: p.icon_url(),
                icon: own_art(&p.id, art::Kind::Icon),
                input_label: p.input.as_ref().map(|i| i.label.clone()),
                platforms: if p.platforms.is_empty() {
                    vec![PlatformView { os: "windows".into(), guide: None, soon: false }]
                } else {
                    p.platforms.iter().map(|x| PlatformView { os: x.os.clone(), guide: x.guide.as_deref().map(|g| p.page_url(g)), soon: x.soon }).collect()
                },
                credits: p.credits.iter().map(|c| CreditView { github: c.github.clone(), role: c.role.clone(), os: c.os.clone(), url: c.profile_url() }).collect(),
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
                rebuild_pending: state == State::Ready && dir.as_deref().is_some_and(|d| update::rebuild_pending(p, d)),
                can_rebuild: state == State::Ready && p.scripts.build.is_some(),
                is_clone: dir.as_ref().is_some_and(|d| d.join(".git").exists()),
                hidden: config.hidden(&p.id),
                last_played: match (&dir, installed) {
                    (Some(d), true) => config.last_played(&p.id).max(achievements::written(d)),
                    _ => config.last_played(&p.id),
                },
                dir,
                input,
                state,
            }
        })
        .collect();
    let order = config.arrange(&projects.iter().map(|p| (p.id.as_str(), p.name.as_str(), p.last_played)).collect::<Vec<_>>());
    let mut slots: Vec<Option<ProjectView>> = projects.drain(..).map(Some).collect();
    projects = order.into_iter().filter_map(|i| slots[i].take()).collect();
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
    // Each lock once: a second lock of the same mutex in this expression would
    // never be granted, and the window would wait for its snapshot forever
    let launcher_news = hub.launcher_news.lock().unwrap().clone();
    Snapshot {
        version: env!("CARGO_PKG_VERSION"),
        portable: hub.places.portable.clone(),
        kit_dir: hub.places.kit.clone(),
        install_root: config.install_root(),
        missing_root,
        language: config.language.clone(),
        license_accepted: config.license_accepted.is_some_and(|v| v >= LICENSE_VERSION),
        config_error: hub.config_error.lock().unwrap().clone(),
        busy: hub.task.lock().unwrap().as_ref().map(|t| t.project.clone()),
        launcher_update: launcher_update(&config, &launcher_news),
        launcher_news,
        latest_video: hub.latest_video.lock().unwrap().clone(),
        announcements,
        sort: config.sort,
        projects,
        coming_soon: registry
            .teasers()
            .map(|t| TeaserView {
                id: t.id.clone(),
                name: t.name.clone(),
                tagline: t.tagline.clone(),
                description: t.description.clone(),
                tags: t.tags.clone(),
                when: t.when.clone().filter(|w| !w.trim().is_empty()),
                hero: t.hero_url(),
                logo: t.logo_url(),
                hub_icon: t.icon_url(),
                screens: t.screen_urls(),
            })
            .collect(),
        feedback_inbox: registry.inbox().is_some(),
        feedback_outbox: feedback::waiting(&feedback::file_next_to(&hub.places.config)),
    }
}

/// The last launcher version that did not record which one the user had seen:
/// a user with no version recorded who accepted the license came from it or
/// from an older one (0.3.6 never got to record it).
const BEFORE_RECORD: &str = "0.3.5";

/// What a launcher update brought, while the user has not seen it.
fn launcher_update(config: &Config, news: &[Entry]) -> Option<LauncherNews> {
    let current = env!("CARGO_PKG_VERSION");
    match config.last_version.as_deref() {
        Some(v) if v == current => None,
        // Read before updating: the user chose to update to that version
        // after reading what it brings
        Some(v) if news::is_newer(v, current) => None,
        None if config.license_accepted.is_none() => None,
        from => {
            let from = from.unwrap_or(BEFORE_RECORD);
            Some(LauncherNews { from: Some(from.to_string()), changes: news::launcher_changes(news, Some(from), current) })
        }
    }
}

fn change_config(hub: &Hub, change: impl FnOnce(&mut Config)) -> Result<(), String> {
    if let Some(e) = hub.config_error.lock().unwrap().as_ref() {
        return Err(format!("{e} Fix or delete {} and start the launcher again.", hub.places.config.display()));
    }
    let mut config = hub.config.lock().unwrap();
    let mut next = config.clone();
    change(&mut next);
    next.save(&hub.places.config)?;
    *config = next;
    Ok(())
}

/// The project as the registry lists it, under the name the user gave it:
/// what its messages and its desktop shortcut call it.
fn find_project(hub: &Hub, id: &str) -> Result<Project, String> {
    let mut p = hub.registry.lock().unwrap().find(id).cloned().ok_or_else(|| format!("Unknown project: {id}"))?;
    if let Some(name) = hub.config.lock().unwrap().name(id) {
        p.name = name.to_string();
    }
    Ok(p)
}

/// An idea, recomp proposal or message, as the Feedback page's form has it.
#[derive(Deserialize)]
struct FeedbackForm {
    kind: String,
    about: Option<String>,
    title: String,
    system: Option<String>,
    text: String,
    contact: Option<String>,
    details: Vec<(String, String)>,
}

#[derive(Serialize)]
struct FeedbackReply {
    /// sent, queued, limited or error.
    status: &'static str,
    error: Option<String>,
    outbox: usize,
}

/// Sends a message to the author's mailbox, or keeps it for later when the
/// mailbox does not answer. Network, so off the main thread.
#[tauri::command(async)]
fn send_feedback(hub: tauri::State<Hub>, message: FeedbackForm) -> FeedbackReply {
    let inbox = hub.registry.lock().unwrap().inbox().map(str::to_string);
    let lang = hub.config.lock().unwrap().language.clone();
    let path = feedback::file_next_to(&hub.places.config);
    let reply = |status, error| FeedbackReply { status, error, outbox: feedback::waiting(&path) };
    let Some(inbox) = inbox else {
        return reply("error", Some("The mailbox is not available right now.".into()));
    };
    let m = feedback::Message {
        kind: message.kind,
        about: message.about,
        title: message.title,
        system: message.system,
        text: message.text,
        contact: message.contact,
        details: message.details,
        lang,
        written: 0,
    };
    match feedback::send(&inbox, &path, m) {
        Ok(feedback::Outcome::Sent) => reply("sent", None),
        Ok(feedback::Outcome::Queued) => reply("queued", None),
        Ok(feedback::Outcome::Limited) => reply("limited", None),
        Err(e) => reply("error", Some(e)),
    }
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
    // Messages for the author that waited for his mailbox: on their own
    // thread, so a mailbox that does not answer never slows the refresh
    let path = feedback::file_next_to(&hub.places.config);
    if let Some(inbox) = hub.registry.lock().unwrap().inbox().map(str::to_string) {
        if feedback::waiting(&path) > 0 {
            std::thread::spawn(move || feedback::flush(&inbox, &path));
        }
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
    /// What it brings since the running version, from the launcher's news,
    /// shown before the user chooses to update. Empty while the news are not
    /// loaded or say nothing of it.
    changes: Vec<news::Release>,
}

/// The installer for an installed launcher; the portable zip, signed with the
/// same key, for a portable one (the installer would install a second copy in
/// AppData instead of updating this one).
fn updater(app: &AppHandle) -> Result<tauri_plugin_updater::Updater, String> {
    let builder = app.updater_builder();
    let builder = match app.state::<Hub>().places.portable {
        Some(_) => builder.target(portable::UPDATE_TARGET),
        None => builder,
    };
    builder.build().map_err(|e| e.to_string())
}

/// Asks the launcher's releases on GitHub whether there is a newer version.
/// Only the launcher itself: projects are never downloaded built.
#[tauri::command]
async fn check_launcher_update(app: AppHandle) -> Result<Option<LauncherUpdate>, String> {
    let update = updater(&app)?.check().await.map_err(|e| e.to_string())?;
    let news = app.state::<Hub>().launcher_news.lock().unwrap().clone();
    let current = env!("CARGO_PKG_VERSION");
    Ok(update.map(|u| LauncherUpdate {
        changes: news::launcher_changes(&news, Some(current), &u.version),
        version: u.version.clone(),
        notes: u.body.clone(),
    }))
}

/// Downloads the new version, checks its signature against the key built into
/// this exe, installs it and starts it. Refused while a task is running.
#[tauri::command]
async fn install_launcher_update(app: AppHandle) -> Result<(), String> {
    if app.state::<Hub>().task.lock().unwrap().is_some() {
        return Err("A task is still running: wait for it to end first.".into());
    }
    let update = updater(&app)?.check().await.map_err(|e| e.to_string())?;
    let update = update.ok_or("This is already the latest version.")?;
    if app.state::<Hub>().places.portable.is_some() {
        // Checked against the signature before it comes back
        let zip = update.download(|_, _| {}, || {}).await.map_err(|e| e.to_string())?;
        let exe = portable::exe_from_zip(&zip)?;
        let current = std::env::current_exe().map_err(|e| e.to_string())?;
        portable::replace_exe(&current, &exe)?;
        // Started from the path it was read from: the running process now
        // sits at the .old name
        std::process::Command::new(&current).spawn().map_err(|e| format!("Could not start the new version: {e}"))?;
        app.exit(0);
        return Ok(());
    } else {
        update.download_and_install(|_, _| {}, || {}).await.map_err(|e| e.to_string())?;
    }
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
    change_config(&hub, |c| c.install_root = Some(c.store(path)))?;
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
/// With `version`, a newer one: its news were read before updating to it, so
/// they are not shown again once it starts.
#[tauri::command]
fn mark_launcher_seen(hub: tauri::State<Hub>, version: Option<String>) -> Result<Snapshot, String> {
    let current = env!("CARGO_PKG_VERSION");
    let seen = version.filter(|v| news::is_newer(v, current)).unwrap_or_else(|| current.to_string());
    change_config(&hub, |c| c.last_version = Some(seen))?;
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
    change_config(&hub, |c| {
        let path = c.store(path);
        c.projects.entry(id).or_default().input = Some(path);
    })?;
    Ok(snapshot_of(&hub))
}

/// The user's own name for a project. Empty, or the registry's name, goes
/// back to the registry's. Only the launcher shows it: the project's folder
/// and files keep their names.
#[tauri::command]
fn set_project_name(hub: tauri::State<Hub>, id: String, name: String) -> Result<Snapshot, String> {
    let default = hub.registry.lock().unwrap().find(&id).map(|p| p.name.clone()).ok_or_else(|| format!("Unknown project: {id}"))?;
    let name = config::clean_name(&name, &default);
    change_config(&hub, |c| match name {
        Some(name) => c.projects.entry(id).or_default().name = Some(name),
        None => {
            if let Some(pc) = c.projects.get_mut(&id) {
                pc.name = None;
            }
        }
    })?;
    Ok(snapshot_of(&hub))
}

/// The project's name as lettering instead of its logo, or its logo again.
#[tauri::command]
fn set_project_text_title(hub: tauri::State<Hub>, id: String, on: bool) -> Result<Snapshot, String> {
    find_project(&hub, &id)?;
    change_config(&hub, |c| c.projects.entry(id).or_default().text_title = on)?;
    Ok(snapshot_of(&hub))
}

/// Hides a project from the library, the sidebar and Home, or shows it again.
#[tauri::command]
fn set_project_hidden(hub: tauri::State<Hub>, id: String, hidden: bool) -> Result<Snapshot, String> {
    find_project(&hub, &id)?;
    change_config(&hub, |c| c.projects.entry(id).or_default().hidden = hidden)?;
    Ok(snapshot_of(&hub))
}

/// How the library lists the projects: "custom", "name" or "recent".
#[tauri::command]
fn set_library_sort(hub: tauri::State<Hub>, sort: String) -> Result<Snapshot, String> {
    let sort = Sort::parse(&sort).ok_or_else(|| format!("Unknown order: {sort}"))?;
    change_config(&hub, |c| c.sort = sort)?;
    Ok(snapshot_of(&hub))
}

/// The user's own order of the projects, by id, and that order in use.
#[tauri::command]
fn set_library_order(hub: tauri::State<Hub>, ids: Vec<String>) -> Result<Snapshot, String> {
    change_config(&hub, |c| {
        c.order = ids;
        c.sort = Sort::Custom;
    })?;
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
    change_config(&hub, |c| {
        let path = c.store(path);
        c.projects.entry(id).or_default().dir = Some(path);
    })?;
    Ok(snapshot_of(&hub))
}

/// Where the user's pictures are kept: next to the settings.
fn art_dir(hub: &Hub) -> PathBuf {
    art::dir(hub.places.config.parent().unwrap_or(Path::new(".")))
}

/// A picture for a project's cover, background, logo or icon, copied into the
/// launcher's pictures folder: a file the user picked, or one of the project's
/// screenshots (from its folder when it is installed, from GitHub otherwise).
/// Returned as the settings would keep it, centred. Only made ready: the
/// settings change when the user saves. A big picture
/// takes a moment, so it runs off the main thread.
#[tauri::command(async)]
fn prepare_art(hub: tauri::State<Hub>, id: String, kind: art::Kind, path: Option<PathBuf>, screen: Option<usize>) -> Result<Art, String> {
    let p = find_project(&hub, &id)?;
    let dir = art_dir(&hub);
    let file = match (path, screen) {
        (Some(path), _) => art::from_file(&dir, &id, kind, &path),
        // Screenshots are not logos
        (None, Some(_)) if kind == art::Kind::Logo => Err("Choose a logo from your PC.".into()),
        (None, Some(i)) => {
            let rel = p.art.screens.get(i).ok_or("That screenshot is not in the list any more.")?;
            let installed = hub.config.lock().unwrap().project_dir(&id).map(|d| d.join(rel)).filter(|f| f.is_file());
            match installed {
                Some(local) => art::from_file(&dir, &id, kind, &local),
                None => art::from_url(&dir, &id, kind, &p.raw_url(rel)),
            }
        }
        (None, None) => Err("No picture was chosen.".into()),
    }?;
    // An icon with see-through parts is shown as it is
    let clear = kind == art::Kind::Icon && art::see_through(&dir, &file);
    Ok(Art { file, focus: [50, 50], clear })
}

/// Saves the user's pictures for a project; None puts the registry's back.
/// The pictures that were made ready and not saved, or that these replace,
/// are deleted. A new icon gets its .ico made, and the shortcut on the
/// desktop, if there is one, gets the new icon (or the game's own back).
#[tauri::command(async)]
fn set_project_art(hub: tauri::State<Hub>, id: String, cover: Option<Art>, background: Option<Art>, logo: Option<Art>, icon: Option<Art>) -> Result<Snapshot, String> {
    let p = find_project(&hub, &id)?;
    let dir = art_dir(&hub);
    for a in [&cover, &background, &logo, &icon].into_iter().flatten() {
        if !art::valid_name(&a.file) || !dir.join(&a.file).is_file() {
            return Err("That picture is not there any more. Choose it again.".into());
        }
    }
    // Whether it is see-through is the file's to say
    let icon = icon.map(|a| Art { clear: art::see_through(&dir, &a.file), ..a.clean() });
    let ico = icon.as_ref().map(|a| art::write_ico(&dir, a)).transpose()?;
    let icon_changed = hub.config.lock().unwrap().art(&id, art::Kind::Icon) != icon.as_ref();
    change_config(&hub, |c| {
        let pc = c.projects.entry(id.clone()).or_default();
        pc.cover = cover.map(Art::clean);
        pc.background = background.map(Art::clean);
        // A new logo is put up to be seen, not left behind the name
        if logo.is_some() && logo.as_ref().map(|a| &a.file) != pc.logo.as_ref().map(|a| &a.file) {
            pc.text_title = false;
        }
        pc.logo = logo.map(Art::clean);
        pc.icon = icon;
    })?;
    prune_art(&hub);
    // The icon is saved either way: a shortcut that cannot be changed now
    // (the desktop on a drive that is not there) is no reason to say it is not
    if icon_changed {
        let game = hub.config.lock().unwrap().project_dir(&id);
        if let Some(game) = game {
            let _ = project::refresh_shortcut(&p, &game, ico.as_deref());
        }
    }
    Ok(snapshot_of(&hub))
}

/// The user closed the pictures without saving: what was made ready goes.
#[tauri::command]
fn discard_art(hub: tauri::State<Hub>) {
    prune_art(&hub);
}

/// Deletes the pictures the settings do not name.
fn prune_art(hub: &Hub) {
    // Settings that could not be read say nothing of which pictures are in use
    if hub.config_error.lock().unwrap().is_some() {
        return;
    }
    let files = hub.config.lock().unwrap().art_files();
    art::prune(&art_dir(hub), &files.iter().map(String::as_str).collect::<Vec<_>>());
}

/// The `art:` protocol: the window asks for a picture of the pictures folder
/// by its name, and gets nothing for any other name.
fn art_response(dir: &Path, name: &str) -> tauri::http::Response<Vec<u8>> {
    let builder = tauri::http::Response::builder();
    match art::read(dir, name) {
        // A name is never given to another picture: the window may keep it
        Some(bytes) => builder.header("Content-Type", if name.ends_with(".png") { "image/png" } else if name.ends_with(".ico") { "image/x-icon" } else { "image/jpeg" }).header("Cache-Control", "max-age=31536000, immutable").body(bytes),
        None => builder.status(404).body(Vec::new()),
    }
    .expect("a response with fixed headers")
}

#[tauri::command]
fn launch(hub: tauri::State<Hub>, id: String) -> Result<(), String> {
    let p = find_project(&hub, &id)?;
    let dir = hub.config.lock().unwrap().project_dir(&id).ok_or("Not installed.")?;
    project::launch(&p, &dir)?;
    // For "Recently played". The game is running: a settings file that cannot
    // be written now is no reason to say it did not start
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let _ = change_config(&hub, |c| c.projects.entry(id).or_default().last_played = Some(now));
    Ok(())
}

/// A shortcut to the built game on the desktop.
#[tauri::command]
fn desktop_shortcut(hub: tauri::State<Hub>, id: String) -> Result<(), String> {
    let p = find_project(&hub, &id)?;
    let (dir, icon) = {
        let config = hub.config.lock().unwrap();
        (config.project_dir(&id).ok_or("Not installed.")?, config.art(&id, art::Kind::Icon).cloned())
    };
    // The user's icon, its .ico made again if it is gone
    let art_dir = art_dir(&hub);
    let ico = icon.and_then(|a| Some(art_dir.join(art::ico_name(&a))).filter(|f| f.is_file()).or_else(|| art::write_ico(&art_dir, &a).ok()));
    project::desktop_shortcut(&p, &dir, ico.as_deref())
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
            let file_gone = !c.input(&id).is_some_and(|file| file.is_file());
            if let Some(pc) = c.projects.get_mut(&id) {
                pc.dir = None;
                if file_gone {
                    pc.input = None;
                }
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
/// `task` events. `fix`: the button of a failed task's diagnosis, done first.
#[tauri::command]
fn start_task(app: AppHandle, hub: tauri::State<Hub>, id: String, action: String, fix: Option<Fix>) -> Result<(), String> {
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
    let kit_root = hub.places.kit.clone();
    let cancel = Cancel::new();
    {
        let mut task = hub.task.lock().unwrap();
        if task.is_some() {
            return Err("Another task is still running.".into());
        }
        *task = Some(Running { project: id.clone(), cancel: cancel.clone() });
    }
    std::thread::spawn(move || {
        // The end of the log, for the diagnosis if it fails
        let mut lines: std::collections::VecDeque<String> = std::collections::VecDeque::new();
        let mut log = |text: &str| {
            lines.extend(text.lines().map(String::from));
            while lines.len() > 1000 {
                lines.pop_front();
            }
            let _ = app.emit("task", TaskEvent::Line { project: id.clone(), text: text.to_string() });
        };
        let result = match &fix {
            Some(f) => f.apply(&kit_root, &dir, &mut log),
            None => Ok(()),
        }
        .and_then(|()| run_task(&p, &dir, input.as_deref(), &kit_root, &action, &cancel, &mut log));
        let (ok, message) = match result {
            _ if cancel.was_asked() => (false, "Cancelled.".to_string()),
            Ok(()) => (true, String::new()),
            Err(e) => (false, e),
        };
        let diagnosis = if ok || cancel.was_asked() { None } else { diagnose::diagnose(lines.make_contiguous(), &message) };
        *app.state::<Hub>().task.lock().unwrap() = None;
        let _ = app.emit("task", TaskEvent::Done { project: id.clone(), ok, message, diagnosis });
    });
    Ok(())
}

fn run_task(
    p: &Project,
    dir: &std::path::Path,
    input: Option<&std::path::Path>,
    kit_root: &std::path::Path,
    action: &str,
    cancel: &Cancel,
    log: &mut dyn FnMut(&str),
) -> Result<(), String> {
    let script = match action {
        "setup" => {
            if !project::is_installed(p, dir) {
                source::install(p, dir, cancel, log)?;
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
            let outcome = update::update(p, dir, false, cancel, log)?;
            if cancel.was_asked() {
                return Ok(());
            }
            if matches!(outcome, Outcome::UpToDate { .. }) && project::is_built(p, dir) {
                // The source is the latest, but an earlier update's rebuild
                // never finished: the program is still the old version
                if !update::rebuild_pending(p, dir) {
                    return Ok(());
                }
                log("The last update was not finished: the game is still the previous version.");
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
            if let Some(problem) = portable::kit_problem(kit_root) {
                return Err(problem);
            }
            build::install_kit(&manifest, kit_root, cancel, log)?;
            match build::Tools::find(&manifest, input, kit_root) {
                Ok(tools) => {
                    let warnings = build::run(&manifest, dir, &tools, cancel, log)?;
                    for w in &warnings {
                        log(&format!("Warning: {w}"));
                    }
                    log(&format!("Done: {}", manifest.exe));
                    built(p, dir, log);
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
    if script != Script::Update && project::is_built(p, dir) {
        built(p, dir, log);
    }
    Ok(())
}

/// A build worked: the program is now the installed version, and an update
/// that was waiting for it is finished.
fn built(p: &Project, dir: &std::path::Path, log: &mut dyn FnMut(&str)) {
    if let Err(e) = update::record_built(p, dir) {
        log(&format!("Warning: {e}"));
    }
}

fn main() {
    let places = Places::here();
    let (mut config, config_error) = match Config::load(&places.config) {
        Ok(c) => (c, None),
        Err(e) => (Config::default(), Some(e)),
    };
    config.base = places.config.parent().map(PathBuf::from).unwrap_or_default();
    config.portable = places.portable.is_some();
    let mut context = tauri::generate_context!();
    // The launcher opens its windows itself. Tauri does not pass a window's
    // `data_directory` from the config to its webview, which then keeps its
    // data in %LOCALAPPDATA%: a portable launcher sets the folder on each.
    // And a link that asks for a new window (YouTube's own, in the video
    // player) opens in the user's browser instead of a bare window.
    let webview_dir = places.webview.clone();
    for window in &mut context.config_mut().app.windows {
        window.create = false;
    }
    // What a portable update left aside, once the old launcher has closed
    if let Ok(exe) = std::env::current_exe() {
        std::thread::spawn(move || portable::remove_old_exe(&exe));
    }
    tauri::Builder::default()
        .setup(move |app| {
            for window in app.config().app.windows.clone() {
                let handle = app.handle().clone();
                let mut builder = WebviewWindowBuilder::from_config(app.handle(), &window)?
                    .on_new_window(move |url, _| {
                        if matches!(url.scheme(), "https" | "http") {
                            let _ = handle.opener().open_url(url.as_str(), None::<&str>);
                        }
                        NewWindowResponse::Deny
                    });
                if let Some(dir) = &webview_dir {
                    builder = builder.data_directory(dir.clone());
                }
                builder.build()?;
            }
            // Pictures left from a launcher closed while one was picked
            let handle = app.handle().clone();
            std::thread::spawn(move || prune_art(&handle.state::<Hub>()));
            Ok(())
        })
        .register_asynchronous_uri_scheme_protocol("art", |ctx, request, responder| {
            let dir = art_dir(&ctx.app_handle().state::<Hub>());
            let name = request.uri().path().trim_start_matches('/').to_string();
            std::thread::spawn(move || responder.respond(art_response(&dir, &name)));
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(Hub {
            places,
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
            set_project_name,
            set_project_text_title,
            set_project_hidden,
            set_library_sort,
            set_library_order,
            prepare_art,
            set_project_art,
            discard_art,
            launch,
            open_folder,
            system_info,
            send_feedback,
            desktop_shortcut,
            uninstall,
            cancel_task,
            start_task
        ])
        .run(context)
        .expect("could not start NicoLauncher");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every command answers with a snapshot: one that never comes back leaves
    // the window empty. It takes each lock once.
    #[test]
    fn a_snapshot_comes_back() {
        let hub = Hub {
            places: Places::for_exe_dir(None),
            config: Mutex::new(Config { license_accepted: Some(LICENSE_VERSION), ..Config::default() }),
            config_error: Mutex::new(None),
            registry: Mutex::new(Registry::bundled()),
            remote_news: Mutex::new(HashMap::new()),
            launcher_news: Mutex::new(news::parse("[1] 2026-11-14\n- Launcher 0.3.6: something\n")),
            latest_video: Mutex::new(None),
            announcements: Mutex::new(Vec::new()),
            task: Mutex::new(None),
        };
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let snap = snapshot_of(&hub);
            tx.send((snap.launcher_update.is_some(), snap.coming_soon.iter().map(|t| t.id.clone()).collect::<Vec<_>>())).unwrap();
        });
        let (update, soon) = rx.recv_timeout(std::time::Duration::from_secs(10)).expect("the snapshot never came back");
        assert!(update);
        assert_eq!(soon.len(), Registry::bundled().teasers().count());
    }

    // Read before updating: nothing to show again after the restart, and
    // nothing while it is still downloading
    #[test]
    fn news_read_before_updating_are_not_shown_again() {
        let news = news::parse("[1] 2026-11-14\n- Launcher 99.0.0: something\n");
        let with = |v: &str| Config { license_accepted: Some(LICENSE_VERSION), last_version: Some(v.into()), ..Config::default() };
        assert!(launcher_update(&with("99.0.0"), &news).is_none());
        assert!(launcher_update(&with(env!("CARGO_PKG_VERSION")), &news).is_none());
        assert!(launcher_update(&with("0.0.1"), &news).is_some());
    }
}
