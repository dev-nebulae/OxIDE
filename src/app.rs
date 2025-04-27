use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

mod styles;

use egui::{self, Context, Window};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // If we add new fields, give them default values when deserializing old state
pub struct App {
    label: String,
    value: f32,

    #[serde(skip)]
    show_window: bool,

    #[serde(skip)]
    project_name: String,

    #[serde(skip)]
    project_type: ProjectType,

    #[serde(skip)]
    target_directory: Option<PathBuf>,
}

#[derive(PartialEq)]
enum ProjectType {
    None,
    Library,
    Binary,
}

impl Default for App {
    fn default() -> Self {
        Self {
            label: "".to_owned(),
            value: 2.7,
            show_window: false,
            project_name: String::new(),
            project_type: ProjectType::None,
            target_directory: None,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        style_app(ctx);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.menu_button("Edit", |ui| {
                        if ui.button("Undo").clicked() {
                            println!("Undo");
                        }
                        if ui.button("Redo").clicked() {
                            println!("Redo");
                        }
                        ui.separator();
                        if ui.button("Cut").clicked() {
                            println!("Cut");
                        }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Open a project to get started");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("OxIDE - The Rust IDE");
            ui.label("What do you want to do today?");
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Open file").clicked() {
                    let file = rfd::FileDialog::new()
                        .add_filter("Rust source file", &["rs"])
                        .set_title("Open file")
                        .pick_file();
                    if let Some(path) = file {
                        let mut file = File::open(path).unwrap();
                        let mut conts = String::new();
                        file.read_to_string(&mut conts).unwrap();
                        println!("{}", conts);
                    } else {
                        println!("No file was selected.");
                    }
                }

                if ui.button("Open project").clicked() {
                    let fol = rfd::FileDialog::new()
                        .set_title("Select folder")
                        .pick_folder();
                    if let Some(path) = fol {
                        println!("Folder selected: {:?}", path);
                        self.show_window = false; // Not opening the project creation window here
                    } else {
                        println!("No folder selected.");
                    }
                }

                if ui.button("New project").clicked() {
                    let fol = rfd::FileDialog::new()
                        .set_title("Select a folder to create your project")
                        .pick_folder();
                    if let Some(path) = fol {
                        self.target_directory = Some(path);
                        self.show_window = true;
                    } else {
                        println!("No folder selected.");
                    }
                }
            });
        });

        self.show_project_creator(ctx);
    }
}

impl App {
 
    fn show_project_creator(&mut self, ctx: &Context) {
        if self.show_window {
            let mut open = true; // Temporary variable!

            Window::new("Create New Rust Project")
                .open(&mut open) // use temp variable
                .show(ctx, |ui| {
                    ui.label("Enter project name:");
                    ui.text_edit_singleline(&mut self.project_name);

                    ui.label("Choose project type:");
                    ui.horizontal(|ui| {
                        if ui.button("Library").clicked() {
                            self.project_type = ProjectType::Library;
                        }
                        if ui.button("Binary").clicked() {
                            self.project_type = ProjectType::Binary;
                        }
                    });

                    if !self.project_name.is_empty() && self.project_type != ProjectType::None {
                        if ui.button("Create Project!").clicked() {
                            self.create_project();
                            // self.show_window = false; <-- NO need here, we will set it after
                        }
                    }
                });

            self.show_window = open; // update the real self.show_window AFTER show()
    }
}


    fn create_project(&self) {
        if let Some(ref target_dir) = self.target_directory {
            let mut cmd = Command::new("cargo");
            cmd.arg("new");

            if self.project_type == ProjectType::Library {
                cmd.arg("--lib");
            }

            let full_path = target_dir.join(&self.project_name);
            cmd.arg(full_path);

            match cmd.status() {
                Ok(status) if status.success() => println!("Project created successfully!"),
                Ok(status) => println!("Command exited with status: {:?}", status),
                Err(err) => println!("Failed to execute command: {}", err),
            }
        } else {
            println!("No target directory selected!");
        }
    }
}

