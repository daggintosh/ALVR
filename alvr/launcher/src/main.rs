mod actions;
mod ui;

use eframe::egui::{IconData, ViewportBuilder};
use ico::IconDir;
use std::{collections::BTreeMap, env, fs, io::Cursor, sync::mpsc, thread};
use ui::Launcher;

pub struct ReleaseChannelsInfo {
    stable: Vec<ReleaseInfo>,
    nightly: Vec<ReleaseInfo>,
}

pub struct Progress {
    message: String,
    progress: f32,
}

pub enum WorkerMessage {
    ReleaseChannelsInfo(ReleaseChannelsInfo),
    ProgressUpdate(Progress),
    Done,
    Error(String),
}

#[derive(Clone)]
pub struct ReleaseInfo {
    version: String,
    assets: BTreeMap<String, String>,
}

pub enum UiMessage {
    InstallServer {
        release_info: ReleaseInfo,
        session_version: Option<String>,
    },
    InstallClient(ReleaseInfo),
    Quit,
}

pub struct InstallationInfo {
    version: String,
    is_apk_downloaded: bool,
    has_session_json: bool, // Only relevant on Windows
}

fn main() {
    let (worker_message_sender, worker_message_receiver) = mpsc::channel::<WorkerMessage>();
    let (ui_message_sender, ui_message_receiver) = mpsc::channel::<UiMessage>();

    let worker_handle =
        thread::spawn(|| actions::worker(ui_message_receiver, worker_message_sender));

    let ico = IconDir::read(Cursor::new(include_bytes!(
        "../../dashboard/resources/dashboard.ico"
    )))
    .unwrap();
    let image = ico.entries().first().unwrap().decode().unwrap();

    // Workaround for the steam deck
    if fs::read_to_string("/sys/devices/virtual/dmi/id/board_vendor")
        .map(|vendor| vendor.trim() == "Valve")
        .unwrap_or(false)
    {
        unsafe { env::set_var("WINIT_X11_SCALE_FACTOR", "1") };
    }

    eframe::run_native(
        "ALVR Launcher",
        eframe::NativeOptions {
            viewport: ViewportBuilder::default()
                .with_app_id("alvr.launcher")
                .with_inner_size((700.0, 400.0))
                .with_icon(IconData {
                    rgba: image.rgba_data().to_owned(),
                    width: image.width(),
                    height: image.height(),
                }),
            ..Default::default()
        },
        Box::new(move |cc| {
            Ok(Box::new(Launcher::new(
                cc,
                worker_message_receiver,
                ui_message_sender,
            )))
        }),
    )
    .expect("Failed to run eframe");

    worker_handle.join().unwrap();
}
