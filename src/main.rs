use eframe::egui;
use regex::Regex;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

#[derive(Clone, Debug)]
enum CameraStatus {
    Unknown,
    CameraOn,
    CameraOff,
}

#[derive(Clone)]
struct AppState {
    camera_status: Arc<Mutex<CameraStatus>>,
    is_paused: Arc<Mutex<bool>>,
    last_event: Arc<Mutex<String>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            camera_status: Arc::new(Mutex::new(CameraStatus::Unknown)),
            is_paused: Arc::new(Mutex::new(false)),
            last_event: Arc::new(Mutex::new(String::from("Waiting for camera events..."))),
        }
    }
}

struct MeetingAutoPauseApp {
    state: AppState,
}

impl MeetingAutoPauseApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let state = AppState::new();

        // Spawn the camera monitor
        let state_clone = state.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(monitor_camera(state_clone));
        });

        Self { state }
    }
}

impl eframe::App for MeetingAutoPauseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mac Meeting Auto-Pause");
            ui.add_space(20.0);

            let camera_status = self.state.camera_status.lock().unwrap();
            let is_paused = self.state.is_paused.lock().unwrap();
            let last_event = self.state.last_event.lock().unwrap();

            ui.horizontal(|ui| {
                ui.label("Camera Status:");
                let (status_text, color) = match *camera_status {
                    CameraStatus::CameraOn => ("ON (In Meeting)", egui::Color32::GREEN),
                    CameraStatus::CameraOff => ("OFF", egui::Color32::RED),
                    CameraStatus::Unknown => ("Unknown", egui::Color32::GRAY),
                };
                ui.colored_label(color, status_text);
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Music Status:");
                let music_text = if *is_paused { "PAUSED" } else { "PLAYING" };
                ui.colored_label(
                    if *is_paused { egui::Color32::YELLOW } else { egui::Color32::GREEN },
                    music_text
                );
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            ui.label("Last Event:");
            ui.label(last_event.as_str());

            ui.add_space(20.0);
            ui.label("Monitoring camera with: log stream --predicate 'eventMessage contains \"Cameras changed to\"'");
        });

        // Request repaint to keep UI updating
        ctx.request_repaint();
    }
}

async fn monitor_camera(state: AppState) {
    println!("Starting camera monitor...");

    let mut child = Command::new("log")
        .args([
            "stream",
            "--predicate",
            "eventMessage contains \"Cameras changed to\"",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start log stream");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();

    // Regex to detect if there are active apps using the camera (camera is on)
    let camera_on_regex = Regex::new(r"appEffects: \[ControlCenterApp\.AppVideoEffects").unwrap();

    while let Ok(Some(line)) = lines.next_line().await {
        println!("Log line: {}", line);

        let camera_on = camera_on_regex.is_match(&line);

        let mut status = state.camera_status.lock().unwrap();
        let mut is_paused = state.is_paused.lock().unwrap();
        let mut last_event = state.last_event.lock().unwrap();

        let new_status = if camera_on {
            CameraStatus::CameraOn
        } else {
            CameraStatus::CameraOff
        };

        // Only trigger actions if status changed
        let status_changed = !matches!(
            (&*status, &new_status),
            (CameraStatus::CameraOn, CameraStatus::CameraOn)
                | (CameraStatus::CameraOff, CameraStatus::CameraOff)
        );

        if status_changed {
            *status = new_status.clone();
            *last_event = format!(
                "{:?} detected at {}",
                status,
                chrono::Local::now().format("%H:%M:%S")
            );

            match *status {
                CameraStatus::CameraOn => {
                    println!("Camera turned ON - Pausing music");
                    pause_music();
                    *is_paused = true;
                }
                CameraStatus::CameraOff => {
                    println!("Camera turned OFF - Resuming music");
                    resume_music();
                    *is_paused = false;
                }
                _ => {}
            }
        }
    }
}

fn pause_music() {
    // Using osascript to control Spotify
    std::process::Command::new("osascript")
        .args(["-e", "tell application \"Spotify\" to pause"])
        .output()
        .ok();
}

fn resume_music() {
    // Using osascript to control Spotify
    std::process::Command::new("osascript")
        .args(["-e", "tell application \"Spotify\" to play"])
        .output()
        .ok();
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Mac Meeting Auto-Pause",
        options,
        Box::new(|cc| Ok(Box::new(MeetingAutoPauseApp::new(cc)))),
    )
}
