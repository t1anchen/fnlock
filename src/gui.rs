use std::ops::Deref;

use eframe::egui;

use crate::Opts;
pub struct FnlockGuiApp {
  cmd_opts: Opts,
  unlock_state: String,
}

impl FnlockGuiApp {
  fn default_with_opts(cmd_opts: Opts) -> Self {
    Self {
      cmd_opts,
      unlock_state: "".to_owned(),
    }
  }
}

impl eframe::App for FnlockGuiApp {
  fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        // File menu
        ui.menu_button("File", |ui| {
          if ui.button("Quit").clicked() {
            std::process::exit(0)
          }
        });
        ui.menu_button("Help", |ui| if ui.button("About").clicked() {})
      });
      egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::Grid::new("dashboard")
          .num_columns(3)
          .spacing([40.0, 2.0])
          .striped(true)
          .show(ui, |ui| {
            ui.add(egui::Label::new("keyboard1"));
            ui.add(egui::Label::new(&self.unlock_state));
            ui.horizontal_centered(|ui| {
              let button_lock = egui::Button::new("Lock");
              let button_unlock = egui::Button::new("Unlock");
              ui.add_enabled(false, button_lock);
              if ui.add(button_unlock).clicked() {
                self.unlock_state = String::from("Locked");
              }
            });
            ui.end_row();
            ui.add(egui::Label::new("keyboard2"));
            if ui.button("Lock").clicked() {}
            if ui.button("Unlock").clicked() {}
            ui.end_row();
          })
      });
    });
  }
}

pub fn gui_main(opts: Opts) {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
    ..Default::default()
  };
  let _ = eframe::run_native(
    "Fnlock",
    options,
    Box::new(|cc| Ok(Box::new(FnlockGuiApp::default_with_opts(opts)))),
  );
  ()
}
